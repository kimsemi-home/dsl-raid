use serde_json::Value;

pub(super) fn query_value<'a>(item: &'a Value, key: &str) -> Option<&'a Value> {
    let mut current = item;
    for part in key.split('.') {
        current = current.get(part)?;
    }
    Some(current)
}
