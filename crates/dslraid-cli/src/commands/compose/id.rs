use dslraid_core::Transition;

pub(super) fn tuple_transition_id(
    composition_id: &str,
    from_tuple: &[String],
    fsm_id: &str,
    transition: &Transition,
) -> String {
    format!(
        "tuple_transition:{}:{}:{}:{}",
        composition_id.trim_start_matches("composition:"),
        from_tuple.join("_"),
        fsm_id.trim_start_matches("fsm:"),
        transition.id
    )
}
