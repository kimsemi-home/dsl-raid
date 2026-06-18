use super::{event_record, guard_action_record, requires_record, state_record};
use crate::builder::ReportBuilder;

pub(super) fn from_state(builder: &mut ReportBuilder, failures: &[serde_json::Value]) {
    state_record::from_state(builder, failures);
}

pub(super) fn to_state(builder: &mut ReportBuilder, failures: &[serde_json::Value]) {
    state_record::to_state(builder, failures);
}

pub(super) fn event(builder: &mut ReportBuilder, failures: &[serde_json::Value]) {
    event_record::record(builder, failures);
}

pub(super) fn guard_action(builder: &mut ReportBuilder, failures: &[serde_json::Value]) {
    guard_action_record::record(builder, failures);
}

pub(super) fn requires(builder: &mut ReportBuilder, failures: &[serde_json::Value]) {
    requires_record::record(builder, failures);
}
