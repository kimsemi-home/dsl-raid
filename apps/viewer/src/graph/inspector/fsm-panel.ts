import type { CoreIr, Fsm, InspectorPanel } from "../../types";

export function fsmPanel(ir: CoreIr, fsm: Fsm): InspectorPanel {
  return {
    subject: fsm.id,
    title: fsm.name,
    sections: [
      {
        title: "Summary",
        rows: [
          { label: "Project", value: ir.project.name, subject: `project:${ir.project.id}` },
          { label: "States", value: String((fsm.states ?? []).length) },
          { label: "Transitions", value: String((fsm.transitions ?? []).length) },
          { label: "Source", value: fsm.defined_at?.uri ?? "not linked" }
        ]
      }
    ]
  };
}
