import { screenToWorld } from "../canvas/camera";
import { hitTest } from "../canvas/hit-test";
import type { Point } from "../types";
import type { ViewerActions } from "../app/actions";
import type { ViewerElements } from "../app/elements";
import type { ViewerSession } from "../app/session";
import { relativePoint } from "./point";

export function bindCanvasControls(elements: ViewerElements, session: ViewerSession, actions: ViewerActions): void {
  let dragging = false;
  let lastMouse: Point | undefined;
  elements.canvas.addEventListener("pointerdown", (event) => {
    elements.canvas.setPointerCapture(event.pointerId);
    dragging = true;
    lastMouse = { x: event.clientX, y: event.clientY };
  });
  elements.canvas.addEventListener("pointerup", (event) => {
    elements.canvas.releasePointerCapture(event.pointerId);
    dragging = false;
    const hit = hitAt(elements, session, event);
    actions.select(hit?.subject);
  });
  elements.canvas.addEventListener("pointermove", (event) => {
    const point = relativePoint(elements.canvas, event);
    const world = screenToWorld(session.store.camera, point);
    const hit = hitTest(session.store.view, world);
    actions.hover(hit?.subject);
    if (dragging && lastMouse && !hit) {
      actions.pan(event.clientX - lastMouse.x, event.clientY - lastMouse.y);
      lastMouse = { x: event.clientX, y: event.clientY };
    }
    actions.updateStatus(world);
  });
  elements.canvas.addEventListener(
    "wheel",
    (event) => {
      event.preventDefault();
      const factor = event.deltaY > 0 ? 0.92 : 1.08;
      actions.zoom(relativePoint(elements.canvas, event), factor);
    },
    { passive: false }
  );
}

function hitAt(elements: ViewerElements, session: ViewerSession, event: PointerEvent) {
  const point = relativePoint(elements.canvas, event);
  return hitTest(session.store.view, screenToWorld(session.store.camera, point));
}
