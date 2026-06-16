import type { AppStore } from "../store/app-store";
import { escapeHtml } from "./html";
import { bindSubjectButtons, type SelectSubject } from "./subject-buttons";

export function renderProjectTree(element: HTMLElement, store: AppStore, onSelect: SelectSubject): void {
  const fsmRows = (store.ir.fsms ?? [])
    .map(
      (fsm) => `<button class="tree-row" data-subject="${escapeHtml(fsm.id)}">
        <span>${escapeHtml(fsm.name)}</span><small>${(fsm.states ?? []).length} states</small>
      </button>`
    )
    .join("");
  element.innerHTML = `
    <div class="project-name">${escapeHtml(store.ir.project.name)}</div>
    ${fsmRows}
  `;
  bindSubjectButtons(element, onSelect);
}
