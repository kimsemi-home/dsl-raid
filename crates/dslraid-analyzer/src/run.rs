use dslraid_core::CoreIr;

use crate::builder::ReportBuilder;
use crate::{checks, ValidateOptions, ValidationReport};

pub fn validate_core_ir(ir: &CoreIr, options: ValidateOptions) -> ValidationReport {
    let mut builder = ReportBuilder::new(options);
    checks::run(ir, &mut builder);
    builder.finish()
}
