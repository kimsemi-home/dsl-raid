import type { RuntimeEvent } from "../types";
import { traceHasProblem } from "./trace-status";

export function traceBadges(events?: RuntimeEvent[]): string[] {
  if (!events?.length) {
    return [];
  }
  const failures = events.filter(traceHasProblem).length;
  return failures > 0 ? [`trace ${events.length}`, `fail ${failures}`] : [`trace ${events.length}`];
}

export function traceTone(events?: RuntimeEvent[]): "success" | "warning" | "danger" | undefined {
  if (!events?.length) {
    return undefined;
  }
  if (events.some((event) => event.status === "failed" || event.status === "timeout")) {
    return "danger";
  }
  if (events.some((event) => event.status === "cancelled" || event.status === "degraded")) {
    return "warning";
  }
  return "success";
}
