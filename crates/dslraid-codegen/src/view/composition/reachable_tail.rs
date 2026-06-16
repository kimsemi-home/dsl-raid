use super::model::TupleEdge;
use dslraid_core::{event_subject, Composition, Fsm, Transition};

pub(crate) fn finish_tuple_edge(
    composition: &Composition,
    fsms: &[&Fsm],
    to: &[String],
    fsm: &Fsm,
    transition: &Transition,
    mut edge: TupleEdge,
) -> TupleEdge {
    edge.to = super::tuple::tuple_subject(&composition.id, &super::tuple::tuple_members(fsms, to));
    edge.members = vec![dslraid_core::transition_subject(&fsm.id, &transition.id)];
    edge.event = transition
        .on
        .as_ref()
        .map(|event| event_subject(&fsm.id, event));
    edge
}

pub(crate) fn all_flag(fsms: &[&Fsm], tuple: &[String], initial: bool) -> bool {
    fsms.iter().zip(tuple.iter()).all(|(fsm, state_id)| {
        fsm.states.iter().any(|state| {
            state.id == *state_id
                && if initial {
                    state.initial
                } else {
                    state.terminal
                }
        })
    })
}

pub(crate) fn state_space_limit(composition: &Composition) -> usize {
    composition
        .state_space
        .as_ref()
        .and_then(|value| value.get("max_materialized_states"))
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(48) as usize
}
