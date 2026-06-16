import type { CoverageSubject, Fsm, RuntimeEvent, SceneNode } from "../types";
import { coverageBadges, coverageTone } from "./coverage";
import { layoutStateId, stateSubject } from "./ids";
import { traceBadges, traceTone } from "./trace";

const width = 168;
const height = 58;
const colGap = 230;
const rowGap = 150;

export function projectStateNodes(
  fsm: Fsm,
  coverage: Map<string, CoverageSubject>,
  trace: Map<string, RuntimeEvent[]>
): SceneNode[] {
  return (fsm.states ?? []).map((state, index) => {
    const subject = stateSubject(fsm.id, state.id);
    const coverageSubject = coverage.get(subject);
    const traceEvents = trace.get(subject);
    return {
      id: layoutStateId(fsm.id, state.id),
      subject,
      x: 80 + (index % 3) * colGap,
      y: 90 + Math.floor(index / 3) * rowGap,
      width,
      height,
      label: state.id,
      badges: [
        ...(state.initial ? ["initial"] : []),
        ...(state.terminal ? [state.terminal_semantics ?? "terminal"] : []),
        ...(state.tags ?? []),
        ...coverageBadges(coverageSubject),
        ...traceBadges(traceEvents)
      ],
      style: {
        tone: traceTone(traceEvents) ?? coverageTone(coverageSubject) ?? fallbackTone(state.terminal, state.terminal_semantics),
        emphasis: coverageSubject?.status === "uncovered" ? "faint" : state.initial || state.terminal ? "strong" : "normal",
        coverage: coverageSubject?.status
      }
    };
  });
}

function fallbackTone(terminal?: boolean, semantics?: string): "default" | "success" | "danger" {
  if (!terminal) {
    return "default";
  }
  return semantics === "failed" ? "danger" : "success";
}
