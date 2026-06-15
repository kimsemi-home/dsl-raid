use super::fsm_behavior::push_behavior_items;
use super::fsm_states::push_state_event_items;
use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(in crate::commands::query::index) fn push_fsm_items(
    items: &mut Vec<Value>,
    ir: &CoreIr,
    marks: &DerivationMarks,
) {
    for fsm in &ir.fsms {
        push_query_item(
            items,
            "fsm",
            &fsm.id,
            &fsm.id,
            &fsm.name,
            &fsm.tags,
            fsm.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "context": fsm.context,
                "states": fsm.states.len(),
                "transitions": fsm.transitions.len()
            })),
        );
        push_state_event_items(items, fsm, marks);
        push_behavior_items(items, fsm, marks);
    }
}
