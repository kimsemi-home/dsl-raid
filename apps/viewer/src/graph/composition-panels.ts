import type { Composition, InspectorPanel, InspectorRow } from "../types";
import type { CompositionMaterialization, TupleEdge, TupleNode } from "./composition-types";
import { compositionPanel } from "./inspector/composition-panel";

export function compositionPanels(composition: Composition, result: CompositionMaterialization): InspectorPanel[] {
  return [
    compositionPanel(composition, {
      materialized: result.nodes.length,
      transitions: result.edges.length,
      stateSpace: result.stateSpace,
      truncated: result.truncated
    }),
    ...result.nodes.map(tuplePanel),
    ...result.edges.map(edgePanel)
  ];
}

function tuplePanel(node: TupleNode): InspectorPanel {
  return {
    subject: node.subject,
    title: node.states.join(" x "),
    sections: [
      {
        title: "State Tuple",
        rows: [
          { label: "Initial", value: String(node.initial) },
          { label: "Terminal", value: String(node.terminal) },
          { label: "Members", value: String(node.members.length) }
        ]
      },
      { title: "Member States", rows: memberRows(node.members) }
    ]
  };
}

function edgePanel(edge: TupleEdge): InspectorPanel {
  return {
    subject: edge.subject,
    title: edge.event ?? edge.members[0] ?? edge.subject,
    sections: [{
      title: "Tuple Transition",
      rows: [
        { label: "From", value: edge.from, subject: edge.from },
        { label: "To", value: edge.to, subject: edge.to },
        { label: "Event", value: edge.event ?? "epsilon", subject: edge.event },
        ...memberRows(edge.members)
      ]
    }]
  };
}

function memberRows(members: string[]): InspectorRow[] {
  return members.map((member) => ({ label: "Member", value: member, subject: member }));
}
