use std::collections::BTreeSet;

use dslraid_core::CoreIr;

pub(super) fn ids(ir: &CoreIr) -> BTreeSet<String> {
    ir.artifacts
        .iter()
        .map(|artifact| artifact.id.clone())
        .collect()
}
