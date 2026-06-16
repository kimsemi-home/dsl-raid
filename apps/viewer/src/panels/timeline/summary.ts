import type { RuntimeTrace } from "../../types";
import { escapeHtml } from "../html";
import { relatedEvents } from "./relation";

export function summaryHtml(trace: RuntimeTrace, selected?: string): string {
  const related = relatedEvents(trace.events, selected);
  return `
    <div class="trace-run">${escapeHtml(trace.run.id)}${environment(trace.run.environment)}</div>
    ${selected ? selectedHtml(selected, related.length) : ""}
  `;
}

function selectedHtml(selected: string, count: number): string {
  return `
    <div class="trace-selection">
      ${escapeHtml(String(count))} related events for ${escapeHtml(selected)}
    </div>
  `;
}

function environment(value: string | undefined): string {
  return value ? ` / ${escapeHtml(value)}` : "";
}
