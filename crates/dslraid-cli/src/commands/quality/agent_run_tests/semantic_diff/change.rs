use super::super::fixtures::{base_manifest, high};
use crate::commands::quality::agent_run;
use serde_json::json;

#[test]
fn changed_semantic_diff_requires_cause_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["semantic_diffs"] = json!([super::diff("sha256:core", json!(["evidence:quality"]))]);

    assert_eq!(
        agent_run::semantic_issues(&value),
        vec!["changed semantic diff semantic-diff:quality requires cause evidence"]
    );
}
