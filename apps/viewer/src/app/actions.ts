import { zoomAt } from "../canvas/camera";
import { renderPanels } from "../panels/render";
import type { CoverageOverlay, CoreIr, Point, RuntimeTrace, SourceMapDocument } from "../types";
import type { ViewerActions } from "./action-types";
import { setCompositionLimit } from "./composition-limit";
import type { ViewerElements } from "./elements";
import { fitGraph as fitCamera } from "./fit";
import { followSelectedSubject } from "./follow-selected";
import { openFsm } from "./open-fsm";
import { selectRelativeSubject } from "./select-relative";
import * as viewerSession from "./session";
import { syncSourceMap, syncTrace } from "./sync-metadata";

export function createActions(session: viewerSession.ViewerSession, elements: ViewerElements, queueRender: () => void): ViewerActions {
  const refresh = () => { actions.syncPanels(); queueRender(); };
  const actions = {
    setIr: (ir: CoreIr, coverage?: CoverageOverlay, sourceMap?: SourceMapDocument, trace?: RuntimeTrace) => {
      viewerSession.setIr(session, ir, coverage, sourceMap, trace);
      actions.fit();
    },
    setCoverage: (coverage: CoverageOverlay) => {
      viewerSession.setCoverage(session, coverage);
      refresh();
    },
    setTrace: (trace: RuntimeTrace) => syncTrace(session, actions, trace),
    setSourceMap: (sourceMap: SourceMapDocument) => syncSourceMap(session, actions, sourceMap),
    openProjection: (projectionId: string) => {
      viewerSession.setProjection(session, projectionId);
      actions.fit();
    },
    openFsm: (fsmId: string) => openFsm(actions, session, fsmId),
    select: (subject: string | undefined) => {
      session.store.selection.selected = subject;
      refresh();
    },
    selectRelative: (step: -1 | 1) => {
      selectRelativeSubject(session, step);
      refresh();
    },
    followSelected: () => {
      followSelectedSubject(session);
      refresh();
    },
    hover: (subject: string | undefined) => {
      session.store.selection.hovered = subject;
      queueRender();
    },
    pan: (dx: number, dy: number) => {
      session.store.camera.panX += dx;
      session.store.camera.panY += dy;
      queueRender();
    },
    zoom: (point: Point, factor: number) => {
      session.store.camera = zoomAt(session.store.camera, point, session.store.camera.zoom * factor);
      queueRender();
    },
    fit: () => {
      session.store.camera = fitCamera(session.store.view, elements.canvas);
      refresh();
    },
    setDiagnosticsVisible: (visible: boolean) => {
      session.store.showDiagnostics = visible;
      actions.syncPanels();
    },
    setFocusDepth: (depth: 1 | 2) => {
      session.store.focusDepth = depth;
      refresh();
    },
    setCompositionLimit: (limit: number) => { setCompositionLimit(session, limit); actions.fit(); },
    updateStatus: (world: Point) => { elements.status.textContent = `zoom ${session.store.camera.zoom.toFixed(2)} / world ${world.x.toFixed(0)}, ${world.y.toFixed(0)}`; },
    syncPanels: () => renderPanels(elements, session.store, actions),
    queueRender
  };
  return actions;
}
