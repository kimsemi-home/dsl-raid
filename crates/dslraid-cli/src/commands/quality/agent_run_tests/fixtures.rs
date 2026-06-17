use serde_json::{json, Value};

pub(super) fn base_manifest(reviewers: Value, lease: &str, mut evidence: Value) -> Value {
    let reviewers = super::fixtures_reviewer::with_defaults(reviewers);
    let gate_evidence = super::fixtures_authority::evidence(&mut evidence);
    let agreements = super::fixtures_agreement::agreements(&reviewers, &gate_evidence);
    let semantic_diffs = super::fixtures_semantic::diffs(&evidence);
    json!({
        "run": { "id": "agent-run:runscope-quality-001", "status": "verified" },
        "ssot": {
            "core_ir": "examples/runscope/runscope.raid.json",
            "core_ir_hash": "sha256:core",
            "ontology_version": "0.1.0",
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
        "lease": { "status": lease, "ontology_version": "0.1.0" },
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

pub(super) fn high() -> Value {
    let snapshot = high_snapshot();
    json!([
        { "id": "evidence:quality", "quality": "high", "kind": "validation", "quality_snapshots": snapshot.clone() },
        { "id": "evidence:trace", "quality": "high", "kind": "trace", "quality_snapshots": snapshot.clone() },
        { "id": "evidence:coverage", "quality": "high", "kind": "coverage", "quality_snapshots": snapshot }
    ])
}

pub(super) fn high_snapshot() -> Value {
    json!([{
        "assessed_at": "2026-06-17T00:00:00Z",
        "assessor": "sidecar:dslraid-quality",
        "purpose": "authority",
        "quality": "high",
        "ontology_version": "0.1.0"
    }])
}

pub(super) fn fresh_lock() -> Value {
    json!({
        "core": { "ir_hash": "sha256:core" },
        "artifacts": [
            { "path": "generated/runtime_fsm.rs", "status": "fresh" }
        ]
    })
}
