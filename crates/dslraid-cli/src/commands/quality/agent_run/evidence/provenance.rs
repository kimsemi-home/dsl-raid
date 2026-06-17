use crate::commands::quality::agent_run::fields::{field_text, items, text};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    for evidence in items(value, "evidence") {
        push_required(evidence, "kind", issues);
        push_required(evidence, "observed_by", issues);
        push_required(evidence, "observed_at", issues);
        push_kind_issue(evidence, issues);
    }
}

fn push_required(evidence: &Value, field: &str, issues: &mut Vec<String>) {
    if text(evidence, &["provenance", field]).is_none() {
        issues.push(format!(
            "evidence {} requires provenance {field}",
            id(evidence)
        ));
    }
}

fn push_kind_issue(evidence: &Value, issues: &mut Vec<String>) {
    let Some(kind) = text(evidence, &["provenance", "kind"]) else {
        return;
    };
    if !matches!(
        kind,
        "sidecar-assessment" | "runtime-trace" | "generated" | "external" | "human-annotation"
    ) {
        issues.push(format!(
            "evidence {} has unsupported provenance kind {kind}",
            id(evidence)
        ));
    }
}

fn id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
