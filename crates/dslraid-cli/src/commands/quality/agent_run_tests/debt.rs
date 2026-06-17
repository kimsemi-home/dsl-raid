use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_rejects_unaccountable_closed_debt() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["debts"] = json!([{
        "id": "debt:review",
        "status": "closed"
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "debt debt:review requires owner",
            "debt debt:review requires opened_at",
            "debt debt:review requires revalidate_at",
            "debt debt:review requires closure evidence"
        ]
    );
}

#[test]
fn approved_manifest_rejects_debt_unknown_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["debts"] = tracked_debt(json!(["evidence:missing"]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["debt debt:review references unknown evidence evidence:missing"]
    );
}

#[test]
fn approved_manifest_accepts_closed_debt_with_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["id"] = json!("evidence:quality");
    value["debts"] = tracked_debt(json!(["evidence:quality"]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn tracked_debt(evidence: serde_json::Value) -> serde_json::Value {
    json!([{
        "id": "debt:review",
        "kind": "review",
        "status": "closed",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "revalidate_at": "2026-06-18T00:00:00Z",
        "closed_at": "2026-06-17T01:00:00Z",
        "evidence": evidence
    }])
}
