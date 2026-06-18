use dslraid_core::{Composition, CoreIr};

pub(super) fn selected_composition<'a>(
    ir: &'a CoreIr,
    composition: Option<&str>,
) -> Option<&'a Composition> {
    composition
        .and_then(|id| ir.compositions.iter().find(|item| item.id == id))
        .or_else(|| ir.compositions.first())
}
