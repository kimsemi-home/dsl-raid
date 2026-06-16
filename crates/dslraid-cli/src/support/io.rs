use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_or_stdout(out: Option<&Path>, bytes: &[u8]) -> Result<()> {
    match out {
        Some(path) => write_bytes(path, bytes),
        None => {
            std::io::stdout().write_all(bytes)?;
            println!();
            Ok(())
        }
    }
}

pub(crate) fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}
