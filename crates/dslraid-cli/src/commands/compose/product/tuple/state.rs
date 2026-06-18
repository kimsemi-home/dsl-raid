use super::members::tuple_members;
use super::subject::tuple_subject;
use anyhow::Result;
use dslraid_core::Fsm;
use serde_json::Value;

pub(in crate::commands::compose) fn tuple_state_value(
    composition_id: &str,
    fsms: &[&Fsm],
    tuple: &[String],
) -> Result<Value> {
    let members = tuple_members(fsms, tuple);
    Ok(serde_json::json!({
        "id": tuple_subject(composition_id, &members),
        "members": members,
        "initial": tuple_is_initial(fsms, tuple),
        "terminal": tuple_is_terminal(fsms, tuple)
    }))
}

fn tuple_is_initial(fsms: &[&Fsm], tuple: &[String]) -> bool {
    fsms.iter().zip(tuple.iter()).all(|(fsm, state_id)| {
        fsm.states
            .iter()
            .any(|state| state.id == *state_id && state.initial)
    })
}

fn tuple_is_terminal(fsms: &[&Fsm], tuple: &[String]) -> bool {
    fsms.iter().zip(tuple.iter()).all(|(fsm, state_id)| {
        fsm.states
            .iter()
            .any(|state| state.id == *state_id && state.terminal)
    })
}
