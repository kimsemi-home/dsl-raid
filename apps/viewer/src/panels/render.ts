import type { AppStore } from "../store/app-store";
import type { ViewerElements } from "../app/elements";
import { renderCoverageSummary } from "./coverage-summary";
import { renderDiagnostics } from "./diagnostics/render";
import { renderInspector } from "./inspector/render";
import { renderTerminalPath } from "./path/render";
import { renderProjectTree } from "./project-tree";
import { renderSearch } from "./search/render";
import type { SelectSubject } from "./subject-buttons";
import { renderTimeline } from "./timeline/render";
import { renderVisibleSubjects } from "./visible/render";

export type PanelActions = {
  select: SelectSubject;
  openFsm: (fsmId: string) => void;
  openProjection: (projectionId: string) => void;
};

export function renderPanels(elements: ViewerElements, store: AppStore, actions: PanelActions): void {
  const subject = store.selection.selected;
  const panel = subject ? store.view.inspector_panels.find((candidate) => candidate.subject === subject) : undefined;
  renderProjectTree(elements.projectTree, store, actions);
  renderVisibleSubjects(elements.visibleSubjects, store, actions.select);
  renderTerminalPath(elements.terminalPath, store, actions.select);
  renderCoverageSummary(elements.coverageSummary, store);
  renderInspector(elements.inspector, panel, store.sourceMap, actions.select);
  renderDiagnostics(elements.diagnostics, store);
  renderTimeline(elements.timeline, store, actions.select);
  renderSearch(elements.searchInput, elements.searchResults, store, actions.select);
}
