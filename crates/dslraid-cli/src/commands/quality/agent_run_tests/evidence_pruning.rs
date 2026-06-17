use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn pruned_evidence_requires_tombstone_fields() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["status"] = json!("pruned");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "pruned evidence evidence:quality requires tombstone reason",
            "pruned evidence evidence:quality requires tombstone pruned_by",
            "pruned evidence evidence:quality requires tombstone pruned_at",
            "pruned evidence evidence:quality requires tombstone policy_hash"
        ]
    );
}

#[test]
fn pruned_evidence_with_tombstone_is_accepted() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][0]["status"] = json!("pruned");
    value["evidence"][0]["tombstone"] = tombstone();

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        Vec::<String>::new()
    );
}

#[test]
fn pruned_evidence_does_not_count_as_active_support() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["evidence"][1]["status"] = json!("pruned");
    value["evidence"][1]["tombstone"] = tombstone();

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["approved run requires trace evidence"]
    );
}

#[test]
fn protected_retention_blocks_pruning() {
    for retention in ["protected", "legal_hold"] {
        let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
        value["evidence"][0]["status"] = json!("pruned");
        value["evidence"][0]["retention"] = json!(retention);
        value["evidence"][0]["tombstone"] = tombstone();

        assert_eq!(
            super::super::agent_run::semantic_issues(&value),
            vec![format!(
                "evidence evidence:quality retention {retention} blocks pruning"
            )]
        );
    }
}

fn tombstone() -> serde_json::Value {
    json!({
        "reason": "superseded by newer validation",
        "pruned_by": "sidecar:dslraid-quality",
        "pruned_at": "2026-06-18T00:00:00Z",
        "policy_hash": "sha256:policy"
    })
}
