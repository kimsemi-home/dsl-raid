use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn high_confidence_claim_requires_linked_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]
        .as_object_mut()
        .unwrap()
        .remove("links");
    value["claims"] = json!([super::fixture::fresh_high(json!(["evidence:quality"]))]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "high confidence claim claim:fresh-artifacts requires linked evidence evidence:quality"
        ]
    );
}
