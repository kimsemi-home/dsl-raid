use anyhow::Result;
use dslraid_core::{event_subject, state_subject, transition_subject, Fsm, Transition};
use serde_json::Value;

pub(super) fn tuple_state_value(
    composition_id: &str,
    fsms: &[&Fsm],
    tuple: &[String],
) -> Result<Value> {
    let members = tuple_members(fsms, tuple);
    let initial = fsms.iter().zip(tuple.iter()).all(|(fsm, state_id)| {
        fsm.states
            .iter()
            .any(|state| state.id == *state_id && state.initial)
    });
    let terminal = fsms.iter().zip(tuple.iter()).all(|(fsm, state_id)| {
        fsm.states
            .iter()
            .any(|state| state.id == *state_id && state.terminal)
    });
    Ok(serde_json::json!({
        "id": tuple_subject(composition_id, &members),
        "members": members,
        "initial": initial,
        "terminal": terminal
    }))
}

pub(super) fn tuple_transition_value(
    composition_id: &str,
    fsms: &[&Fsm],
    from_tuple: &[String],
    to_tuple: &[String],
    fsm_id: &str,
    transition: &Transition,
) -> Result<Value> {
    let from_members = tuple_members(fsms, from_tuple);
    let to_members = tuple_members(fsms, to_tuple);
    Ok(serde_json::json!({
        "id": format!("tuple_transition:{}:{}", composition_id.trim_start_matches("composition:"), transition.id),
        "from": tuple_subject(composition_id, &from_members),
        "to": tuple_subject(composition_id, &to_members),
        "members": [transition_subject(fsm_id, &transition.id)],
        "event": transition.on.as_ref().map(|event| event_subject(fsm_id, event))
    }))
}

pub(super) fn tuple_members(fsms: &[&Fsm], tuple: &[String]) -> Vec<String> {
    fsms.iter()
        .zip(tuple.iter())
        .map(|(fsm, state)| state_subject(&fsm.id, state))
        .collect()
}

pub(super) fn tuple_subject(composition_id: &str, members: &[String]) -> String {
    format!(
        "state_tuple:{}.{}",
        composition_id.trim_start_matches("composition:"),
        members
            .iter()
            .map(|member| member.replace([':', '.'], "_"))
            .collect::<Vec<_>>()
            .join("__")
    )
}

pub(super) fn tuple_key(tuple: &[String]) -> String {
    tuple.join("\u{1f}")
}
