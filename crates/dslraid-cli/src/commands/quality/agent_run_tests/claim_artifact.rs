use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn artifact_claim_requires_artifact_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([super::claim::fixture::artifact(json!(["evidence:quality"]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["high confidence artifact claim claim:fresh-artifacts requires artifact evidence"]
    );
}

#[test]
fn artifact_claim_accepts_artifact_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"]
        .as_array_mut()
        .unwrap()
        .push(artifact_evidence());
    value["claims"] = json!([super::claim::fixture::artifact(json!([
        "evidence:quality",
        "evidence:artifact-compare"
    ]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn artifact_evidence() -> Value {
    json!({
        "id": "evidence:artifact-compare",
        "kind": "artifact",
        "uri": "cargo run -p dslraid-cli -- artifact verify",
        "subject": "agent-run:runscope-quality-001",
        "links": [{ "relation": "derived_from", "target": "evidence:quality" }],
        "provenance": {
            "kind": "sidecar-assessment",
            "observed_by": "sidecar:dslraid-quality",
            "observed_at": "2026-06-17T00:00:00Z",
            "ontology_version": "0.1.0"
        }
    })
}
