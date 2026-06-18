use serde_json::{json, Value};

mod artifacts;
mod authority_gate;
mod lease;
mod producer;
mod ssot;

pub(super) fn base_manifest(reviewers: Value, lease: &str, mut evidence: Value) -> Value {
    let reviewers = super::reviewer::with_defaults(reviewers);
    super::evidence::with_subject(&mut evidence);
    let gate_evidence = super::authority::evidence(&mut evidence);
    let agreements = super::agreement::agreements(&reviewers, &gate_evidence);
    let orchestration = super::orchestration::receipt(&reviewers, &gate_evidence);
    let semantic_diffs = super::semantic::diffs(&evidence);
    json!({
        "run": { "id": "agent-run:runscope-quality-001", "status": "verified" },
        "ssot": ssot::fixture(),
        "producer": producer::fixture(),
        "reviewers": reviewers,
        "agreements": agreements,
        "semantic_diffs": semantic_diffs,
        "authority_gate": authority_gate::fixture(gate_evidence),
        "orchestration": orchestration,
        "lease": self::lease::record(lease),
        "evidence": evidence,
        "artifacts": artifacts::fixture(),
        "debts": []
    })
}
