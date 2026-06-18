use serde_json::{json, Value};

pub(super) fn diffs(evidence: &Value) -> Value {
    let Some(reference) = evidence_id(evidence) else {
        return json!([]);
    };
    json!([{
        "id": "semantic-diff:quality",
        "base_hash": "sha256:core",
        "head_hash": "sha256:core",
        "status": "unchanged",
        "evidence": [reference]
    }])
}

fn evidence_id(evidence: &Value) -> Option<Value> {
    evidence
        .as_array()
        .and_then(|items| items.first())
        .and_then(|item| item.get("id"))
        .cloned()
}
