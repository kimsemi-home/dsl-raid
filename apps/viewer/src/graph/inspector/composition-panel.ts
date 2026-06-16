import type { Composition, InspectorPanel, InspectorRow } from "../../types";

export type CompositionMetrics = {
  materialized?: number;
  transitions?: number;
  stateSpace?: number;
  truncated?: boolean;
};

export function compositionPanel(composition: Composition, metrics: CompositionMetrics = {}): InspectorPanel {
  return {
    subject: composition.id,
    title: composition.name,
    sections: [
      {
        title: "Composition",
        rows: [
          { label: "Kind", value: composition.kind },
          { label: "Inputs", value: String((composition.inputs ?? []).length) },
          { label: "State Space", value: String(metrics.stateSpace ?? "lazy") },
          { label: "Materialized", value: String(metrics.materialized ?? 0) },
          { label: "Transitions", value: String(metrics.transitions ?? 0) },
          { label: "Truncated", value: String(metrics.truncated ?? false) }
        ]
      },
      {
        title: "Input FSMs",
        rows: inputRows(composition)
      }
    ]
  };
}

function inputRows(composition: Composition): InspectorRow[] {
  const inputs = composition.inputs ?? [];
  if (inputs.length === 0) {
    return [{ label: "FSM", value: "none" }];
  }
  return inputs.map((input) => ({ label: "FSM", value: input, subject: input }));
}
