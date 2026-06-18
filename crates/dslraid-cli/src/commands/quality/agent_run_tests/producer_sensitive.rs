use super::fixtures::{base_manifest, high};
use super::fixtures_authority::attach_producer_reliability;
use super::fixtures_reviewer::adversarial;
use serde_json::json;

#[test]
fn security_authority_requires_trusted_producer() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["authority_gate"]["scope"] = json!("security");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = capacity();
    attach_producer_reliability(&mut value);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["security authority requires producer trust tier T3 or T4"]
    );
}

#[test]
fn security_authority_accepts_trusted_producer() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["producer"]["trust_tier"] = json!("T3");
    value["authority_gate"]["scope"] = json!("security");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = capacity();
    attach_producer_reliability(&mut value);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

#[test]
fn audit_authority_requires_trusted_producer() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["authority_gate"]["scope"] = json!("audit");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = capacity();

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["audit authority requires producer trust tier T3 or T4"]
    );
}

fn capacity() -> serde_json::Value {
    json!({
        "status": "available",
        "queue_depth": 1,
        "max_queue_depth": 5,
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "evidence": ["evidence:quality"]
    })
}
