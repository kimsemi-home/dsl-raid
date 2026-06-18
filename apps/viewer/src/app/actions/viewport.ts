import { zoomAt } from "../../canvas/camera";
import type { Point } from "../../types";
import type { ViewerActions } from "../action-types";
import type { ViewerElements } from "../elements";
import { fitGraph as fitCamera } from "../fit";
import type { ViewerSession } from "../session";

type ViewportActions = Pick<ViewerActions, "fit" | "hover" | "pan" | "updateStatus" | "zoom">;

export function viewportActions(
  session: ViewerSession,
  elements: ViewerElements,
  refresh: () => void,
  queueRender: () => void
): ViewportActions {
  return {
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
    updateStatus: (world: Point) => {
      elements.status.textContent = `zoom ${session.store.camera.zoom.toFixed(2)} / world ${world.x.toFixed(0)}, ${world.y.toFixed(0)}`;
    }
  };
}
