use serde_json::{json, Value};

pub(super) fn agreements(reviewers: &Value, evidence: &Value) -> Value {
    let Some(reviewer) = reviewer(reviewers) else {
        return json!([]);
    };
    if evidence.as_array().is_none_or(Vec::is_empty) {
        return json!([]);
    }
    json!([{
        "id": "agreement:quality",
        "subject": "agent-run:runscope-quality-001",
        "participants": ["agent:codex", reviewer],
        "decision": "agree",
        "interpreted_under": "0.1.0",
        "evidence": evidence
    }])
}

fn reviewer(reviewers: &Value) -> Option<Value> {
    reviewers
        .as_array()
        .and_then(|items| items.first())
        .and_then(|item| item.get("id"))
        .cloned()
}
