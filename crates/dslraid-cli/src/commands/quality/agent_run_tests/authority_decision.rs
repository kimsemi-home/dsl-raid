use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn non_approved_authority_requires_evidence() {
    let mut value = base_manifest(json!([]), "finished", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("rejected");
    value["authority_gate"]["evidence"] = json!([]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["non-approved authority gate requires evidence"]
    );
}

#[test]
fn escalated_authority_requires_human_review() {
    let mut value = base_manifest(json!([]), "finished", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("escalated");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["escalated authority gate requires human review"]
    );
}

#[test]
fn escalated_authority_requires_human_or_steward_target() {
    let mut value = base_manifest(json!([]), "finished", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("escalated");
    value["authority_gate"]["human_review_required"] = json!(true);
    value["authority_gate"]["approved_by"] = json!("gate:quality");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["escalated authority gate requires human or steward target"]
    );
}
