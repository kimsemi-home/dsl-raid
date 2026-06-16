import type { CoreIr, CoverageSubject, Fsm, InspectorPanel } from "../../types";
import { coverageRows } from "../coverage";
import { stateSubject } from "../ids";
import { artifactRow, artifactsForSubject } from "../traceability";

export function statePanel(ir: CoreIr, fsm: Fsm, stateId: string, subject: string, coverage?: CoverageSubject): InspectorPanel {
  const transitions = fsm.transitions ?? [];
  const incoming = transitions.filter((transition) => transition.to === stateId);
  const outgoing = transitions.filter((transition) => transition.from === stateId);
  const artifacts = artifactsForSubject(ir, subject);
  return {
    subject,
    title: stateId,
    sections: [
      {
        title: "State",
        rows: [
          { label: "Parent FSM", value: fsm.id, subject: fsm.id },
          { label: "Incoming", value: String(incoming.length) },
          { label: "Outgoing", value: String(outgoing.length) },
          { label: "Subject", value: subject, subject: stateSubject(fsm.id, stateId) }
        ]
      },
      {
        title: "Coverage",
        rows: coverageRows(coverage)
      },
      {
        title: "Traceability",
        rows: artifacts.length > 0 ? artifacts.map(artifactRow) : [{ label: "Artifacts", value: "none linked" }]
      }
    ]
  };
}
