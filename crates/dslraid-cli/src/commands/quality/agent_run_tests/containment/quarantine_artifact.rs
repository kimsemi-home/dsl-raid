use super::super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn open_quarantine_blocks_verified_artifact() {
    let mut value = base_manifest(json!([]), "quarantined", high());
    value["run"]["status"] = json!("quarantined");
    value["authority_gate"]["decision"] = json!("blocked");
    value["containments"] = json!([{
        "id": "containment:quarantine",
        "kind": "quarantine",
        "subject": "agent-run:runscope-quality-001",
        "reason": "Contain suspicious output.",
        "status": "open",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"]
    }]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "open quarantine blocks verified artifact artifact:runtime-rust",
            "quarantined lease blocks verified artifact artifact:runtime-rust"
        ]
    );
}
