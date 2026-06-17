use serde_json::Value;

pub(super) fn text<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    path.iter()
        .try_fold(value, |current, key| current.get(*key))
        .and_then(Value::as_str)
}

pub(super) fn items<'a>(value: &'a Value, key: &str) -> impl Iterator<Item = &'a Value> {
    value
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
}

pub(super) fn field_is(value: &Value, key: &str, expected: &str) -> bool {
    value.get(key).and_then(Value::as_str) == Some(expected)
}

pub(super) fn field_text<'a>(value: &'a Value, key: &str) -> Option<&'a str> {
    value.get(key).and_then(Value::as_str)
}
