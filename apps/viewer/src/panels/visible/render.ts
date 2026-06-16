import { focusedView } from "../../graph/neighborhood";
import { visibleSubjects, visibleSubjectSummary } from "../../graph/visible-subjects";
import type { AppStore } from "../../store/app-store";
import { escapeHtml } from "../html";
import { bindSubjectButtons, type SelectSubject } from "../subject-buttons";

export function renderVisibleSubjects(element: HTMLElement, store: AppStore, onSelect: SelectSubject): void {
  const view = focusedView(store.view, store.selection.selected, store.focusDepth);
  const summary = visibleSubjectSummary(view);
  const subjects = visibleSubjects(view);
  element.innerHTML = `
    <div class="visible-summary">${summary.states} states / ${summary.transitions} transitions</div>
    <div class="visible-list">${subjects.map((item) => itemHtml(item, store.selection.selected)).join("")}</div>
  `;
  bindSubjectButtons(element, onSelect);
}

function itemHtml(item: { subject: string; label: string; kind: string }, selected?: string): string {
  const active = item.subject === selected ? " active" : "";
  return `
    <button class="visible-subject${active}" data-subject="${escapeHtml(item.subject)}">
      <span>${escapeHtml(item.label)}</span>
      <small>${escapeHtml(item.kind)}</small>
    </button>
  `;
}
