import type { CoreIr, CoverageSubject, Fsm, InspectorPanel, RuntimeEvent, Transition } from "../../types";
import { coverageRows } from "../coverage";
import { eventSubject, stateSubject } from "../ids";
import { traceRows } from "../trace";
import { artifactRow, artifactsForSubject } from "../traceability";
import { diagnosticSection } from "./diagnostic-section";

export function transitionPanel(
  ir: CoreIr,
  fsm: Fsm,
  transition: Transition,
  subject: string,
  coverage?: CoverageSubject,
  traceEvents?: RuntimeEvent[]
): InspectorPanel {
  const requires = transition.requires ?? [];
  return {
    subject,
    title: transition.id,
    sections: [
      {
        title: "Transition",
        rows: [
          { label: "From", value: transition.from, subject: stateSubject(fsm.id, transition.from) },
          { label: "To", value: transition.to, subject: stateSubject(fsm.id, transition.to) },
          {
            label: "Event",
            value: transition.on ?? "epsilon",
            subject: transition.on ? eventSubject(fsm.id, transition.on) : undefined
          }
        ]
      },
      {
        title: "Policy",
        rows:
          requires.length > 0
            ? requires.map((required) => ({ label: "Requires", value: required, subject: required }))
            : [{ label: "Requires", value: "none" }]
      },
      {
        title: "Coverage",
        rows: coverageRows(coverage)
      },
      {
        title: "Runtime Trace",
        rows: traceRows(traceEvents)
      },
      {
        title: "Traceability",
        rows: artifactsForSubject(ir, subject).map(artifactRow)
      },
      ...[diagnosticSection(ir, subject)].filter((section) => section !== undefined)
    ]
  };
}
