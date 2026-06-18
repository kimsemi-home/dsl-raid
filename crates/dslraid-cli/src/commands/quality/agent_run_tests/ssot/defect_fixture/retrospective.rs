use serde_json::{json, Value};

pub(super) fn review_debt() -> Value {
    json!({
        "id": "debt:ssot-defect-review",
        "kind": "review",
        "status": "closed",
        "owner": "steward:ops",
        "opened_at": "2026-06-17T00:00:00Z",
        "revalidate_at": "2026-07-17T00:00:00Z",
        "closed_at": "2026-06-17T02:00:00Z",
        "evidence": ["evidence:quality"],
        "updates": [knowledge_update()]
    })
}

fn knowledge_update() -> Value {
    json!({
        "id": "update:ssot-defect-policy",
        "kind": "policy",
        "subject": "policy:agent-quality",
        "status": "applied",
        "evidence": ["evidence:quality"]
    })
}
