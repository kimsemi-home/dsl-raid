use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn approved_manifest_requires_semantic_diff() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["semantic_diffs"] = json!([]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires semantic diff receipt"]
    );
}

#[test]
fn semantic_diff_head_must_match_ssot_hash() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["semantic_diffs"] = json!([diff("sha256:other", json!(["evidence:quality"]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["semantic diff semantic-diff:quality head_hash differs from ssot core hash"]
    );
}

#[test]
fn semantic_diff_rejects_unknown_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["semantic_diffs"] = json!([diff("sha256:core", json!(["evidence:missing"]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["semantic diff semantic-diff:quality references unknown evidence evidence:missing"]
    );
}

#[test]
fn changed_semantic_diff_requires_validation_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["semantic_diffs"] = json!([diff("sha256:core", json!(["evidence:trace"]))]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["changed semantic diff semantic-diff:quality requires validation evidence"]
    );
}

#[test]
fn blocked_semantic_diff_cannot_approve() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut item = diff("sha256:core", json!(["evidence:quality"]));
    item["status"] = json!("blocked");
    value["semantic_diffs"] = json!([item]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run cannot use blocked semantic diff semantic-diff:quality"]
    );
}

fn diff(head_hash: &str, evidence: Value) -> Value {
    json!({
        "id": "semantic-diff:quality",
        "base_hash": "sha256:base",
        "head_hash": head_hash,
        "status": "changed",
        "evidence": evidence
    })
}
