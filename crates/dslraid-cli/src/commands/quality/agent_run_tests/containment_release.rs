use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn released_containment_requires_met_conditions() {
    let mut value = base_manifest(json!([]), "finished", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("blocked");
    value["containments"] = json!([{
        "id": "containment:quarantine",
        "kind": "quarantine",
        "reason": "Contain suspicious work.",
        "status": "released",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"],
        "release_conditions": [{
            "id": "release:review",
            "description": "Independent review complete.",
            "status": "pending",
            "evidence": ["evidence:quality"]
        }]
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["released containment containment:quarantine has unmet release condition"]
    );
}
