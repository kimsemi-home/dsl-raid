use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn orchestrator_cannot_approve_authority_gate() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["approved_by"] = json!("control-plane:dslraid");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["control plane orchestrator control-plane:dslraid cannot approve authority gate"]
    );
}
