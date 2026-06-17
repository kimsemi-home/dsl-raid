use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn agreement_requires_producer_participant() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["agreements"][0]["participants"] = json!(["agent:other", "reviewer:quality"]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["agreement agreement:quality requires producer participant"]
    );
}
