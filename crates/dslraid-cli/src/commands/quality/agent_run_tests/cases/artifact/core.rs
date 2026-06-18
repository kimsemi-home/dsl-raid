use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_requires_output_artifact_record() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["artifacts"] = json!([]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires output artifact record"]
    );
}

#[test]
fn approved_manifest_rejects_stale_artifact() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["artifacts"][0]["status"] = json!("stale");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run cannot carry stale artifact artifact:runtime-rust"]
    );
}

#[test]
fn approved_manifest_rejects_candidate_artifact() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["artifacts"][0]["status"] = json!("candidate");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run cannot carry candidate artifact artifact:runtime-rust"]
    );
}
