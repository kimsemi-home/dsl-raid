use super::fixtures::{base_manifest, high};
use super::fixtures_reviewer::adversarial;
use serde_json::json;

#[test]
fn incident_authority_requires_debt_record() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["authority_gate"]["scope"] = json!("incident");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("human:alice");
    value["review_capacity"] = json!({
        "status": "available",
        "queue_depth": 1,
        "max_queue_depth": 5,
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "evidence": ["evidence:quality"]
    });

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["incident authority requires debt record"]
    );
}
