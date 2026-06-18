use serde_json::json;

#[test]
fn governance_steward_requires_authority_evidence() {
    let mut value = super::authority_manifest("governance", "steward:ops");
    value["orchestration"]["authority_profile"] = json!("governance");

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        vec!["governance steward approver steward:ops requires authority evidence"]
    );
}

#[test]
fn governance_steward_accepts_authority_evidence() {
    let mut value = super::authority_manifest("governance", "steward:ops");
    value["orchestration"]["authority_profile"] = json!("governance");
    value["authority_gate"]["evidence"]
        .as_array_mut()
        .unwrap()
        .push(json!("evidence:steward-ops"));
    value["evidence"]
        .as_array_mut()
        .unwrap()
        .push(steward_evidence());

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn steward_evidence() -> serde_json::Value {
    json!({
        "id": "evidence:steward-ops",
        "kind": "decision",
        "uri": "evidence/steward-ops.json",
        "subject": "steward:ops",
        "provenance": {
            "kind": "human-annotation",
            "observed_by": "human:alice",
            "observed_at": "2026-06-17T00:00:00Z",
            "ontology_version": "0.1.0"
        }
    })
}
