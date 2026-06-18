use serde_json::{json, Value};

pub(super) fn receipt(reviewers: &Value, evidence: &Value) -> Value {
    let reviewer_ids = reviewer_ids(reviewers);
    let verifier = reviewer_ids.first().cloned().unwrap_or_default();
    json!({
        "id": "orchestration:runscope-quality",
        "ontology_version": "0.1.0",
        "policy_hash": "sha256:policy",
        "routed_by": "control-plane:dslraid",
        "verified_by": verifier,
        "selected_producer": "agent:codex",
        "selected_reviewers": reviewer_ids,
        "lease": "lease:runscope-quality-001",
        "authority_profile": "sidecar",
        "input_evidence": evidence,
        "output_artifacts": ["artifact:runtime-rust"],
        "evidence": evidence,
        "shadow": {
            "routed_by": "control-plane:dslraid-shadow",
            "severity": "D0",
            "evidence": evidence
        }
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
