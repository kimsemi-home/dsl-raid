use anyhow::{bail, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub(super) fn check(input: &Path) -> Result<()> {
    let out = temp_dir();
    crate::commands::demo::run(crate::DemoArgs {
        command: crate::DemoCommand::Package {
            input: input.to_path_buf(),
            out: out.clone(),
            trace: Some(Path::new("examples/runscope/run-001.trace.json").to_path_buf()),
        },
    })?;
    require(&out, "runscope.raid.json")?;
    require(&out, "runscope.sourcemap.json")?;
    require(&out, "run-001.coverage.json")?;
    fs::remove_dir_all(out).ok();
    Ok(())
}

fn require(out: &Path, file: &str) -> Result<()> {
    if out.join(file).exists() {
        Ok(())
    } else {
        bail!("demo package did not write {file}")
    }
}

fn temp_dir() -> PathBuf {
    std::env::temp_dir().join(format!("dslraid-demo-quality-{}", std::process::id()))
}
