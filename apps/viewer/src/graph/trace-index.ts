import type { RuntimeEvent, RuntimeTrace } from "../types";

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

function eventSubjects(event: RuntimeEvent): string[] {
  return [...new Set([event.subject, event.from, event.to].filter(Boolean) as string[])];
}
