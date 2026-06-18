use super::id;
use crate::commands::quality::agent_run::fields::field_text;
use serde_json::Value;

pub(super) fn push_issue(evidence: &Value, link: &Value, issues: &mut Vec<String>) {
    let Some(relation) = field_text(link, "relation") else {
        issues.push(format!("evidence {} link requires relation", id(evidence)));
        return;
    };
    if !is_supported(relation) {
        issues.push(format!(
            "evidence {} has unsupported link relation {relation}",
            id(evidence)
        ));
    }
}

fn is_supported(relation: &str) -> bool {
    matches!(
        relation,
        "corroborates" | "derived_from" | "reproduces" | "observed_with" | "supersedes"
    )
}
