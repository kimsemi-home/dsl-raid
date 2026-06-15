use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(in crate::commands::query::index) fn push_policies_and_commands(
    items: &mut Vec<Value>,
    ir: &CoreIr,
    marks: &DerivationMarks,
) {
    for policy in &ir.policies {
        push_query_item(
            items,
            "policy",
            &policy.id,
            &policy.id,
            &policy.name,
            &policy.tags,
            policy.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "policy_kind": policy.kind,
                "applies_to": policy.applies_to
            })),
        );
    }
    for command in &ir.commands {
        push_query_item(
            items,
            "command",
            &command.id,
            &command.id,
            &command.name,
            &command.tags,
            command.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({ "capability": command.capability })),
        );
    }
}
