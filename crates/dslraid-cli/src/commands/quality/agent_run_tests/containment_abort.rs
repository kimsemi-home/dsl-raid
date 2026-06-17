use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn aborted_run_blocks_approved_authority() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["run"]["status"] = json!("aborted");
    value["containments"] = json!([{
        "id": "containment:abort",
        "kind": "abort",
        "subject": "agent-run:runscope-quality-001",
        "reason": "Abort suspicious work.",
        "status": "closed",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"]
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["aborted run cannot have approved authority gate"]
    );
}
