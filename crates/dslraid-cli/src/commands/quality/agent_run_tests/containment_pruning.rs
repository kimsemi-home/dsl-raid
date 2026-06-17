use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn containment_rejects_pruned_evidence() {
    let mut value = base_manifest(json!([]), "finished", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("blocked");
    value["evidence"][0]["status"] = json!("pruned");
    value["containments"] = json!([{
        "id": "containment:abort",
        "kind": "abort",
        "subject": "agent-run:runscope-quality-001",
        "reason": "Contain interrupted work.",
        "status": "closed",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"]
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["containment containment:abort references pruned evidence evidence:quality"]
    );
}
