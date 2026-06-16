import type { AppStore } from "../../store/app-store";
import type { RuntimeEvent } from "../../types";
import { escapeHtml } from "../html";
import { bindSubjectButtons, type SelectSubject } from "../subject-buttons";

export function renderTimeline(element: HTMLElement, store: AppStore, onSelect: SelectSubject): void {
  const trace = store.trace;
  if (!trace || trace.events.length === 0) {
    element.innerHTML = `<p class="muted">No runtime trace loaded.</p>`;
    return;
  }
  element.innerHTML = `
    <div class="trace-run">${escapeHtml(trace.run.id)}${environment(trace.run.environment)}</div>
    <div class="timeline-list">${trace.events.map(eventHtml).join("")}</div>
  `;
  bindSubjectButtons(element, onSelect);
}

function eventHtml(event: RuntimeEvent): string {
  const subject = event.subject ?? event.to ?? event.from ?? "";
  return `
    <button class="timeline-event ${escapeHtml(event.status ?? "ok")}" data-subject="${escapeHtml(subject)}">
      <span>${escapeHtml(timeLabel(event.timestamp))}</span>
      <strong>${escapeHtml(event.kind.replaceAll("_", " "))}</strong>
      <small>${escapeHtml(subject || event.id)}</small>
    </button>
  `;
}

function timeLabel(timestamp: string): string {
  return timestamp.split("T")[1]?.replace("Z", "") ?? timestamp;
}

function environment(value: string | undefined): string {
  return value ? ` / ${escapeHtml(value)}` : "";
}
