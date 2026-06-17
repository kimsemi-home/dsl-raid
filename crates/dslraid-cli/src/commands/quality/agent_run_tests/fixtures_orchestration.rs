use serde_json::{json, Value};

pub(super) fn receipt(reviewers: &Value, evidence: &Value) -> Value {
    json!({
        "id": "orchestration:runscope-quality",
        "policy_hash": "sha256:policy",
        "selected_producer": "agent:codex",
        "selected_reviewers": reviewer_ids(reviewers),
        "lease": "lease:runscope-quality-001",
        "authority_profile": "sidecar",
        "input_evidence": evidence,
        "output_artifacts": ["artifact:runtime-rust"],
        "evidence": evidence
    })
}

fn reviewer_ids(value: &Value) -> Vec<String> {
    value
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|item| item.get("id").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}
