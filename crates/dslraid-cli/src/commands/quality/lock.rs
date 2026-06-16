use anyhow::{bail, Result};
use std::fs;
use std::path::Path;

pub(super) fn check(input: &Path) -> Result<()> {
    let lock_path = Path::new("examples/runscope/runscope.lock.json");
    let temp = std::env::temp_dir().join(format!("dslraid-lock-{}.json", std::process::id()));
    crate::commands::artifact::update_lock(input, Some(&temp))?;
    crate::schema_validate(Path::new("schemas/dslraid-lock.schema.json"), &temp)?;
    let actual = fs::read(&temp)?;
    fs::remove_file(&temp).ok();
    let expected = fs::read(lock_path)?;
    if actual != expected {
        bail!(
            "lock file is stale: run `dslraid artifact lock update {}`",
            input.display()
        );
    }
    Ok(())
}
