use serde_json::{json, Value};

pub(super) fn evidence(evidence: &mut Value) -> Value {
    let Some(items) = evidence.as_array_mut() else {
        return json!([]);
    };
    if let Some(id) = items.iter().find_map(|item| item.get("id").cloned()) {
        return json!([id]);
    }
    let Some(first) = items.first_mut() else {
        return json!([]);
    };
    first["id"] = json!("evidence:authority");
    json!(["evidence:authority"])
}
