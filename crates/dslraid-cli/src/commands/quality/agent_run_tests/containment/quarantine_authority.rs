use super::super::fixtures::{base_manifest, high};
use super::super::fixtures_authority::attach_producer_reliability;
use serde_json::{json, Value};

#[test]
fn open_quarantine_blocks_automatic_authority() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["producer"]["trust_tier"] = json!("T3");
    attach_producer_reliability(&mut value);
    value["authority_gate"]["profile"] = json!("automatic");
    value["orchestration"]["authority_profile"] = json!("automatic");
    value["containments"] = json!([quarantine()]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "open quarantine blocks verified artifact artifact:runtime-rust",
            "open quarantine blocks automatic authority approval",
        ]
    );
}

fn quarantine() -> Value {
    json!({
        "id": "containment:quarantine",
        "kind": "quarantine",
        "subject": "agent-run:runscope-quality-001",
        "reason": "Contain suspicious output.",
        "status": "open",
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": ["evidence:quality"]
    })
}
