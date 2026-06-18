use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeSet;

use super::missing;

pub(super) fn collect(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for command in &ir.commands {
        if let Some(capability) = &command.capability {
            missing::push_if_missing(subjects, missing, &command.id, capability);
        }
    }
}
