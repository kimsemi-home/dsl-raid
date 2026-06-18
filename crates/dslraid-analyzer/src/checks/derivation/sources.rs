use std::collections::BTreeSet;

use dslraid_core::CoreIr;

pub(super) fn ids(ir: &CoreIr) -> BTreeSet<String> {
    ir.fsms
        .iter()
        .map(|fsm| fsm.id.clone())
        .chain(
            ir.compositions
                .iter()
                .map(|composition| composition.id.clone()),
        )
        .chain(ir.contexts.iter().map(|context| context.id.clone()))
        .collect()
}
