#[test]
fn package_writes_viewer_demo_assets() {
    let out = super::support::temp_dir();
    super::super::package::run(
        &super::support::repo_path("examples/runscope/runscope.raid.json"),
        &out,
        None,
    )
    .unwrap();

    assert!(out.join("runscope.raid.json").exists());
    assert!(out.join("runscope.raid.view.json").exists());
    assert!(out.join("runscope.raid.svg").exists());
    assert!(out.join("runscope.sourcemap.json").exists());
    super::support::remove_dir(out);
}
