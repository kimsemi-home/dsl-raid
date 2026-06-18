mod reliability;

use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn independent_reviewer_requires_capability_receipt() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let reviewer = value["reviewers"][0].as_object_mut().unwrap();
    reviewer.remove("role");
    reviewer.remove("reasoning_level");
    reviewer.remove("trust_tier");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "independent reviewer reviewer:quality requires role",
            "independent reviewer reviewer:quality requires reasoning level",
            "independent reviewer reviewer:quality requires trust tier"
        ]
    );
}

#[test]
fn approved_manifest_rejects_cold_start_reviewer() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["reviewers"][0]["trust_tier"] = json!("T1");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run cannot use cold-start reviewer reviewer:quality"]
    );
}
