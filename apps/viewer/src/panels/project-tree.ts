import type { AppStore } from "../store/app-store";
import { escapeHtml } from "./html";

export type ProjectTreeActions = {
  openFsm: (fsmId: string) => void;
  openProjection: (projectionId: string) => void;
};

export function renderProjectTree(element: HTMLElement, store: AppStore, actions: ProjectTreeActions): void {
  element.innerHTML = `
    <div class="project-name">${escapeHtml(store.ir.project.name)}</div>
    <h2>Views</h2>
    ${projectionRows(store)}
    <h2>FSMs</h2>
    ${fsmRows(store)}
  `;
  bindProjectTree(element, actions);
}

function projectionRows(store: AppStore): string {
  return (store.ir.projections ?? [])
    .map((projection) => {
      const active = projection.id === store.activeProjectionId ? " active" : "";
      return `<button class="tree-row${active}" data-projection="${escapeHtml(projection.id)}">
        <span>${escapeHtml(projection.id)}</span><small>${escapeHtml(projection.source)}</small>
      </button>`;
    })
    .join("");
}

function fsmRows(store: AppStore): string {
  return (store.ir.fsms ?? [])
    .map(
      (fsm) => `<button class="tree-row" data-fsm="${escapeHtml(fsm.id)}">
        <span>${escapeHtml(fsm.name)}</span><small>${(fsm.states ?? []).length} states</small>
      </button>`
    )
    .join("");
}

function bindProjectTree(element: HTMLElement, actions: ProjectTreeActions): void {
  element.querySelectorAll<HTMLButtonElement>("[data-projection]").forEach((button) => {
    button.addEventListener("click", () => actions.openProjection(button.dataset.projection ?? ""));
  });
  element.querySelectorAll<HTMLButtonElement>("[data-fsm]").forEach((button) => {
    button.addEventListener("click", () => actions.openFsm(button.dataset.fsm ?? ""));
  });
}
