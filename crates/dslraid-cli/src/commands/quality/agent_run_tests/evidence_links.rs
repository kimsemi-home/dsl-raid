use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_evidence_graph_link() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    for evidence in value["evidence"].as_array_mut().unwrap() {
        evidence.as_object_mut().unwrap().remove("links");
    }

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires evidence graph link"]
    );
}

#[test]
fn evidence_link_target_must_exist() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["links"][0]["target"] = json!("evidence:missing");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality link target evidence:missing is not evidence"]
    );
}

#[test]
fn evidence_link_cannot_target_itself() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["links"][0]["target"] = json!("evidence:quality");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["evidence evidence:quality link cannot target itself"]
    );
}
