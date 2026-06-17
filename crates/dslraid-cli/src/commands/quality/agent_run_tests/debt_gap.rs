use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn open_debt_requires_debt_kind_gap_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["debts"] = open_debt(json!(["evidence:quality"]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "approved run cannot carry open debt",
            "open debt debt:loop-gap requires loop gap evidence"
        ]
    );
}

#[test]
fn open_debt_accepts_debt_kind_gap_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"].as_array_mut().unwrap().push(json!({
        "id": "evidence:loop-gap",
        "kind": "debt",
        "uri": "agent-run://loop-gap/review",
        "subject": "agent-run:runscope-quality-001",
        "provenance": {
            "kind": "sidecar-assessment",
            "observed_by": "sidecar:dslraid-quality",
            "observed_at": "2026-06-17T00:00:00Z"
        }
    }));
    value["debts"] = open_debt(json!(["evidence:loop-gap"]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run cannot carry open debt"]
    );
}

fn open_debt(evidence: serde_json::Value) -> serde_json::Value {
    json!([{
        "id": "debt:loop-gap",
        "kind": "review",
        "status": "open",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "revalidate_at": "2026-06-18T00:00:00Z",
        "evidence": evidence
    }])
}
