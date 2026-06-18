use dslraid_core::CoreIr;
use serde_json::Value;
use std::collections::BTreeSet;

use super::missing;

pub(super) fn collect(ir: &CoreIr, subjects: &BTreeSet<String>, missing: &mut Vec<Value>) {
    for capability in &ir.capabilities {
        for subject in capability.provides.iter().chain(capability.requires.iter()) {
            missing::push_if_missing(subjects, missing, &capability.id, subject);
        }
    }
}
