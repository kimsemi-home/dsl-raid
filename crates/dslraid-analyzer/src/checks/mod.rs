mod artifact;
mod collection;
mod composition;
mod composition_record;
mod derivation;
mod fsm;
mod projection;
mod references;
mod unique;
mod visibility;

use dslraid_core::CoreIr;

use crate::builder::ReportBuilder;

pub(crate) use collection::{record_collection_check, CollectionCheck};

pub(crate) fn run(ir: &CoreIr, builder: &mut ReportBuilder) {
    unique::check(ir, builder);
    references::check(ir, builder);
    for fsm in &ir.fsms {
        fsm::check(fsm, ir, builder);
    }
    composition::check(ir, builder);
    projection::check(ir, builder);
    derivation::check(ir, builder);
    artifact::check(ir, builder);
    visibility::check(ir, builder);
}
