use super::evaluate::matches_query;
use super::index::build_query_items;
use super::parse::parse_query;
use anyhow::Result;
use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) fn values(ir: &CoreIr, expression: &str) -> Result<Vec<Value>> {
    let filters = parse_query(expression)?;
    Ok(build_query_items(ir)
        .into_iter()
        .filter(|item| matches_query(item, &filters))
        .collect())
}

pub(crate) fn item_map(ir: &CoreIr) -> BTreeMap<String, Value> {
    build_query_items(ir)
        .into_iter()
        .filter_map(subject_pair)
        .collect()
}

fn subject_pair(item: Value) -> Option<(String, Value)> {
    let subject = item.get("subject")?.as_str()?.to_string();
    Some((subject, item))
}
