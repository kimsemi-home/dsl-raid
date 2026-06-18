use super::super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_rejects_unknown_authority_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["evidence"] = json!(["evidence:missing"]);

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        vec!["authority gate references unknown evidence evidence:missing"]
    );
}

#[test]
fn authority_evidence_requires_validation_or_decision() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["evidence"] = json!(["evidence:trace"]);

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        vec!["approved authority gate requires validation or decision evidence"]
    );
}
