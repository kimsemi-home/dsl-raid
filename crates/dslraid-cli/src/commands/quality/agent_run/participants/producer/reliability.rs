use super::super::reliability::has_authority_subject;
use serde_json::Value;

use super::profile::ProducerProfile;

pub(super) fn push_issues(value: &Value, producer: &ProducerProfile, issues: &mut Vec<String>) {
    if !producer.is_trusted() {
        return;
    }
    let Some(id) = producer.id_value() else {
        return;
    };
    if !has_authority_subject(value, id) {
        issues.push(format!(
            "trusted producer {id} requires reliability evidence"
        ));
    }
}
