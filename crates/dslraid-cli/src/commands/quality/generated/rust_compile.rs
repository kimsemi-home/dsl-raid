use anyhow::{bail, Context, Result};
use dslraid_codegen::{generate_code, CodegenTarget};
use dslraid_core::CoreIr;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub(super) fn check(ir: &CoreIr) -> Result<()> {
    let source = temp_path("rs");
    let artifact = temp_path("rlib");
    fs::write(&source, generate_code(ir, CodegenTarget::Rust)?)
        .with_context(|| format!("write {}", source.display()))?;
    let output = rustc(&source, &artifact);
    cleanup(&source, &artifact);
    let output = output?;
    if output.status.success() {
        Ok(())
    } else {
        bail!(
            "generated Rust backend failed to compile:\n{}",
            String::from_utf8_lossy(&output.stderr)
        )
    }
}

fn rustc(source: &PathBuf, artifact: &PathBuf) -> Result<std::process::Output> {
    Command::new("rustc")
        .arg("--edition=2021")
        .arg("--crate-type=lib")
        .arg(source)
        .arg("-o")
        .arg(artifact)
        .output()
        .context("run rustc for generated Rust backend")
}

fn temp_path(ext: &str) -> PathBuf {
    let name = format!("dslraid-generated-rust-{}.{}", std::process::id(), ext);
    std::env::temp_dir().join(name)
}

fn cleanup(source: &PathBuf, artifact: &PathBuf) {
    let _ = fs::remove_file(source);
    let _ = fs::remove_file(artifact);
}
