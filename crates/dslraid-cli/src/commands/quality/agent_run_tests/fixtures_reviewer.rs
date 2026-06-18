use serde_json::{json, Value};

pub(super) fn adversarial() -> Value {
    json!([{ "id": "reviewer:red-team", "role": "adversarial-review" }])
}

pub(super) fn with_defaults(value: Value) -> Value {
    let Some(items) = value.as_array() else {
        return value;
    };
    Value::Array(items.iter().map(with_default).collect())
}

fn with_default(item: &Value) -> Value {
    let mut item = item.clone();
    let Some(object) = item.as_object_mut() else {
        return item;
    };
    object.entry("role").or_insert(json!("verification"));
    object.entry("reasoning_level").or_insert(json!("R0"));
    object.entry("trust_tier").or_insert(json!("T2"));
    item
}
