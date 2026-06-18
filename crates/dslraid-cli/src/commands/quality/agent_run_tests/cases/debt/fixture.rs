use serde_json::{json, Value};

pub(in crate::commands::quality::agent_run_tests) fn closed_with(
    evidence: Value,
    status: &str,
) -> Value {
    json!([{
        "id": "debt:review",
        "kind": "review",
        "status": "closed",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "revalidate_at": "2026-06-18T00:00:00Z",
        "closed_at": "2026-06-17T01:00:00Z",
        "evidence": evidence,
        "updates": [update(status, json!(["evidence:quality"]))]
    }])
}

pub(in crate::commands::quality::agent_run_tests) fn update(
    status: &str,
    evidence: Value,
) -> Value {
    json!({
        "id": "update:review-policy",
        "kind": "policy",
        "subject": "policy:agent-quality",
        "status": status,
        "evidence": evidence
    })
}
