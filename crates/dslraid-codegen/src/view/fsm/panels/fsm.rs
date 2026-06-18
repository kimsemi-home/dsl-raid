use dslraid_core::{CoreIr, Fsm};

use crate::view::{InspectorPanel, InspectorRow, InspectorSection};

pub(crate) fn fsm_panel(ir: &CoreIr, fsm: &Fsm) -> InspectorPanel {
    InspectorPanel {
        subject: fsm.id.clone(),
        title: fsm.name.clone(),
        sections: vec![InspectorSection {
            title: "Summary".to_string(),
            rows: vec![
                row("States", fsm.states.len().to_string(), None),
                row("Transitions", fsm.transitions.len().to_string(), None),
                row(
                    "Project",
                    ir.project.name.clone(),
                    Some(format!("project:{}", ir.project.id)),
                ),
            ],
        }],
    }
}

fn row(label: &str, value: String, subject: Option<String>) -> InspectorRow {
    InspectorRow {
        label: label.to_string(),
        value,
        subject,
    }
}
