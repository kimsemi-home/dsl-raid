use super::capability::push_capability_items;
use super::item::{push_basic_item, push_query_item};
use super::marks::DerivationMarks;
use super::policy_command::push_policies_and_commands;
use dslraid_core::CoreIr;
use serde_json::Value;

pub(in crate::commands::query::index) fn push_root_items(
    items: &mut Vec<Value>,
    ir: &CoreIr,
    marks: &DerivationMarks,
) {
    push_query_item(
        items,
        "project",
        &format!("project:{}", ir.project.id),
        &ir.project.id,
        &ir.project.name,
        &ir.project.tags,
        ir.project.visibility.as_deref(),
        None,
        marks,
        None,
    );
    for context in &ir.contexts {
        push_basic_item(
            items,
            "context",
            &context.id,
            &context.name,
            &context.tags,
            context.visibility.as_deref(),
            marks,
        );
    }
    for requirement in &ir.requirements {
        push_basic_item(
            items,
            "requirement",
            &requirement.id,
            &requirement.name,
            &requirement.tags,
            requirement.visibility.as_deref(),
            marks,
        );
    }
    push_capability_items(items, ir, marks);
    push_policies_and_commands(items, ir, marks);
}
