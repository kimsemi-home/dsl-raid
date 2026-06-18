mod architecture;
mod capability;
mod derivation;
mod diagnostic;
mod fsm;
mod fsm_behavior;
mod fsm_effects;
mod fsm_states;
mod item;
mod marks;
mod policy_command;
mod root;

use dslraid_core::CoreIr;
use marks::DerivationMarks;
use serde_json::Value;

pub(super) fn build_query_items(ir: &CoreIr) -> Vec<Value> {
    let marks = DerivationMarks::from_ir(ir);
    let mut items = Vec::new();
    root::push_root_items(&mut items, ir, &marks);
    fsm::push_fsm_items(&mut items, ir, &marks);
    architecture::push_architecture_items(&mut items, ir, &marks);
    derivation::push_derivation_items(&mut items, ir, &marks);
    diagnostic::push_diagnostic_items(&mut items, ir, &marks);
    items
}
