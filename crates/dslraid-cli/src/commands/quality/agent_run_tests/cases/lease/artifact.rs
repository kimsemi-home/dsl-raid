use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn expired_lease_blocks_verified_artifact() {
    let mut value = base_manifest(json!([]), "expired", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("blocked");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["expired lease blocks verified artifact artifact:runtime-rust"]
    );
}

#[test]
fn active_lease_blocks_verified_artifact() {
    let mut value = base_manifest(json!([]), "active", high());
    value["run"]["status"] = json!("running");
    value["authority_gate"]["decision"] = json!("blocked");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["active lease blocks verified artifact artifact:runtime-rust"]
    );
}

#[test]
fn rebase_required_lease_blocks_verified_artifact() {
    let mut value = base_manifest(json!([]), "rebase_required", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("blocked");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["rebase_required lease blocks verified artifact artifact:runtime-rust"]
    );
}
