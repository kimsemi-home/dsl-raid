mod steward;

use super::fixtures::{base_manifest, high};
use super::fixtures_authority::attach_producer_reliability;
use super::fixtures_reviewer::adversarial;
use serde_json::{json, Value};

#[test]
fn authority_scope_requires_governance_profile() {
    let value = authority_manifest("sidecar", "steward:ops");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["authority scope authority requires governance profile"]
    );
}

#[test]
fn governance_profile_requires_steward_approver() {
    let mut value = authority_manifest("governance", "human:alice");
    value["orchestration"]["authority_profile"] = json!("governance");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["governance authority profile requires steward approver"]
    );
}

fn authority_manifest(profile: &str, approver: &str) -> Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["producer"]["trust_tier"] = json!("T3");
    attach_producer_reliability(&mut value);
    value["authority_gate"]["profile"] = json!(profile);
    value["authority_gate"]["scope"] = json!("authority");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!(approver);
    value["review_capacity"] = capacity();
    value
}

fn capacity() -> Value {
    json!({
        "status": "available",
        "queue_depth": 1,
        "max_queue_depth": 5,
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "evidence": ["evidence:quality"]
    })
}
