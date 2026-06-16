import type { CoverageSubject, Fsm, SceneEdge, SceneNode } from "../types";
import { coverageTone } from "./coverage";
import { layoutTransitionId, stateSubject, transitionSubject } from "./ids";

export function projectTransitionEdges(
  fsm: Fsm,
  nodes: SceneNode[],
  coverage: Map<string, CoverageSubject>
): SceneEdge[] {
  return (fsm.transitions ?? []).flatMap((transition) => {
    const from = nodes.find((node) => node.subject === stateSubject(fsm.id, transition.from));
    const to = nodes.find((node) => node.subject === stateSubject(fsm.id, transition.to));
    if (!from || !to) {
      return [];
    }
    const subject = transitionSubject(fsm.id, transition.id);
    const coverageSubject = coverage.get(subject);
    return [
      {
        id: layoutTransitionId(fsm.id, transition.id),
        subject,
        from: from.id,
        to: to.id,
        label: transition.on ?? "epsilon",
        route: [
          { x: from.x + from.width, y: from.y + from.height / 2 },
          { x: to.x, y: to.y + to.height / 2 }
        ],
        style: {
          tone: coverageTone(coverageSubject) ?? ((transition.requires ?? []).length > 0 ? "warning" : "default"),
          emphasis: coverageSubject?.status === "uncovered" ? "faint" : "normal",
          coverage: coverageSubject?.status
        }
      }
    ];
  });
}
