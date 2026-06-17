use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn evidence_subject_must_match_run() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["subject"] = json!("agent-run:other");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality subject differs from run agent-run:runscope-quality-001"]
    );
}
