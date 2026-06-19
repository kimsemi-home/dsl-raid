mod fixtures;
mod golden;
mod syntax;

use anyhow::Result;

pub(super) fn check_fixtures() -> Result<()> {
    syntax::check_json_syntax("schemas")?;
    syntax::check_json_syntax("examples")?;
    syntax::check_json_syntax("docs/generated")?;
    syntax::check_json_syntax("tests/golden")?;
    fixtures::check()?;
    golden::check()
}
