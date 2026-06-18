use serde_json::{json, Value};

pub(super) fn base_manifest(reviewers: Value, lease: &str, mut evidence: Value) -> Value {
    let reviewers = super::reviewer::with_defaults(reviewers);
    super::evidence::with_subject(&mut evidence);
    let gate_evidence = super::authority::evidence(&mut evidence);
    let agreements = super::agreement::agreements(&reviewers, &gate_evidence);
    let orchestration = super::orchestration::receipt(&reviewers, &gate_evidence);
    let semantic_diffs = super::semantic::diffs(&evidence);
    json!({
        "run": { "id": "agent-run:runscope-quality-001", "status": "verified" },
        "ssot": ssot(),
        "producer": producer(),
        "reviewers": reviewers,
        "agreements": agreements,
        "semantic_diffs": semantic_diffs,
        "authority_gate": authority_gate(gate_evidence),
        "orchestration": orchestration,
        "lease": lease_record(lease),
        "evidence": evidence,
        "artifacts": artifacts(),
        "debts": []
    })
}

fn ssot() -> Value {
    json!({
        "context": "runscope",
        "core_ir": "examples/runscope/runscope.raid.json",
        "core_ir_hash": "sha256:core",
        "ontology_version": "0.1.0",
        "contract_version": "0.1.0",
        "revalidation": {
            "status": "valid",
            "assessed_at": "2026-06-17T00:00:00Z",
            "assessor": "sidecar:dslraid-quality",
            "revalidate_at": "2026-07-17T00:00:00Z"
        }
    })
}

fn producer() -> Value {
    json!({
        "id": "agent:codex",
        "role": "implementation",
        "reasoning_level": "R3",
        "trust_tier": "T2"
    })
}

fn authority_gate(evidence: Value) -> Value {
    json!({
        "decision": "approved",
        "policy_hash": "sha256:policy",
        "profile": "sidecar",
        "scope": "routine",
        "human_review_required": false,
        "approved_by": "gate:quality",
        "evidence": evidence
    })
}

fn lease_record(status: &str) -> Value {
    json!({ "id": "lease:runscope-quality-001", "status": status, "ontology_version": "0.1.0" })
}

fn artifacts() -> Value {
    json!([{ "id": "artifact:runtime-rust", "path": "generated/runtime_fsm.rs", "status": "verified" }])
}
