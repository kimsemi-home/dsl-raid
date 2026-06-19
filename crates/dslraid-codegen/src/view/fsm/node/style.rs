use dslraid_core::State;

use crate::view::StyleToken;

pub(super) fn state_style(state: &State, diagnostic: Option<&str>) -> StyleToken {
    StyleToken {
        tone: tone(state, diagnostic).to_string(),
        emphasis: emphasis(state).to_string(),
    }
}

fn tone<'a>(state: &'a State, diagnostic: Option<&'a str>) -> &'a str {
    diagnostic.unwrap_or(if state.terminal { "success" } else { "default" })
}

fn emphasis(state: &State) -> &'static str {
    if state.initial || state.terminal {
        "strong"
    } else {
        "normal"
    }
}
