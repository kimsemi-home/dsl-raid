use super::tuple::tuple_members;
use dslraid_core::Fsm;
use serde_json::Value;

pub(super) fn tuple_matches_focus(
    fsms: &[&Fsm],
    tuple: &[String],
    subject: &str,
    focus_depth: usize,
    depth: usize,
) -> bool {
    depth <= focus_depth
        && tuple_members(fsms, tuple)
            .iter()
            .any(|member| member == subject)
}

pub(super) fn transition_matches_focus(edge: &Value, subject: &str) -> bool {
    edge.get("members")
        .and_then(Value::as_array)
        .is_some_and(|members| {
            members
                .iter()
                .any(|member| member.as_str() == Some(subject))
        })
        || edge.get("from").and_then(Value::as_str) == Some(subject)
        || edge.get("to").and_then(Value::as_str) == Some(subject)
}
