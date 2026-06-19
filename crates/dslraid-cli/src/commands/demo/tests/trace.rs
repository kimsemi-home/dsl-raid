#[test]
fn package_trace_writes_coverage_overlay() {
    let out = super::support::temp_dir();
    super::super::package::run(
        &super::support::repo_path("examples/runscope/runscope.raid.json"),
        &out,
        Some(&super::support::repo_path(
            "examples/runscope/run-001.trace.json",
        )),
    )
    .unwrap();

    let coverage = super::support::read_json(&out.join("run-001.coverage.json"));
    assert_eq!(coverage["coverage_version"].as_str(), Some("0.1.0"));
    assert!(out.join("run-001.trace.json").exists());
    super::support::remove_dir(out);
}
