use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_rejects_unaccountable_closed_debt() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["debts"] = json!([{
        "id": "debt:review",
        "status": "closed"
    }]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "debt debt:review requires owner",
            "debt debt:review requires opened_at",
            "debt debt:review requires revalidate_at",
            "debt debt:review requires closure evidence",
            "debt debt:review requires feedback closure update"
        ]
    );
}

#[test]
fn approved_manifest_rejects_debt_unknown_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["debts"] = tracked_debt(json!(["evidence:missing"]));

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["debt debt:review references unknown evidence evidence:missing"]
    );
}

#[test]
fn approved_manifest_accepts_closed_debt_with_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = tracked_debt(json!(["evidence:quality"]));

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn tracked_debt(evidence: serde_json::Value) -> serde_json::Value {
    super::fixture::closed_with(evidence, "applied")
}
