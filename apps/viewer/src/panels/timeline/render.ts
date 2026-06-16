import type { AppStore } from "../../store/app-store";
import { bindSubjectButtons, type SelectSubject } from "../subject-buttons";
import { eventHtml } from "./event-card";
import { summaryHtml } from "./summary";

export function renderTimeline(element: HTMLElement, store: AppStore, onSelect: SelectSubject): void {
  const trace = store.trace;
  if (!trace || trace.events.length === 0) {
    element.innerHTML = `<p class="muted">No runtime trace loaded.</p>`;
    return;
  }
  const selected = store.selection.selected;
  element.innerHTML = `${summaryHtml(trace, selected)}
    <div class="timeline-list">${trace.events.map((event) => eventHtml(event, selected)).join("")}</div>`;
  bindSubjectButtons(element, onSelect);
}
