use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn trusted_reviewer_requires_reliability_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["reviewers"][0]["trust_tier"] = json!("T3");

    assert!(super::super::super::agent_run::semantic_issues(&value)
        .contains(&"trusted reviewer reviewer:quality requires reliability evidence".to_string()));
}

#[test]
fn trusted_reviewer_accepts_reliability_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["reviewers"][0]["trust_tier"] = json!("T3");
    attach_reliability(&mut value);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

fn attach_reliability(value: &mut serde_json::Value) {
    value["authority_gate"]["evidence"]
        .as_array_mut()
        .unwrap()
        .push(json!("evidence:reviewer-quality"));
    value["evidence"].as_array_mut().unwrap().push(json!({
        "id": "evidence:reviewer-quality",
        "kind": "decision",
        "subject": "reviewer:quality",
        "links": [{ "relation": "corroborates", "target": "evidence:quality" }],
        "provenance": {
            "kind": "human-annotation",
            "observed_by": "human:alice",
            "observed_at": "2026-06-17T00:00:00Z",
            "ontology_version": "0.1.0"
        }
    }));
}
