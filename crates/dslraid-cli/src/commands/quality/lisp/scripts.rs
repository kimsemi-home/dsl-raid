mod base;
mod governance;
mod runtime;
mod ssot;

pub(super) fn checks() -> impl Iterator<Item = &'static str> {
    base::CHECKS
        .iter()
        .chain(governance::CHECKS)
        .chain(runtime::CHECKS)
        .chain(ssot::CHECKS)
        .copied()
}
