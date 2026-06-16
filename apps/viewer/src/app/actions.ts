import { zoomAt } from "../canvas/camera";
import { renderPanels } from "../panels/render";
import type { CoverageOverlay, CoreIr, Point } from "../types";
import type { ViewerElements } from "./elements";
import { fitGraph as fitCamera } from "./fit";
import { setCoverage as applyCoverage, setIr as applyIr, type ViewerSession } from "./session";

export type ViewerActions = {
  setIr: (ir: CoreIr, coverage?: CoverageOverlay) => void;
  setCoverage: (coverage: CoverageOverlay) => void;
  select: (subject: string | undefined) => void;
  hover: (subject: string | undefined) => void;
  pan: (dx: number, dy: number) => void;
  zoom: (point: Point, factor: number) => void;
  fit: () => void;
  setDiagnosticsVisible: (visible: boolean) => void;
  setFocusDepth: (depth: 1 | 2) => void;
  updateStatus: (world: Point) => void;
  syncPanels: () => void;
  queueRender: () => void;
};

export function createActions(session: ViewerSession, elements: ViewerElements, queueRender: () => void): ViewerActions {
  const actions = {
    setIr: (ir: CoreIr, coverage?: CoverageOverlay) => {
      applyIr(session, ir, coverage);
      actions.fit();
    },
    setCoverage: (coverage: CoverageOverlay) => {
      applyCoverage(session, coverage);
      actions.syncPanels();
      queueRender();
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
    syncPanels: () => renderPanels(elements, session.store, actions.select),
    queueRender
  };
  return actions;
}
