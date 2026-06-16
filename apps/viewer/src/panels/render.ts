import type { AppStore } from "../store/app-store";
import type { ViewerElements } from "../app/elements";
import { renderCoverageSummary } from "./coverage-summary";
import { renderDiagnostics } from "./diagnostics/render";
import { renderInspector } from "./inspector/render";
import { renderProjectTree } from "./project-tree";
import { renderSearch } from "./search/render";
import type { SelectSubject } from "./subject-buttons";

export type PanelActions = {
  select: SelectSubject;
  openFsm: (fsmId: string) => void;
  openProjection: (projectionId: string) => void;
};

export function renderPanels(elements: ViewerElements, store: AppStore, actions: PanelActions): void {
  const subject = store.selection.selected;
  const panel = subject ? store.view.inspector_panels.find((candidate) => candidate.subject === subject) : undefined;
  renderProjectTree(elements.projectTree, store, actions);
  renderCoverageSummary(elements.coverageSummary, store);
  renderInspector(elements.inspector, panel, actions.select);
  renderDiagnostics(elements.diagnostics, store);
  renderSearch(elements.searchInput, elements.searchResults, store, actions.select);
}
