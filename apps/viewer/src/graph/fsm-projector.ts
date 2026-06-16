import type { CoreIr, CoverageSubject, Fsm, InspectorPanel, Projection, RuntimeEvent, ViewModel } from "../types";
import { coverageIndex } from "./coverage";
import { artifactPanel } from "./inspector/artifact-panel";
import { derivationPanel } from "./inspector/derivation-panel";
import { fsmPanel } from "./inspector/fsm-panel";
import { statePanel } from "./inspector/state-panel";
import { transitionPanel } from "./inspector/transition-panel";
import { stateSubject, transitionSubject } from "./ids";
import { projectStateNodes } from "./state-nodes";
import { projectTransitionEdges } from "./transition-edges";

export function projectFsm(
  ir: CoreIr,
  projection: Projection,
  fsm: Fsm,
  coverage: Map<string, CoverageSubject>,
  trace: Map<string, RuntimeEvent[]>
): ViewModel {
  const nodes = projectStateNodes(fsm, coverage, trace);
  const edges = projectTransitionEdges(fsm, nodes, coverage, trace);
  return {
    view_version: "0.1.0",
    source: {
      core_ir: "loaded",
      projection: projection.id
    },
    layout: {
      engine: "manual",
      version: "0.1.0"
    },
    nodes,
    edges,
    inspector_panels: inspectorPanels(ir, fsm, coverage, trace)
  };
}

export { coverageIndex };

function inspectorPanels(
  ir: CoreIr,
  fsm: Fsm,
  coverage: Map<string, CoverageSubject>,
  trace: Map<string, RuntimeEvent[]>
): InspectorPanel[] {
  return [
    fsmPanel(ir, fsm),
    ...(ir.derivations ?? []).map(derivationPanel),
    ...(ir.artifacts ?? []).map((artifact) => artifactPanel(ir, artifact)),
    ...(fsm.states ?? []).map((state) => {
      const subject = stateSubject(fsm.id, state.id);
      return statePanel(ir, fsm, state.id, subject, coverage.get(subject), trace.get(subject));
    }),
    ...(fsm.transitions ?? []).map((transition) => {
      const subject = transitionSubject(fsm.id, transition.id);
      return transitionPanel(ir, fsm, transition, subject, coverage.get(subject), trace.get(subject));
    })
  ];
}
