use dslraid_core::load_core_ir;
use std::path::{Path, PathBuf};

use super::generate_code;
use crate::CodegenTarget;

#[test]
fn rust_backend_is_generated_from_canonical_ir() {
    let ir = load_core_ir(repo_path("examples/runscope/runscope.raid.json")).unwrap();
    let code = generate_code(&ir, CodegenTarget::Rust).unwrap();

    assert!(code.starts_with("// Generated from DSLRaid Canonical IR"));
    assert!(code.contains("pub enum RuntimeFSMState"));
    assert!(code.contains("pub fn runtime_transition"));
    assert!(code.contains("RuntimeFSMState::Running"));
}

#[test]
fn rust_backend_accepts_lisp_emitted_canonical_ir() {
    let ir = load_core_ir(repo_path("examples/runscope/runscope.lisp.raid.json")).unwrap();
    let code = generate_code(&ir, CodegenTarget::Rust).unwrap();

    assert!(code.starts_with("// Generated from DSLRaid Canonical IR"));
    assert!(code.contains("pub enum RuntimeFSMState"));
    assert!(code.contains("RuntimeFSMState::Completed"));
}

#[test]
fn lisp_emitted_ir_declares_rust_as_generated_artifact() {
    let ir = load_core_ir(repo_path("examples/runscope/runscope.lisp.raid.json")).unwrap();
    let artifact = ir.artifact_by_id("artifact:runtime_fsm.rs").unwrap();
    let derivation = ir
        .derivation_by_id("derivation:lisp_runtime_codegen")
        .unwrap();

    assert_eq!(artifact.kind, "generated");
    assert_eq!(
        artifact.generated_by.as_deref(),
        Some(derivation.id.as_str())
    );
    assert_eq!(derivation.source, "context:runtime");
    assert!(derivation
        .targets
        .iter()
        .any(|target| target.artifact == artifact.id && target.role == "generated"));
}

#[test]
fn generated_backends_share_canonical_ir_header() {
    let ir = load_core_ir(repo_path("examples/runscope/runscope.raid.json")).unwrap();
    let go = generate_code(&ir, CodegenTarget::Go).unwrap();
    let ts = generate_code(&ir, CodegenTarget::TypeScript).unwrap();

    assert!(go.starts_with("// Generated from DSLRaid Canonical IR"));
    assert!(ts.starts_with("// Generated from DSLRaid Canonical IR"));
}

fn repo_path(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
}
