import { terminalPath, type TerminalPathStep } from "../../graph/terminal-path";
import type { AppStore } from "../../store/app-store";
import { escapeHtml } from "../html";
import { bindSubjectButtons, type SelectSubject } from "../subject-buttons";

export function renderTerminalPath(element: HTMLElement, store: AppStore, onSelect: SelectSubject): void {
  const steps = terminalPath(store.view, store.selection.selected);
  element.innerHTML = steps.length ? stepList(steps, store.selection.selected) : emptyState();
  bindSubjectButtons(element, onSelect);
}

function stepList(steps: TerminalPathStep[], selected?: string): string {
  return `<div class="path-list">${steps.map((step) => stepHtml(step, selected)).join("")}</div>`;
}

function stepHtml(step: TerminalPathStep, selected?: string): string {
  const active = step.subject === selected ? " active" : "";
  return `
    <button class="path-step${active}" data-subject="${escapeHtml(step.subject)}">
      <span>${escapeHtml(step.label)}</span>
      <small>${escapeHtml(step.kind)}</small>
    </button>
  `;
}

function emptyState(): string {
  return `<div class="muted">No path</div>`;
}
