mod approval;
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
        push_conformance_issue(translation, issues);
        evidence::push_unknown(
            "translation",
            translation_id(translation),
            evidence::refs(translation),
            &evidence_ids,
            issues,
        );
        for item in loss::items(translation) {
            push_forbidden_issue(translation, item, issues);
            evidence::push_unknown(
                "loss",
                loss::id(item),
                evidence::refs(item),
                &evidence_ids,
                issues,
            );
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

fn push_conformance_issue(translation: &Value, issues: &mut Vec<String>) {
    if !field_is(translation, "conformance", "source") {
        return;
    }
    if field_is(translation, "status", "lossy") {
        issues.push(format!(
            "lossy translation {} cannot claim source conformance",
            translation_id(translation)
        ));
    }
    if translation.get("round_trip").and_then(Value::as_bool) == Some(false) {
        issues.push(format!(
            "non-round-trip translation {} cannot claim source conformance",
            translation_id(translation)
        ));
    }
}

fn push_forbidden_issue(translation: &Value, loss: &Value, issues: &mut Vec<String>) {
    if loss::is_forbidden(loss) {
        issues.push(format!(
            "translation {} contains forbidden loss {}",
            translation_id(translation),
            loss::id(loss)
        ));
    }
}

fn translation_id(value: &Value) -> &str {
    field_text(value, "id").unwrap_or("<unknown>")
}
