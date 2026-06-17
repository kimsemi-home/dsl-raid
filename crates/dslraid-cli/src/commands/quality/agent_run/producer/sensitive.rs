use crate::commands::quality::agent_run::fields::text;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let Some(scope) = text(value, &["authority_gate", "scope"]) else {
        return;
    };
    if !matches!(scope, "security" | "authority") {
        return;
    }
    if !matches!(text(value, &["producer", "trust_tier"]), Some("T3" | "T4")) {
        issues.push(format!(
            "{scope} authority requires producer trust tier T3 or T4"
        ));
    }
}
