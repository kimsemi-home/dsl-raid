use dslraid_core::load_core_ir;
use std::path::{Path, PathBuf};

use super::generate_fsm_catalog_doc;

#[test]
fn fsm_catalog_is_generated_from_canonical_ir() {
    let ir = load_core_ir(repo_path("examples/runscope/runscope.raid.json")).unwrap();
    let catalog = generate_fsm_catalog_doc(&ir);

    assert!(catalog.starts_with("# DSLRaid FSM Catalog"));
    assert!(catalog.contains("| RuntimeFSM | fsm:runtime | 5 | idle | completed, failed | 4 |"));
    assert!(catalog.contains("| WorkspaceFSM | fsm:workspace | 5 | clean | synced, conflict | 4 |"));
}

fn repo_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}
