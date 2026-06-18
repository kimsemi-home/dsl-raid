use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn evidence_subject_must_match_run() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["subject"] = json!("agent-run:other");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality subject agent-run:other is not authorized for run agent-run:runscope-quality-001"]
    );
}

#[test]
fn authority_evidence_can_describe_approver() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["evidence"] = json!(["evidence:quality"]);
    value["authority_gate"]["approved_by"] = json!("steward:ops");
    value["evidence"][0]["subject"] = json!("steward:ops");

    let issues = super::super::super::agent_run::semantic_issues(&value);
    assert!(issues.iter().all(|issue| !issue.contains("subject")));
}
