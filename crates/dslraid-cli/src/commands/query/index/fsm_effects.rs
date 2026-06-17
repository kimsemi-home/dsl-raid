use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::{action_subject, guard_subject, Fsm};
use serde_json::Value;

pub(in crate::commands::query::index) fn push_effect_items(
    items: &mut Vec<Value>,
    fsm: &Fsm,
    marks: &DerivationMarks,
) {
    for guard in &fsm.guards {
        push_query_item(
            items,
            "guard",
            &guard_subject(&fsm.id, &guard.id),
            &guard.id,
            guard.name.as_deref().unwrap_or(&guard.id),
            &guard.tags,
            guard.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "fsm": fsm.id,
                "guard_kind": guard.kind,
                "capability": guard.capability,
                "expression": guard.expression,
                "input": guard.input
            })),
        );
    }
    for action in &fsm.actions {
        push_query_item(
            items,
            "action",
            &action_subject(&fsm.id, &action.id),
            &action.id,
            action.name.as_deref().unwrap_or(&action.id),
            &action.tags,
            action.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "fsm": fsm.id,
                "action_kind": action.kind,
                "capability": action.capability,
                "command": action.command,
                "emits": action.emits,
                "expression": action.expression,
                "depends_on": action.depends_on
            })),
        );
    }
}
