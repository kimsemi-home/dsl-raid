use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn agreement_subject_must_match_run() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["agreements"][0]["subject"] = json!("agent-run:other");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["agreement agreement:quality subject differs from run agent-run:runscope-quality-001"]
    );
}
