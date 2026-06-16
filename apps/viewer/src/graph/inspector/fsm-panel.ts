import type { CoreIr, Fsm, InspectorPanel } from "../../types";
import { fsmSummary } from "../fsm-summary";
import { artifactRow, artifactsForSubject } from "../traceability";

export function fsmPanel(ir: CoreIr, fsm: Fsm): InspectorPanel {
  const summary = fsmSummary(fsm);
  const artifacts = artifactsForSubject(ir, fsm.id);
  return {
    subject: fsm.id,
    title: fsm.name,
    sections: [
      {
        title: "Summary",
        rows: [
          { label: "Project", value: ir.project.name, subject: `project:${ir.project.id}` },
          { label: "States", value: String(summary.states) },
          { label: "Transitions", value: String(summary.transitions) },
          { label: "Events", value: String((fsm.events ?? []).length) },
          { label: "Initial", value: stateList(fsm, "initial") },
          { label: "Terminal", value: stateList(fsm, "terminal") },
          { label: "Source", value: fsm.defined_at?.uri ?? "not linked" }
        ]
      },
      {
        title: "Traceability",
        rows: artifacts.length > 0 ? artifacts.map(artifactRow) : [{ label: "Artifacts", value: "none linked" }]
      }
    ]
  };
}

function stateList(fsm: Fsm, flag: "initial" | "terminal"): string {
  const states = (fsm.states ?? []).filter((state) => Boolean(state[flag]));
  return states.map((state) => state.id).join(", ") || "none";
}
