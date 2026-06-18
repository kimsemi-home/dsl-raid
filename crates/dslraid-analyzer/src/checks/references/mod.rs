mod capability_refs;
mod collect;
mod command_refs;
mod context_refs;
mod message;
mod missing;
mod policy_refs;
mod record;
mod requirement_refs;

use dslraid_core::CoreIr;

use crate::builder::ReportBuilder;

pub(crate) fn check(ir: &CoreIr, builder: &mut ReportBuilder) {
    let missing = collect::missing_refs(ir);
    builder.record(record::assertion(missing));
}
