use serde_json::{json, Value};

pub(super) fn diff(head_hash: &str, evidence: Value) -> Value {
    json!({
        "id": "semantic-diff:quality",
        "base_hash": "sha256:base",
        "head_hash": head_hash,
        "status": "changed",
        "evidence": evidence
    })
}
