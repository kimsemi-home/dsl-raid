use super::profile::ProducerProfile;

pub(super) fn push_issues(producer: &ProducerProfile, issues: &mut Vec<String>) {
    let Some(scope) = producer.scope() else {
        return;
    };
    if !producer.is_sensitive_scope() {
        return;
    }
    if !producer.is_trusted() {
        issues.push(format!(
            "{scope} authority requires producer trust tier T3 or T4"
        ));
    }
}
