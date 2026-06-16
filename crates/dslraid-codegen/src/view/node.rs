use dslraid_core::{state_subject, Fsm, State};

use super::{layout_state_id, StyleToken, ViewNode};

const WIDTH: f64 = 168.0;
const HEIGHT: f64 = 58.0;
const COL_GAP: f64 = 230.0;
const ROW_GAP: f64 = 150.0;

pub(crate) fn state_node(fsm: &Fsm, state: &State, index: usize) -> ViewNode {
    let x = 80.0 + (index as f64 % 3.0) * COL_GAP;
    let y = 90.0 + (index as f64 / 3.0).floor() * ROW_GAP;
    ViewNode {
        id: layout_state_id(fsm, &state.id),
        subject: state_subject(&fsm.id, &state.id),
        x,
        y,
        width: WIDTH,
        height: HEIGHT,
        label: state.id.clone(),
        badges: state_badges(state),
        style: Some(state_style(state)),
    }
}

fn state_badges(state: &State) -> Vec<String> {
    let mut badges = Vec::new();
    if state.initial {
        badges.push("initial".to_string());
    }
    if state.terminal {
        badges.push(
            state
                .terminal_semantics
                .clone()
                .unwrap_or_else(|| "terminal".to_string()),
        );
    }
    badges.extend(state.tags.clone());
    badges
}

fn state_style(state: &State) -> StyleToken {
    StyleToken {
        tone: if state.terminal { "success" } else { "default" }.to_string(),
        emphasis: if state.initial || state.terminal {
            "strong"
        } else {
            "normal"
        }
        .to_string(),
    }
}
