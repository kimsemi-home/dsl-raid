mod examples;
mod verification;

use anyhow::Result;
use std::path::Path;

type Fixture = (&'static str, &'static str);

pub(super) fn check() -> Result<()> {
    for (schema, input) in all_schemas() {
        crate::schema_validate(Path::new(schema), Path::new(input))?;
    }
    Ok(())
}

fn all_schemas() -> impl Iterator<Item = Fixture> {
    examples::schemas()
        .into_iter()
        .chain(verification::schemas())
}
