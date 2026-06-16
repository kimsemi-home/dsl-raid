import { zoomAt } from "../canvas/camera";
import { renderPanels } from "../panels/render";
import type { CoverageOverlay, CoreIr, Point, SourceMapDocument } from "../types";
import type { ViewerActions } from "./action-types";
import type { ViewerElements } from "./elements";
import { fitGraph as fitCamera } from "./fit";
import * as viewerSession from "./session";

export function createActions(session: viewerSession.ViewerSession, elements: ViewerElements, queueRender: () => void): ViewerActions {
  const actions = {
    setIr: (ir: CoreIr, coverage?: CoverageOverlay, sourceMap?: SourceMapDocument) => {
      viewerSession.setIr(session, ir, coverage, sourceMap);
      actions.fit();
    },
    setCoverage: (coverage: CoverageOverlay) => {
      viewerSession.setCoverage(session, coverage);
      actions.syncPanels();
      queueRender();
    },
    setSourceMap: (sourceMap: SourceMapDocument) => {
      viewerSession.setSourceMap(session, sourceMap);
      actions.syncPanels();
    },
    openProjection: (projectionId: string) => {
      viewerSession.setProjection(session, projectionId);
      actions.fit();
    },
    openFsm: (fsmId: string) => {
      const projection = session.store.ir.projections?.find((candidate) => candidate.source === fsmId);
      if (projection) {
        actions.openProjection(projection.id);
      } else {
        actions.select(fsmId);
      }
    },
    select: (subject: string | undefined) => {
      session.store.selection.selected = subject;
      actions.syncPanels();
      queueRender();
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
      actions.syncPanels();
      queueRender();
    },
    setDiagnosticsVisible: (visible: boolean) => {
      session.store.showDiagnostics = visible;
      actions.syncPanels();
    },
    setFocusDepth: (depth: 1 | 2) => {
      session.store.focusDepth = depth;
      actions.syncPanels();
    },
    updateStatus: (world: Point) => {
      elements.status.textContent = `zoom ${session.store.camera.zoom.toFixed(2)} / world ${world.x.toFixed(0)}, ${world.y.toFixed(0)}`;
    },
    syncPanels: () => renderPanels(elements, session.store, actions),
    queueRender
  };
  return actions;
}
