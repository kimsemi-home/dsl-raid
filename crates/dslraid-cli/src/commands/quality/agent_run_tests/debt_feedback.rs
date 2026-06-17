use super::debt_fixture::{closed_with, update};
use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn closed_debt_requires_feedback_update() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = closed_with(json!(["evidence:quality"]), "applied");
    value["debts"][0]["updates"] = json!([]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["debt debt:review requires feedback closure update"]
    );
}

#[test]
fn feedback_update_must_be_applied() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = closed_with(json!(["evidence:quality"]), "proposed");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["debt debt:review has unapplied feedback update update:review-policy"]
    );
}

#[test]
fn feedback_update_requires_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = closed_with(json!(["evidence:quality"]), "applied");
    value["debts"][0]["updates"] = json!([update("applied", json!([]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["feedback update update:review-policy requires evidence"]
    );
}

#[test]
fn feedback_update_rejects_unknown_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = closed_with(json!(["evidence:quality"]), "applied");
    value["debts"][0]["updates"] = json!([update("applied", json!(["evidence:missing"]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["feedback update update:review-policy references unknown evidence evidence:missing"]
    );
}

#[test]
fn review_debt_requires_policy_ontology_or_spec_update() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = closed_with(json!(["evidence:quality"]), "applied");
    value["debts"][0]["updates"][0]["kind"] = json!("revalidation");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["debt debt:review requires policy, ontology, or spec knowledge update"]
    );
}
