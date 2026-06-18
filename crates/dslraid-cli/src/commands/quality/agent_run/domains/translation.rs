mod approval;
mod conformance;
mod evidence;
mod loss;
mod ontology;

use super::fields::{field_is, field_text, items};
use serde_json::Value;

pub(super) fn push_issues(value: &Value, issues: &mut Vec<String>) {
    let evidence_ids = evidence::ids(value);
    for translation in items(value, "translations") {
        approval::push_issues(value, translation, issues);
        ontology::push_issues(value, translation, issues);
        push_lossy_issue(translation, issues);
        conformance::push_issues(translation, translation_id(translation), issues);
        evidence::push_unknown(
            "translation",
            translation_id(translation),
            evidence::refs(translation),
            &evidence_ids,
            issues,
        );
        for item in loss::items(translation) {
            loss::push_issues(translation_id(translation), item, &evidence_ids, issues);
        }
    }
}

fn push_lossy_issue(translation: &Value, issues: &mut Vec<String>) {
    if field_is(translation, "status", "lossy") && loss::items(translation).next().is_none() {
        issues.push(format!(
            "lossy translation {} requires loss ledger",
            translation_id(translation)
        ));
    }
}

fn translation_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
