use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn high_confidence_claim_requires_verification_plan() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = super::fixture::fresh_high(json!(["evidence:quality"]));
    item.as_object_mut().unwrap().remove("verification_plan");
    value["claims"] = json!([item]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["high confidence claim claim:fresh-artifacts requires verification plan"]
    );
}
