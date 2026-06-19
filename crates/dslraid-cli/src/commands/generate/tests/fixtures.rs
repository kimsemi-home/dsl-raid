use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}

pub(super) fn isolated_runscope_fixture() -> PathBuf {
    let root = super::temp::temp_dir("runscope-project");
    let path = root.join("examples/runscope/runscope.raid.json");
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::copy(runscope_fixture(), &path).unwrap();
    seed_generated_targets(&root);
    path
}

pub(super) fn cleanup_isolated_fixture(path: &Path) {
    if let Some(root) = path.ancestors().nth(3) {
        fs::remove_dir_all(root).ok();
    }
}

fn seed_generated_targets(root: &Path) {
    for relative in [
        "generated/runtime_fsm.rs",
        "generated/runtime_fsm.go",
        "examples/runscope/runscope.generated.md",
    ] {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, b"placeholder").unwrap();
    }
}
