import type { InspectorRow, RuntimeEvent } from "../types";
import { traceDuration, traceStatus } from "./trace-status";

export function traceRows(events?: RuntimeEvent[]): InspectorRow[] {
  if (!events?.length) {
    return [{ label: "Events", value: "not observed" }];
  }
  const latest = events.at(-1);
  return [
    { label: "Events", value: String(events.length) },
    { label: "Status", value: traceStatus(events) },
    { label: "Last event", value: latest?.kind.replaceAll("_", " ") ?? "n/a" },
    { label: "Last seen", value: latest?.timestamp ?? "never" },
    { label: "Duration", value: `${traceDuration(events)}ms` }
  ];
}
