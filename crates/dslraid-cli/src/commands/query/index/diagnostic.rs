use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(in crate::commands::query::index) fn push_diagnostic_items(
    items: &mut Vec<Value>,
    ir: &CoreIr,
    marks: &DerivationMarks,
) {
    let tags: &[String] = &[];
    for diagnostic in &ir.diagnostics {
        push_query_item(
            items,
            "diagnostic",
            &diagnostic.id,
            &diagnostic.id,
            &diagnostic.message,
            tags,
            Some(&diagnostic.severity),
            None,
            marks,
            Some(serde_json::json!({
                "code": diagnostic.code,
                "severity": diagnostic.severity,
                "subjects": diagnostic.subjects
            })),
        );
    }
}
