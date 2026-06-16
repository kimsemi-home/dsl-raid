mod fixture;

use crate::{validate_core_ir, ValidateOptions};

#[test]
fn missing_initial_is_blocking() {
    let report = validate_core_ir(&fixture::missing_initial_ir(), ValidateOptions::default());
    assert!(report.has_blocking_errors());
}
