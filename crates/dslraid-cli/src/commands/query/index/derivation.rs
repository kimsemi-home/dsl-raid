use super::item::push_query_item;
use super::marks::DerivationMarks;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(in crate::commands::query::index) fn push_derivation_items(
    items: &mut Vec<Value>,
    ir: &CoreIr,
    marks: &DerivationMarks,
) {
    for derivation in &ir.derivations {
        push_query_item(
            items,
            "derivation",
            &derivation.id,
            &derivation.id,
            &derivation.id,
            &derivation.tags,
            derivation.visibility.as_deref(),
            None,
            marks,
            Some(serde_json::json!({
                "source": derivation.source,
                "rule": derivation.rule.id,
                "targets": derivation.targets.iter().map(|target| &target.artifact).collect::<Vec<_>>()
            })),
        );
    }
    for artifact in &ir.artifacts {
        push_query_item(
            items,
            "artifact",
            &artifact.id,
            &artifact.id,
            &artifact.path,
            &artifact.tags,
            artifact.visibility.as_deref(),
            Some(&artifact.path),
            marks,
            Some(serde_json::json!({
                "artifact_kind": artifact.kind,
                "generated_by": artifact.generated_by
            })),
        );
    }
}
