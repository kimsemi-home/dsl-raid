use serde_json::{json, Value};

pub(in super::super) fn fresh(confidence: &str, assessor: &str, evidence: Value) -> Value {
    claim(
        "claim:fresh-artifacts",
        "Fresh conformance matches the canonical IR.",
        confidence,
        assessor,
        evidence,
    )
}

pub(in super::super) fn artifact(evidence: Value) -> Value {
    claim(
        "claim:fresh-artifacts",
        "Generated artifacts match the canonical IR.",
        "high",
        "sidecar:dslraid-quality",
        evidence,
    )
}

pub(in super::super) fn root_cause(evidence: Value) -> Value {
    claim(
        "claim:root-cause",
        "Root cause is the stale generated artifact path.",
        "medium",
        "sidecar:dslraid-quality",
        evidence,
    )
}

fn claim(id: &str, statement: &str, confidence: &str, assessor: &str, evidence: Value) -> Value {
    json!({
        "id": id,
        "subject": "agent-run:runscope-quality-001",
        "statement": statement,
        "confidence": confidence,
        "assessor": assessor,
        "interpreted_under": "0.1.0",
        "verification_plan": "verification:quality",
        "status": "supported",
        "evidence": evidence
    })
}
