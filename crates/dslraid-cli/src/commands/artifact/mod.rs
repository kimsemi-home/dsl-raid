mod kind;
mod lock;
mod lock_update;
mod path;
mod verify;

#[cfg(test)]
mod tests;

use std::path::Path;

pub(crate) use path::resolve_artifact_path;

pub(crate) fn verify(
    input: &Path,
    lock: Option<&Path>,
    format: crate::OutputFormat,
) -> anyhow::Result<()> {
    verify::run(input, lock, format)
}

pub(crate) fn update_lock(input: &Path, out: Option<&Path>) -> anyhow::Result<()> {
    lock_update::run(input, out)
}
