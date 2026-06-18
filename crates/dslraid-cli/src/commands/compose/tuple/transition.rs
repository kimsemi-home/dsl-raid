use super::members::tuple_members;
use super::subject::tuple_subject;
use crate::commands::compose::id::tuple_transition_id;
use anyhow::Result;
use dslraid_core::{event_subject, transition_subject, Fsm, Transition};
use serde_json::Value;

pub(in crate::commands::compose) fn tuple_transition_value(
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
        "id": tuple_transition_id(composition_id, from_tuple, fsm_id, transition),
        "from": tuple_subject(composition_id, &from_members),
        "to": tuple_subject(composition_id, &to_members),
        "members": [transition_subject(fsm_id, &transition.id)],
        "event": transition.on.as_ref().map(|event| event_subject(fsm_id, event))
    }))
}
