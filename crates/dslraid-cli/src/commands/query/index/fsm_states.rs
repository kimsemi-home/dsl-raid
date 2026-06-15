use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::{event_subject, state_subject, Fsm};
use serde_json::Value;

pub(in crate::commands::query::index) fn push_state_event_items(
    items: &mut Vec<Value>,
    fsm: &Fsm,
    marks: &DerivationMarks,
) {
    for state in &fsm.states {
        let subject = state_subject(&fsm.id, &state.id);
        push_query_item(
            items,
            "state",
            &subject,
            &state.id,
            &state.id,
            &state.tags,
            state.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "fsm": fsm.id,
                "state_kind": state.kind,
                "initial": state.initial,
                "terminal": state.terminal,
                "terminal_semantics": state.terminal_semantics
            })),
        );
    }
    for event in &fsm.events {
        let subject = event_subject(&fsm.id, &event.id);
        push_query_item(
            items,
            "event",
            &subject,
            &event.id,
            event.name.as_deref().unwrap_or(&event.id),
            &event.tags,
            event.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "fsm": fsm.id,
                "event_kind": event.kind
            })),
        );
    }
}
