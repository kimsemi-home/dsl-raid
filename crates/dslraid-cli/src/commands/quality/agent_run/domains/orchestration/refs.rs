mod catalog;
mod receipt;

use catalog::RefCatalog;
use serde_json::Value;

pub(super) fn push_issues(value: &Value, item: &Value, issues: &mut Vec<String>) {
    let known = RefCatalog::from_manifest(value);
    receipt::push_refs(
        item,
        "selected_reviewers",
        known.reviewers(),
        "reviewer",
        issues,
    );
    receipt::push_refs(
        item,
        "input_evidence",
        known.evidence(),
        "input evidence",
        issues,
    );
    receipt::push_refs(
        item,
        "output_artifacts",
        known.artifacts(),
        "output artifact",
        issues,
    );
}
