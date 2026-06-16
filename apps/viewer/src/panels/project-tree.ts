import type { AppStore } from "../store/app-store";
import { fsmSummary, fsmSummaryLabel } from "../graph/fsm-summary";
import { escapeHtml } from "./html";
import { activeFsmId } from "./project-tree-state";

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
  const activeId = activeFsmId(store);
  return (store.ir.fsms ?? [])
    .map((fsm) => {
      const active = fsm.id === activeId ? " active" : "";
      return `<button class="tree-row${active}" data-fsm="${escapeHtml(fsm.id)}">
        <span>${escapeHtml(fsm.name)}</span>
        <small>${escapeHtml(fsmSummaryLabel(fsmSummary(fsm)))}</small>
      </button>`;
    })
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
