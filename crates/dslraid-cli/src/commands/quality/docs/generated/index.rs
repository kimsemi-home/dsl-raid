use anyhow::Result;

const SCRIPT: &str = "scripts/gendocindex.sh";

pub(super) fn check() -> Result<()> {
    super::script_check::check(SCRIPT)
}
