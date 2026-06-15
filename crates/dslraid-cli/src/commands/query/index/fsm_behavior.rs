use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::{event_subject, state_subject, transition_subject, Fsm};
use serde_json::Value;

pub(in crate::commands::query::index) fn push_behavior_items(
    items: &mut Vec<Value>,
    fsm: &Fsm,
    marks: &DerivationMarks,
) {
    push_guard_action_items(items, fsm, marks);
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

fn push_guard_action_items(items: &mut Vec<Value>, fsm: &Fsm, marks: &DerivationMarks) {
    for guard in &fsm.guards {
        let subject = format!("guard:{}.{}", fsm.local_name(), guard.id);
        push_query_item(
            items,
            "guard",
            &subject,
            &guard.id,
            guard.name.as_deref().unwrap_or(&guard.id),
            &guard.tags,
            guard.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "fsm": fsm.id,
                "capability": guard.capability
            })),
        );
    }
    for action in &fsm.actions {
        let subject = format!("action:{}.{}", fsm.local_name(), action.id);
        push_query_item(
            items,
            "action",
            &subject,
            &action.id,
            action.name.as_deref().unwrap_or(&action.id),
            &action.tags,
            action.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "fsm": fsm.id,
                "capability": action.capability,
                "depends_on": action.depends_on
            })),
        );
    }
}
