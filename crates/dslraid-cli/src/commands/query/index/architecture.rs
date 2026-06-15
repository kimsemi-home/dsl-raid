use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(in crate::commands::query::index) fn push_architecture_items(
    items: &mut Vec<Value>,
    ir: &CoreIr,
    marks: &DerivationMarks,
) {
    for composition in &ir.compositions {
        push_query_item(
            items,
            "composition",
            &composition.id,
            &composition.id,
            &composition.name,
            &composition.tags,
            composition.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "composition_kind": composition.kind,
                "inputs": composition.inputs
            })),
        );
    }
    for projection in &ir.projections {
        push_query_item(
            items,
            "projection",
            &projection.id,
            &projection.id,
            &projection.id,
            &projection.tags,
            projection.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "source": projection.source,
                "show": projection.show
            })),
        );
    }
}
