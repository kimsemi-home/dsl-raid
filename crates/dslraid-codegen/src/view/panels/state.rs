use dslraid_core::Fsm;

use crate::view::{InspectorPanel, InspectorRow, InspectorSection};

pub(crate) fn state_panel(fsm: &Fsm, state_id: &str, subject: &str) -> InspectorPanel {
    InspectorPanel {
        subject: subject.to_string(),
        title: state_id.to_string(),
        sections: vec![InspectorSection {
            title: "State".to_string(),
            rows: vec![
                row("Parent FSM", fsm.id.clone(), Some(fsm.id.clone())),
                row("Incoming", count_incoming(fsm, state_id).to_string(), None),
                row("Outgoing", count_outgoing(fsm, state_id).to_string(), None),
            ],
        }],
    }
}

fn count_incoming(fsm: &Fsm, state_id: &str) -> usize {
    fsm.transitions
        .iter()
        .filter(|transition| transition.to == state_id)
        .count()
}

fn count_outgoing(fsm: &Fsm, state_id: &str) -> usize {
    fsm.transitions
        .iter()
        .filter(|transition| transition.from == state_id)
        .count()
}

fn row(label: &str, value: String, subject: Option<String>) -> InspectorRow {
    InspectorRow {
        label: label.to_string(),
        value,
        subject,
    }
}
