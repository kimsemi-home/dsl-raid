use super::fixtures::{base_manifest, fresh_lock, high};
use serde_json::json;
use std::path::Path;

#[test]
fn approved_manifest_rejects_stale_lock_link() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["artifacts"] = json!([{ "path": "generated/runtime_fsm.rs", "status": "verified" }]);
    let mut lock = fresh_lock();
    lock["artifacts"][0]["status"] = json!("stale");

    assert_eq!(
        super::super::agent_run::semantic_issues_with_context(&value, &lock, Path::new(".")),
        vec!["verified artifact generated/runtime_fsm.rs must be fresh in lock"]
    );
}

#[test]
fn approved_manifest_rejects_core_hash_mismatch() {
    let value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut lock = fresh_lock();
    lock["core"]["ir_hash"] = json!("sha256:other");

    assert_eq!(
        super::super::agent_run::semantic_issues_with_context(&value, &lock, Path::new(".")),
        vec!["manifest core_ir_hash differs from lock core hash"]
    );
}
