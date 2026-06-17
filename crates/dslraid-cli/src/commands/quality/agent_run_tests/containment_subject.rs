use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn containment_requires_run_subject() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["containments"] = json!([{
        "id": "containment:abort",
        "kind": "abort",
        "reason": "Contain interrupted work.",
        "status": "open",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"]
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["containment containment:abort requires subject"]
    );
}

#[test]
fn containment_subject_must_match_run() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["containments"] = json!([{
        "id": "containment:abort",
        "kind": "abort",
        "subject": "agent-run:other",
        "reason": "Contain interrupted work.",
        "status": "open",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"]
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "containment containment:abort subject agent-run:other must match run agent-run:runscope-quality-001"
        ]
    );
}
