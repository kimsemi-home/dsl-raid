use dslraid_core::{event_subject, state_subject, Fsm, Transition};

use crate::view::{InspectorPanel, InspectorRow, InspectorSection};

pub(crate) fn transition_panel(
    fsm: &Fsm,
    transition: &Transition,
    subject: &str,
) -> InspectorPanel {
    InspectorPanel {
        subject: subject.to_string(),
        title: transition.id.clone(),
        sections: vec![InspectorSection {
            title: "Transition".to_string(),
            rows: rows(fsm, transition),
        }],
    }
}

fn rows(fsm: &Fsm, transition: &Transition) -> Vec<InspectorRow> {
    vec![
        row(
            "From",
            transition.from.clone(),
            Some(state_subject(&fsm.id, &transition.from)),
        ),
        row(
            "To",
            transition.to.clone(),
            Some(state_subject(&fsm.id, &transition.to)),
        ),
        row(
            "Event",
            event_value(transition),
            event_subject_ref(fsm, transition),
        ),
    ]
}

fn event_value(transition: &Transition) -> String {
    transition
        .on
        .clone()
        .unwrap_or_else(|| "epsilon".to_string())
}

fn event_subject_ref(fsm: &Fsm, transition: &Transition) -> Option<String> {
    transition
        .on
        .as_ref()
        .map(|event| event_subject(&fsm.id, event))
}

fn row(label: &str, value: String, subject: Option<String>) -> InspectorRow {
    InspectorRow {
        label: label.to_string(),
        value,
        subject,
    }
}
