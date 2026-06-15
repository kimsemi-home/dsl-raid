mod evaluate;
mod index;
mod model;
mod parse;
mod syntax;
mod value;

use crate::OutputFormat;
use anyhow::Result;
use dslraid_core::{load_core_ir, CoreIr};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

#[cfg(test)]
mod tests;

pub(crate) fn run(input: &Path, expression: &str, format: OutputFormat) -> Result<()> {
    let ir = load_core_ir(input)?;
    let items = values(&ir, expression)?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&items)?),
        OutputFormat::Text => print_text_items(items),
    }
    Ok(())
}

pub(crate) fn values(ir: &CoreIr, expression: &str) -> Result<Vec<Value>> {
    let filters = parse::parse_query(expression)?;
    Ok(index::build_query_items(ir)
        .into_iter()
        .filter(|item| evaluate::matches_query(item, &filters))
        .collect())
}

pub(crate) fn item_map(ir: &CoreIr) -> BTreeMap<String, Value> {
    index::build_query_items(ir)
        .into_iter()
        .filter_map(|item| {
            let subject = item
                .get("subject")
                .and_then(Value::as_str)
                .map(str::to_string)?;
            Some((subject, item))
        })
        .collect()
}

fn print_text_items(items: Vec<Value>) {
    for item in items {
        println!(
            "{} {} {}",
            item.get("kind")
                .and_then(Value::as_str)
                .unwrap_or("unknown"),
            item.get("subject")
                .and_then(Value::as_str)
                .unwrap_or("<unknown>"),
            item.get("label").and_then(Value::as_str).unwrap_or("")
        );
    }
}
