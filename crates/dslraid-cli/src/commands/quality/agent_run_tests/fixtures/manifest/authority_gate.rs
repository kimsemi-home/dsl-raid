use serde_json::{json, Value};

pub(super) fn fixture(evidence: Value) -> Value {
    json!({
        "decision": "approved",
        "policy_hash": "sha256:policy",
        "profile": "sidecar",
        "scope": "routine",
        "human_review_required": false,
        "approved_by": "gate:quality",
        "evidence": evidence
    })
}
