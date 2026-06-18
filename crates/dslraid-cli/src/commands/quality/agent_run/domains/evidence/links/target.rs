use super::id;
use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;
use std::collections::HashSet;

pub(super) fn push_issue(
    ids: &HashSet<&str>,
    evidence: &Value,
    link: &Value,
    issues: &mut Vec<String>,
) {
    let Some(target) = field_text(link, "target") else {
        issues.push(format!("evidence {} link requires target", id(evidence)));
        return;
    };
    if target == id(evidence) {
        issues.push(format!(
            "evidence {} link cannot target itself",
            id(evidence)
        ));
    } else if !ids.contains(target) {
        issues.push(format!(
            "evidence {} link target {target} is not evidence",
            id(evidence)
        ));
    }
}
