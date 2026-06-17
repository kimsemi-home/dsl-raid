use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn non_approved_authority_requires_evidence() {
    let mut value = base_manifest(json!([]), "finished", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("rejected");
    value["authority_gate"]["evidence"] = json!([]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["non-approved authority gate requires evidence"]
    );
}
