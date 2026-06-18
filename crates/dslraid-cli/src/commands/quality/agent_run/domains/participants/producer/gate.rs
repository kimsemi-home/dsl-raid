use super::profile::ProducerProfile;

pub(super) fn push_issues(producer: &ProducerProfile, issues: &mut Vec<String>) {
    push_required_issues(producer, issues);
    push_cold_start_issue(producer, issues);
    push_automatic_issue(producer, issues);
    push_high_risk_issue(producer, issues);
}

fn push_required_issues(producer: &ProducerProfile, issues: &mut Vec<String>) {
    if producer.reasoning().is_none() {
        issues.push("approved run requires producer reasoning level".to_string());
    }
    if producer.trust().is_none() {
        issues.push("approved run requires producer trust tier".to_string());
    }
}

fn push_cold_start_issue(producer: &ProducerProfile, issues: &mut Vec<String>) {
    if producer.is_cold_start() {
        issues.push(format!(
            "approved run cannot use cold-start producer {}",
            producer.id()
        ));
    }
}

fn push_automatic_issue(producer: &ProducerProfile, issues: &mut Vec<String>) {
    if producer.is_automatic_gate() && !producer.is_trusted() {
        issues.push("automatic authority requires trusted producer T3 or T4".to_string());
    }
}

fn push_high_risk_issue(producer: &ProducerProfile, issues: &mut Vec<String>) {
    if producer.is_high_risk_scope() && !producer.is_high_reasoning() {
        issues.push("high-risk authority requires producer reasoning level R3 or R4".to_string());
    }
}
