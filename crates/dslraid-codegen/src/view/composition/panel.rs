use super::model::{MaterializedComposition, TupleNode};
use super::panel_edge::edge_panel;
use super::panel_tail::{member_rows, row};
use crate::view::{InspectorPanel, InspectorSection};
use dslraid_core::Composition;

pub(crate) fn panels(
    composition: &Composition,
    result: &MaterializedComposition,
) -> Vec<InspectorPanel> {
    let mut panels = vec![composition_panel(composition, result)];
    panels.extend(result.nodes.iter().map(tuple_panel));
    panels.extend(result.edges.iter().map(edge_panel));
    panels
}

fn composition_panel(
    composition: &Composition,
    result: &MaterializedComposition,
) -> InspectorPanel {
    InspectorPanel {
        subject: composition.id.clone(),
        title: composition.name.clone(),
        sections: vec![
            InspectorSection {
                title: "Composition".to_string(),
                rows: vec![
                    row("Kind", &composition.kind, None),
                    row("Inputs", &composition.inputs.len().to_string(), None),
                    row("State Space", &result.state_space.to_string(), None),
                    row("Materialized", &result.nodes.len().to_string(), None),
                    row("Transitions", &result.edges.len().to_string(), None),
                    row("Truncated", &result.truncated.to_string(), None),
                ],
            },
            InspectorSection {
                title: "Input FSMs".to_string(),
                rows: composition
                    .inputs
                    .iter()
                    .map(|input| row("FSM", input, Some(input.clone())))
                    .collect(),
            },
        ],
    }
}

fn tuple_panel(node: &TupleNode) -> InspectorPanel {
    InspectorPanel {
        subject: node.subject.clone(),
        title: node.states.join(" x "),
        sections: vec![InspectorSection {
            title: "State Tuple".to_string(),
            rows: member_rows(&node.members),
        }],
    }
}
