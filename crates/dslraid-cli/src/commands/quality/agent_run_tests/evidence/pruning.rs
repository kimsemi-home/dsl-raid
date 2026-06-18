use super::super::fixtures::{base_manifest, high};
use super::super::fixtures::{push_pruned_extra, tombstone};
use serde_json::json;

#[test]
fn pruned_evidence_requires_tombstone_fields() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    push_pruned_extra(&mut value);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "pruned evidence evidence:old-validation requires tombstone reason",
            "pruned evidence evidence:old-validation requires tombstone pruned_by",
            "pruned evidence evidence:old-validation requires tombstone pruned_at",
            "pruned evidence evidence:old-validation requires tombstone policy_hash"
        ]
    );
}

#[test]
fn pruned_evidence_with_tombstone_is_accepted() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    push_pruned_extra(&mut value);
    value["evidence"][3]["tombstone"] = tombstone();

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

#[test]
fn pruned_evidence_does_not_count_as_active_support() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][1]["status"] = json!("pruned");
    value["evidence"][1]["tombstone"] = tombstone();

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires trace evidence"]
    );
}

#[test]
fn protected_retention_blocks_pruning() {
    for retention in ["protected", "legal_hold"] {
        let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
        push_pruned_extra(&mut value);
        value["evidence"][3]["retention"] = json!(retention);
        value["evidence"][3]["tombstone"] = tombstone();

        assert_eq!(
            super::super::super::agent_run::semantic_issues(&value),
            vec![format!(
                "evidence evidence:old-validation retention {retention} blocks pruning"
            )]
        );
    }
}
