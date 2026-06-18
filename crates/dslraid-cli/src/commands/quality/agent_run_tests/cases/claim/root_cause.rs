use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn root_cause_claim_requires_validation_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][1]["quality_snapshots"][0]["purpose"] = json!("root_cause");
    value["claims"] = json!([super::fixture::root_cause(json!(["evidence:trace"]))]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["root cause claim claim:root-cause requires validation evidence"]
    );
}

#[test]
fn root_cause_claim_requires_root_cause_snapshot() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["claims"] = json!([super::fixture::root_cause(json!(["evidence:quality"]))]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["root cause claim claim:root-cause requires root_cause quality snapshot"]
    );
}
