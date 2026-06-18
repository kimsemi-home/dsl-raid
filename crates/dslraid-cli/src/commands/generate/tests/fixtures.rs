use std::fs;
use std::path::{Path, PathBuf};

use super::temp::temp_path;

pub(super) fn runscope_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("examples/runscope/runscope.raid.json")
}

pub(super) fn isolated_runscope_fixture() -> PathBuf {
    let path = temp_path("runscope-input");
    fs::copy(runscope_fixture(), &path).unwrap();
    path
}
