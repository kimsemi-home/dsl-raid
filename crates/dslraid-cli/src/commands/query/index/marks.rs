use dslraid_core::CoreIr;
use std::collections::BTreeSet;

pub(in crate::commands::query::index) struct DerivationMarks {
    pub(in crate::commands::query::index) tested: BTreeSet<String>,
    pub(in crate::commands::query::index) generated: BTreeSet<String>,
}

impl DerivationMarks {
    pub(in crate::commands::query::index) fn from_ir(ir: &CoreIr) -> Self {
        let mut tested = BTreeSet::new();
        let mut generated = BTreeSet::new();
        for derivation in &ir.derivations {
            for target in &derivation.targets {
                if target.role == "test" {
                    tested.insert(derivation.source.clone());
                }
                if target.role == "generated" {
                    generated.insert(derivation.source.clone());
                    generated.insert(target.artifact.clone());
                }
            }
        }
        Self { tested, generated }
    }
}
