use super::Fixture;

mod core;
mod governance;
mod merge;
mod objective;
mod shape;
mod tail;

pub(super) const MANIFEST: &str = "schemas/dslraid-verification-manifest.schema.json";

pub(super) fn schemas() -> impl Iterator<Item = Fixture> {
    core::schemas()
        .into_iter()
        .chain(governance::schemas())
        .chain(merge::schemas())
        .chain(objective::schemas())
        .chain(shape::schemas())
        .chain(tail::schemas())
}
