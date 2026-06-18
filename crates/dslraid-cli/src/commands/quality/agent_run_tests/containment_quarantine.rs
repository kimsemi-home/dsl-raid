use super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn open_quarantine_blocks_high_confidence_claim() {
    let mut value = base_manifest(json!([]), "quarantined", high());
    value["run"]["status"] = json!("quarantined");
    value["authority_gate"]["decision"] = json!("blocked");
    value["containments"] = json!([quarantine()]);
    value["artifacts"] = json!([]);
    value["claims"] = json!([claim()]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["open quarantine blocks high confidence claim claim:fresh-artifacts"]
    );
}

fn quarantine() -> Value {
    json!({
        "id": "containment:quarantine",
        "kind": "quarantine",
        "subject": "agent-run:runscope-quality-001",
        "reason": "Contain suspicious work.",
        "status": "open",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"]
    })
}

fn claim() -> Value {
    json!({
        "id": "claim:fresh-artifacts",
        "subject": "agent-run:runscope-quality-001",
        "statement": "Fresh conformance matches the canonical IR.",
        "confidence": "high",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "0.1.0",
        "verification_plan": "verification:quality",
        "status": "supported",
        "evidence": ["evidence:quality"]
    })
}
