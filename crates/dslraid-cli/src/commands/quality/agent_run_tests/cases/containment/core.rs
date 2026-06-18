use super::super::fixtures::{base_manifest, high};
use serde_json::{json, Value};

#[test]
fn aborted_manifest_requires_abort_bundle() {
    let mut value = base_manifest(json!([]), "aborted", high());
    value["run"]["status"] = json!("aborted");
    value["authority_gate"]["decision"] = json!("rejected");

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec![
            "aborted run requires abort evidence bundle",
            "aborted lease blocks verified artifact artifact:runtime-rust"
        ]
    );
}

#[test]
fn quarantined_artifact_requires_quarantine_bundle() {
    let mut value = base_manifest(json!([]), "quarantined", high());
    value["run"]["status"] = json!("quarantined");
    value["authority_gate"]["decision"] = json!("blocked");
    value["artifacts"] = json!([{ "path": "generated/runtime_fsm.rs", "status": "quarantined" }]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["quarantined output requires quarantine evidence bundle"]
    );
}

#[test]
fn containment_rejects_unknown_evidence() {
    let mut value = base_manifest(json!([]), "finished", high());
    value["run"]["status"] = json!("rejected");
    value["authority_gate"]["decision"] = json!("blocked");
    value["containments"] = json!([bundle(
        "abort",
        "closed",
        json!(["evidence:missing"]),
        json!([])
    )]);

    assert_eq!(
        super::super::super::agent_run::semantic_issues(&value),
        vec!["containment containment:abort references unknown evidence evidence:missing"]
    );
}

fn bundle(kind: &str, status: &str, evidence: Value, conditions: Value) -> Value {
    json!({
        "id": format!("containment:{kind}"),
        "kind": kind,
        "subject": "agent-run:runscope-quality-001",
        "reason": "Contain suspicious or interrupted work.",
        "status": status,
        "owner": "sidecar:dslraid-quality",
        "opened_at": "2026-06-17T00:00:00Z",
        "evidence": evidence,
        "release_conditions": conditions
    })
}
