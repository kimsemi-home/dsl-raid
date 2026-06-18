use super::fixtures::{base_manifest, high};
use serde_json::json;

#[test]
fn approved_manifest_rejects_missing_review_evidence_and_lease() {
    let value = base_manifest(json!([]), "active", json!([]));

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "active lease blocks verified artifact artifact:runtime-rust",
            "approved authority gate requires evidence",
            "approved run requires finished lease",
            "approved run requires high quality evidence",
            "approved run requires trace evidence",
            "approved run requires coverage evidence",
            "approved run requires high quality evidence snapshot",
            "orchestration receipt requires evidence",
            "shadow orchestration requires evidence",
            "approved run requires independent reviewer",
            "approved run requires cross-agent agreement",
            "approved run requires semantic diff receipt"
        ]
    );
}

#[test]
fn verified_manifest_requires_approved_gate() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["decision"] = json!("blocked");

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec!["verified run requires approved authority gate"]
    );
}

#[test]
fn approved_manifest_rejects_self_approval_and_open_debt() {
    let mut value = base_manifest(json!([{ "id": "reviewer:quality" }]), "finished", high());
    value["authority_gate"]["approved_by"] = json!("agent:codex");
    value["debts"] = json!([{
        "id": "debt:evidence",
        "kind": "evidence",
        "status": "open",
        "owner": "agent:codex",
        "opened_at": "2026-06-17T00:00:00Z",
        "revalidate_at": "2026-06-18T00:00:00Z"
    }]);

    assert_eq!(
        super::super::agent_run::semantic_issues(&value),
        vec![
            "producer cannot approve its own output",
            "approved run cannot carry open debt",
            "open debt debt:evidence requires loop gap evidence"
        ]
    );
}
