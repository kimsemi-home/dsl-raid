import type { Camera, Point } from "../types";

export function screenToWorld(camera: Camera, point: Point): Point {
  return {
    x: (point.x - camera.panX) / camera.zoom,
    y: (point.y - camera.panY) / camera.zoom
  };
}

export function worldToScreen(camera: Camera, point: Point): Point {
  return {
    x: point.x * camera.zoom + camera.panX,
    y: point.y * camera.zoom + camera.panY
  };
}

export function zoomAt(camera: Camera, screen: Point, nextZoom: number): Camera {
  const zoom = Math.min(2.4, Math.max(0.35, nextZoom));
  const world = screenToWorld(camera, screen);
  return {
    zoom,
    panX: screen.x - world.x * zoom,
    panY: screen.y - world.y * zoom
  };
}
