use super::super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_authority_profile_scope_and_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let gate = value["authority_gate"].as_object_mut().unwrap();
    gate.remove("profile");
    gate.remove("scope");
    gate.remove("evidence");

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        vec![
            "approved authority gate requires evidence",
            "approved authority gate requires profile",
            "approved authority gate requires scope"
        ]
    );
}
