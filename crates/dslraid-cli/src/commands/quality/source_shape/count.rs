use anyhow::Result;
use std::fs;
use std::path::Path;

pub(super) fn lines(path: &Path) -> Result<usize> {
    let bytes = fs::read(path)?;
    let mut count = bytes.iter().filter(|byte| **byte == b'\n').count();
    if !bytes.is_empty() && !bytes.ends_with(b"\n") {
        count += 1;
    }
    Ok(count)
}
