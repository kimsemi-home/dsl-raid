import type { RuntimeEvent } from "../types";

const problemStatuses = ["failed", "timeout", "cancelled", "policy_blocked", "degraded"];

export function traceHasProblem(event: RuntimeEvent): boolean {
  return problemStatuses.includes(event.status ?? "");
}

export function traceStatus(events: RuntimeEvent[]): string {
  const problem = events.find(traceHasProblem);
  return problem?.status ?? "ok";
}

export function traceDuration(events: RuntimeEvent[]): number {
  return events.reduce((sum, event) => sum + (event.duration_ms ?? 0), 0);
}
