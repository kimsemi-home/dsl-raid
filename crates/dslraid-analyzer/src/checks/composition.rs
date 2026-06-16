use std::collections::BTreeSet;

use dslraid_core::CoreIr;
use serde_json::{json, Value};

use crate::builder::ReportBuilder;
use crate::checks::composition_record;

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let bad_inputs = invalid_inputs(ir);
    let missing_policy = missing_policy(ir);
    composition_record::inputs(ir, builder, bad_inputs);
    composition_record::policy(ir, builder, missing_policy);
}

fn invalid_inputs(ir: &CoreIr) -> Vec<Value> {
    let fsm_ids: BTreeSet<_> = ir.fsms.iter().map(|fsm| fsm.id.as_str()).collect();
    let mut bad_inputs = Vec::new();
    for composition in &ir.compositions {
        for input in &composition.inputs {
            if !fsm_ids.contains(input.as_str()) {
                bad_inputs.push(json!({ "composition": composition.id, "input": input }));
            }
        }
    }
    bad_inputs
}

fn missing_policy(ir: &CoreIr) -> Vec<Value> {
    ir.compositions
        .iter()
        .filter(|composition| composition.conflict_policy.is_none())
        .map(|composition| json!({ "composition": composition.id }))
        .collect()
}
