import type { InspectorRow, RuntimeEvent, RuntimeTrace } from "../types";

export type TraceIndex = Map<string, RuntimeEvent[]>;

export function traceIndex(trace?: RuntimeTrace): TraceIndex {
  const index = new Map<string, RuntimeEvent[]>();
  for (const event of trace?.events ?? []) {
    for (const subject of eventSubjects(event)) {
      const events = index.get(subject) ?? [];
      events.push(event);
      index.set(subject, events);
    }
  }
  return index;
}

export function traceBadges(events?: RuntimeEvent[]): string[] {
  if (!events?.length) {
    return [];
  }
  const failures = events.filter(isProblem).length;
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

export function traceRows(events?: RuntimeEvent[]): InspectorRow[] {
  if (!events?.length) {
    return [{ label: "Events", value: "not observed" }];
  }
  const latest = events.at(-1);
  return [
    { label: "Events", value: String(events.length) },
    { label: "Status", value: status(events) },
    { label: "Last event", value: latest?.kind.replaceAll("_", " ") ?? "n/a" },
    { label: "Last seen", value: latest?.timestamp ?? "never" },
    { label: "Duration", value: `${duration(events)}ms` }
  ];
}

function eventSubjects(event: RuntimeEvent): string[] {
  return [...new Set([event.subject, event.from, event.to].filter(Boolean) as string[])];
}

function isProblem(event: RuntimeEvent): boolean {
  return ["failed", "timeout", "cancelled", "policy_blocked", "degraded"].includes(event.status ?? "");
}

function status(events: RuntimeEvent[]): string {
  const problem = events.find(isProblem);
  return problem?.status ?? "ok";
}

function duration(events: RuntimeEvent[]): number {
  return events.reduce((sum, event) => sum + (event.duration_ms ?? 0), 0);
}
