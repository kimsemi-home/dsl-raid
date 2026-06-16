mod count;
mod roots;
mod scan;

use anyhow::Result;
use std::path::Path;

pub(super) fn check() -> Result<()> {
    check_at(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .as_path(),
    )
}

fn check_at(root: &Path) -> Result<()> {
    let limit = roots::line_limit();
    let failures = scan::failures(root, limit)?;
    if failures.is_empty() {
        println!("source shape ok: <= {limit} lines");
        Ok(())
    } else {
        for failure in &failures {
            eprintln!(
                "line budget exceeded: {} has {} lines, limit is {}",
                failure.path.display(),
                failure.lines,
                limit
            );
        }
        anyhow::bail!("source shape failed with {} files", failures.len())
    }
}
