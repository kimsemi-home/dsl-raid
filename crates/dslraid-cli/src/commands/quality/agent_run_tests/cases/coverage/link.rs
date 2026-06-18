use super::fixture::{issues, temp_root, write_coverage, write_trace};

#[test]
fn approved_manifest_rejects_coverage_design_mismatch() {
    let root = temp_root("design");
    write_trace(&root, "trace.json");
    write_coverage(&root, "coverage.json", "other.raid.json", "trace.json");

    assert_eq!(
        issues(&root, "coverage.json", "trace.json"),
        vec!["coverage evidence coverage.json design_ir does not match manifest ssot"]
    );
}

#[test]
fn approved_manifest_rejects_coverage_trace_mismatch() {
    let root = temp_root("trace");
    write_trace(&root, "trace.json");
    write_coverage(
        &root,
        "coverage.json",
        "examples/runscope/runscope.raid.json",
        "other.json",
    );

    assert_eq!(
        issues(&root, "coverage.json", "trace.json"),
        vec!["coverage evidence coverage.json must reference trace evidence"]
    );
}
