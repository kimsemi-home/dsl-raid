use super::super::fixtures::{base_manifest, high};
use super::super::fixtures_reviewer::adversarial;
use super::fixture::closed_with;
use serde_json::json;

#[test]
fn incident_authority_requires_debt_record() {
    let value = incident_manifest();

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["incident authority requires debt record"]
    );
}

#[test]
fn incident_authority_requires_evidence_debt() {
    let mut value = incident_manifest();
    value["debts"] = closed_with(json!(["evidence:quality"]), "applied");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["incident authority requires evidence debt record"]
    );
}

#[test]
fn incident_authority_accepts_evidence_debt() {
    let mut value = incident_manifest();
    value["debts"] = closed_with(json!(["evidence:quality"]), "applied");
    value["debts"][0]["kind"] = json!("evidence");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn incident_manifest() -> serde_json::Value {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["authority_gate"]["scope"] = json!("incident");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = capacity();
    value
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
