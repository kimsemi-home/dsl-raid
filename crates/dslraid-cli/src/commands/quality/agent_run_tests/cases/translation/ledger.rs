use super::super::fixtures::{base_manifest, high};
use crate::commands::quality::agent_run;
use serde_json::json;

#[test]
fn lossy_translation_requires_loss_ledger() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["translations"] = json!([super::core::translation("lossy", "target", true, json!([]))]);

    assert_eq!(
        agent_run::semantic_issues(&value),
        vec!["lossy translation translation:lisp-to-ir requires loss ledger"]
    );
}

#[test]
fn loss_requires_evidence() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let mut loss = super::core::loss("L1");
    loss["evidence"] = json!([]);
    value["translations"] = json!([super::core::translation(
        "lossy",
        "target",
        true,
        json!([loss])
    )]);

    assert_eq!(
        agent_run::semantic_issues(&value),
        vec!["loss loss:macro-detail requires evidence"]
    );
}

#[test]
fn forbidden_loss_blocks_translation() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    let losses = json!([super::core::loss("L4")]);
    value["translations"] = json!([super::core::translation("lossy", "target", false, losses)]);

    assert_eq!(
        agent_run::semantic_issues(&value),
        vec!["translation translation:lisp-to-ir contains forbidden loss loss:macro-detail"]
    );
}
