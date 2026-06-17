use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::{event_subject, state_subject, transition_subject, Fsm};
use serde_json::Value;

pub(in crate::commands::query::index) fn push_behavior_items(
    items: &mut Vec<Value>,
    fsm: &Fsm,
    marks: &DerivationMarks,
) {
    for transition in &fsm.transitions {
        let subject = transition_subject(&fsm.id, &transition.id);
        push_query_item(
            items,
            "transition",
            &subject,
            &transition.id,
            &transition.id,
            &transition.tags,
            transition.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "fsm": fsm.id,
                "from": state_subject(&fsm.id, &transition.from),
                "to": state_subject(&fsm.id, &transition.to),
                "on": transition.on.as_ref().map(|event| event_subject(&fsm.id, event)),
                "guards": transition.guards,
                "actions": transition.actions,
                "requires": transition.requires
            })),
        );
    }
}
