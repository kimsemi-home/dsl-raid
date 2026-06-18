use super::operator::matches_operator;
use crate::commands::query::model::QueryOperator;
use serde_json::Value;

pub(super) fn matches_tag(item: &Value, operator: &QueryOperator) -> bool {
    item.get("tags")
        .and_then(Value::as_array)
        .is_some_and(|tags| tags.iter().any(|tag| matches_operator(Some(tag), operator)))
}
