use super::model::TupleEdge;
use super::panel_tail::{member_rows, row};
use crate::view::{InspectorPanel, InspectorSection};

pub(crate) fn edge_panel(edge: &TupleEdge) -> InspectorPanel {
    InspectorPanel {
        subject: edge.subject.clone(),
        title: edge.event.clone().unwrap_or_else(|| edge.subject.clone()),
        sections: vec![InspectorSection {
            title: "Tuple Transition".to_string(),
            rows: edge_rows(edge),
        }],
    }
}

fn edge_rows(edge: &TupleEdge) -> Vec<crate::view::InspectorRow> {
    [
        vec![
            row("From", &edge.from, Some(edge.from.clone())),
            row("To", &edge.to, Some(edge.to.clone())),
            row(
                "Event",
                edge.event.as_deref().unwrap_or("epsilon"),
                edge.event.clone(),
            ),
        ],
        member_rows(&edge.members),
    ]
    .concat()
}
