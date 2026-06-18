use super::super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn human_review_requires_human_or_steward_approver() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("gate:quality");

    assert_eq!(
        super::super::super::super::agent_run::semantic_issues(&value),
        vec!["human review authority gate requires human or steward approver"]
    );
}
