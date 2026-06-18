use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn artifact_claim_requires_artifact_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([claim(json!(["evidence:quality"]))]);

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
    value["claims"] = json!([claim(json!([
        "evidence:quality",
        "evidence:artifact-compare"
    ]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn claim(evidence: Value) -> Value {
    json!({
        "id": "claim:fresh-artifacts",
        "subject": "agent-run:runscope-quality-001",
        "statement": "Generated artifacts match the canonical IR.",
        "confidence": "high",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "0.1.0",
        "status": "supported",
        "evidence": evidence
    })
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
