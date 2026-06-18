use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(in crate::commands::query::index) fn push_capability_items(
    items: &mut Vec<Value>,
    ir: &CoreIr,
    marks: &DerivationMarks,
) {
    for capability in &ir.capabilities {
        push_query_item(
            items,
            "capability",
            &capability.id,
            &capability.id,
            &capability.name,
            &capability.tags,
            capability.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "capability_kind": capability.kind,
                "owner": capability.owner
            })),
        );
    }
}
