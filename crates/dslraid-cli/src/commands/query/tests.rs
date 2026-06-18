use serde_json::Value;

mod fixtures;
mod item_map;
mod values;

pub(super) fn subject(item: &Value) -> Option<&str> {
    item.get("subject").and_then(Value::as_str)
}
