use serde_json::{json, Value};

mod agreement;
mod authority;
mod evidence;
mod links;
mod orchestration;
mod pruning;
mod reviewer;
mod semantic;
mod surface;

pub(super) use surface::{
    adversarial, attach_producer_reliability, fresh_lock, high, high_snapshot, push_pruned_extra,
    tombstone,
};

pub(super) fn base_manifest(reviewers: Value, lease: &str, mut evidence: Value) -> Value {
    let reviewers = reviewer::with_defaults(reviewers);
    evidence::with_subject(&mut evidence);
    let gate_evidence = authority::evidence(&mut evidence);
    let agreements = agreement::agreements(&reviewers, &gate_evidence);
    let orchestration = orchestration::receipt(&reviewers, &gate_evidence);
    let semantic_diffs = semantic::diffs(&evidence);
    json!({
        "run": { "id": "agent-run:runscope-quality-001", "status": "verified" },
        "ssot": {
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
        },
        "producer": { "id": "agent:codex", "role": "implementation", "reasoning_level": "R3", "trust_tier": "T2" },
        "reviewers": reviewers,
        "agreements": agreements,
        "semantic_diffs": semantic_diffs,
        "authority_gate": {
            "decision": "approved",
            "policy_hash": "sha256:policy",
            "profile": "sidecar",
            "scope": "routine",
            "human_review_required": false,
            "approved_by": "gate:quality",
            "evidence": gate_evidence
        },
        "orchestration": orchestration,
        "lease": { "id": "lease:runscope-quality-001", "status": lease, "ontology_version": "0.1.0" },
        "evidence": evidence,
        "artifacts": [
            {
                "id": "artifact:runtime-rust",
                "path": "generated/runtime_fsm.rs",
                "status": "verified"
            }
        ],
        "debts": []
    })
}
