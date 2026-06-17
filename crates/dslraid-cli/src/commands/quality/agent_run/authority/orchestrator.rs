use super::super::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(approver) = text(value, &["authority_gate", "approved_by"]) else {
        return;
    };
    let Some(orchestrator) = text(value, &["orchestration", "routed_by"]) else {
        return;
    };
    if approver == orchestrator {
        issues.push(format!(
            "control plane orchestrator {orchestrator} cannot approve authority gate"
        ));
    }
}
