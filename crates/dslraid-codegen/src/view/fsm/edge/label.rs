use dslraid_core::Transition;

pub(super) fn transition_label(transition: &Transition) -> String {
    transition
        .on
        .clone()
        .unwrap_or_else(|| "epsilon".to_string())
}
