use dslraid_core::{state_subject, Fsm, State};

use super::diagnostic::DiagnosticMarks;
use super::{layout_state_id, StyleToken, ViewNode};

const WIDTH: f64 = 168.0;
const HEIGHT: f64 = 58.0;
const COL_GAP: f64 = 230.0;
const ROW_GAP: f64 = 150.0;

pub(crate) fn state_node(
    fsm: &Fsm,
    state: &State,
    index: usize,
    diagnostics: &DiagnosticMarks,
) -> ViewNode {
    let subject = state_subject(&fsm.id, &state.id);
    let x = 80.0 + (index as f64 % 3.0) * COL_GAP;
    let y = 90.0 + (index as f64 / 3.0).floor() * ROW_GAP;
    ViewNode {
        id: layout_state_id(fsm, &state.id),
        subject: subject.clone(),
        x,
        y,
        width: WIDTH,
        height: HEIGHT,
        label: state.id.clone(),
        badges: state_badges(state, diagnostics.badge(&subject)),
        style: Some(state_style(state, diagnostics.tone(&subject))),
    }
}

fn state_badges(state: &State, diagnostic: Option<&str>) -> Vec<String> {
    let mut badges = Vec::new();
    if let Some(badge) = diagnostic {
        badges.push(badge.to_string());
    }
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

fn state_style(state: &State, diagnostic: Option<&str>) -> StyleToken {
    StyleToken {
        tone: diagnostic
            .unwrap_or(if state.terminal { "success" } else { "default" })
            .to_string(),
        emphasis: if state.initial || state.terminal {
            "strong"
        } else {
            "normal"
        }
        .to_string(),
    }
}
