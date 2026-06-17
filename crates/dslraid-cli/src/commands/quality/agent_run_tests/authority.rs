use super::fixtures::{base_manifest, high};
use super::fixtures_reviewer::adversarial;
use serde_json::json;

#[test]
fn approved_manifest_requires_authority_profile_scope_and_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let gate = value["authority_gate"].as_object_mut().unwrap();
    gate.remove("profile");
    gate.remove("scope");
    gate.remove("evidence");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "approved authority gate requires evidence",
            "approved authority gate requires profile",
            "approved authority gate requires scope"
        ]
    );
}

#[test]
fn approved_manifest_rejects_unknown_authority_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["evidence"] = json!(["evidence:missing"]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["authority gate references unknown evidence evidence:missing"]
    );
}

#[test]
fn authority_evidence_requires_validation_or_decision() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["evidence"] = json!(["evidence:trace"]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved authority gate requires validation or decision evidence"]
    );
}

#[test]
fn human_review_requires_human_or_steward_approver() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("gate:quality");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["human review authority gate requires human or steward approver"]
    );
}

#[test]
fn ontology_scope_requires_human_review() {
    let mut value = base_manifest(adversarial(), "finished", high());
    value["authority_gate"]["scope"] = json!("ontology");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "authority scope ontology requires human review",
            "high-risk authority requires review capacity receipt"
        ]
    );
}
