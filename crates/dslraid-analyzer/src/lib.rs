mod builder;
mod checks;
mod model;
mod options;
mod run;
mod subjects;

pub use model::{
    AssertionResult, CountSummary, DiagnosticRef, PropositionResult, Summary, ValidationReport,
    ValidationRun, ValidationSource,
};
pub use options::ValidateOptions;
pub use run::validate_core_ir;

pub const VALIDATION_VERSION: &str = "0.1.0";
pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests;
