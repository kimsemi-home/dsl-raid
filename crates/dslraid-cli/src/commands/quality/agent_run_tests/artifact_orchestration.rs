use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn artifact_must_be_listed_in_orchestration_outputs() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["artifacts"] = json!([
        {
            "id": "artifact:runtime-rust",
            "path": "generated/runtime_fsm.rs",
            "status": "verified"
        },
        {
            "id": "artifact:runtime-go",
            "path": "generated/runtime_fsm.go",
            "status": "verified"
        }
    ]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["artifact artifact:runtime-go must be listed in orchestration output_artifacts"]
    );
}
