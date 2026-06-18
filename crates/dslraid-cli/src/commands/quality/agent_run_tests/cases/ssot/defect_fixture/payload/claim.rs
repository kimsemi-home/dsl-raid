use serde_json::{json, Value};

pub(super) fn fixture(plan: Option<&str>) -> Value {
    let mut value = json!({
        "id": "claim:ssot-defect",
        "subject": "agent-run:runscope-quality-001",
        "statement": "SSOT Defect is confirmed in the canonical IR.",
        "confidence": "medium",
        "assessor": "sidecar:dslraid-quality",
        "interpreted_under": "0.1.0",
        "status": "supported",
        "evidence": ["evidence:quality"]
    });
    if let Some(plan) = plan {
        value["verification_plan"] = json!(plan);
    }
    value
}
