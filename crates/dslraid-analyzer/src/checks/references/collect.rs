use dslraid_core::CoreIr;
use serde_json::Value;

use super::{capability_refs, command_refs, context_refs, policy_refs, requirement_refs};

pub(super) fn missing_refs(ir: &CoreIr) -> Vec<Value> {
    let subjects = ir.semantic_subjects();
    let mut missing = Vec::new();
    context_refs::collect(ir, &subjects, &mut missing);
    requirement_refs::collect(ir, &subjects, &mut missing);
    capability_refs::collect(ir, &subjects, &mut missing);
    policy_refs::collect(ir, &subjects, &mut missing);
    command_refs::collect(ir, &subjects, &mut missing);
    missing
}
