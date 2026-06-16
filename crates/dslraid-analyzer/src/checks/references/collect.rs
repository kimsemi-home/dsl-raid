use dslraid_core::CoreIr;
use serde_json::{json, Value};
use std::collections::BTreeSet;

pub(super) fn missing_refs(ir: &CoreIr) -> Vec<Value> {
    let subjects = ir.semantic_subjects();
    let mut missing = Vec::new();
    collect_contexts(ir, &subjects, &mut missing);
    collect_requirements(ir, &subjects, &mut missing);
    collect_capabilities(ir, &subjects, &mut missing);
    collect_policies(ir, &subjects, &mut missing);
    collect_commands(ir, &subjects, &mut missing);
    missing
}

fn collect_contexts(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for context in &ir.contexts {
        for owned in &context.owns {
            push_if_missing(subjects, missing, &context.id, owned);
        }
    }
}

fn collect_requirements(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for requirement in &ir.requirements {
        for subject in &requirement.satisfied_by {
            push_if_missing(subjects, missing, &requirement.id, subject);
        }
    }
}

fn collect_capabilities(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for capability in &ir.capabilities {
        for subject in capability.provides.iter().chain(capability.requires.iter()) {
            push_if_missing(subjects, missing, &capability.id, subject);
        }
    }
}

fn collect_policies(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for policy in &ir.policies {
        for subject in &policy.applies_to {
            push_if_missing(subjects, missing, &policy.id, subject);
        }
    }
}

fn collect_commands(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for command in &ir.commands {
        if let Some(capability) = &command.capability {
            push_if_missing(subjects, missing, &command.id, capability);
        }
    }
}

fn push_if_missing(
    subjects: &BTreeSet<String>,
    missing: &mut Vec<Value>,
    source: &str,
    reference: &str,
) {
    if !subjects.contains(reference) {
        missing.push(json!({ "source": source, "reference": reference }));
    }
}
