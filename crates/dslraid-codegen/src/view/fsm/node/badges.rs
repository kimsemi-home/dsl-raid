use dslraid_core::State;

pub(super) fn state_badges(state: &State, diagnostic: Option<&str>) -> Vec<String> {
    let mut badges = Vec::new();
    if let Some(badge) = diagnostic {
        badges.push(badge.to_string());
    }
    if state.initial {
        badges.push("initial".to_string());
    }
    if state.terminal {
        badges.push(terminal_badge(state));
    }
    badges.extend(state.tags.clone());
    badges
}

fn terminal_badge(state: &State) -> String {
    state
        .terminal_semantics
        .clone()
        .unwrap_or_else(|| "terminal".to_string())
}
