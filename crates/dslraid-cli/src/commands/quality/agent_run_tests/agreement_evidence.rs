use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn agreement_rejects_review_only_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"]
        .as_array_mut()
        .unwrap()
        .push(review_evidence());
    value["agreements"][0]["evidence"] = json!(["evidence:review-only"]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["agreement agreement:quality cannot rely only on review evidence"]
    );
}

fn review_evidence() -> serde_json::Value {
    json!({
        "id": "evidence:review-only",
        "kind": "review",
        "subject": "agent-run:runscope-quality-001",
        "provenance": {
            "kind": "human-annotation",
            "observed_by": "human:alice",
            "observed_at": "2026-06-17T00:00:00Z",
            "ontology_version": "0.1.0"
        }
    })
}
