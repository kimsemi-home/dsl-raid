use dslraid_core::{state_subject, transition_subject, Fsm, Transition};

pub(crate) fn initial_tuple(fsms: &[&Fsm]) -> Vec<String> {
    fsms.iter()
        .map(|fsm| {
            fsm.states
                .iter()
                .find(|state| state.initial)
                .or_else(|| fsm.states.first())
                .map(|state| state.id.clone())
                .unwrap_or_default()
        })
        .collect()
}

pub(crate) fn tuple_key(tuple: &[String]) -> String {
    tuple.join("\u{1f}")
}

pub(crate) fn tuple_members(fsms: &[&Fsm], tuple: &[String]) -> Vec<String> {
    fsms.iter()
        .zip(tuple.iter())
        .map(|(fsm, state)| state_subject(&fsm.id, state))
        .collect()
}

pub(crate) fn tuple_subject(composition: &str, members: &[String]) -> String {
    format!(
        "state_tuple:{}.{}",
        local(composition),
        members
            .iter()
            .map(|member| sanitize(member))
            .collect::<Vec<_>>()
            .join("__")
    )
}

pub(crate) fn edge_subject(
    composition: &str,
    from: &[String],
    to: &[String],
    fsm: &Fsm,
    transition: &Transition,
) -> String {
    format!(
        "tuple_transition:{}.{}.{}.{}",
        local(composition),
        sanitize(&tuple_key(from)),
        sanitize(&tuple_key(to)),
        sanitize(&transition_subject(&fsm.id, &transition.id))
    )
}

pub(crate) fn local(subject: &str) -> &str {
    subject.split(':').next_back().unwrap_or(subject)
}

fn sanitize(value: &str) -> String {
    value
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}
