use crate::commands::quality::agent_run::fields::field_is;
use serde_json::Value;

pub(super) fn push_issues(translation: &Value, id: &str, issues: &mut Vec<String>) {
    if !field_is(translation, "conformance", "source") {
        return;
    }
    push_lossy_issue(translation, id, issues);
    push_round_trip_issue(translation, id, issues);
}

fn push_lossy_issue(translation: &Value, id: &str, issues: &mut Vec<String>) {
    if field_is(translation, "status", "lossy") {
        issues.push(format!(
            "lossy translation {id} cannot claim source conformance"
        ));
    }
}

fn push_round_trip_issue(translation: &Value, id: &str, issues: &mut Vec<String>) {
    if translation.get("round_trip").and_then(Value::as_bool) == Some(false) {
        issues.push(format!(
            "non-round-trip translation {id} cannot claim source conformance"
        ));
    }
}
