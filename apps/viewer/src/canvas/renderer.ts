import type { Camera, SelectionState, ViewModel } from "../types";
import { drawBackground } from "./background";
import { screenToWorld } from "./camera";
import { drawEdges } from "./edge-layer";
import { drawGrid } from "./grid";
import { visibleNodes } from "./hit-test";
import { drawNodes } from "./node-layer";

export function renderCanvas(canvas: HTMLCanvasElement, view: ViewModel, camera: Camera, selection: SelectionState): void {
  const context = canvas.getContext("2d");
  if (!context) {
    return;
  }
  const rect = resize(canvas);
  context.clearRect(0, 0, rect.width, rect.height);
  drawBackground(context, rect.width, rect.height);
  context.save();
  context.translate(camera.panX, camera.panY);
  context.scale(camera.zoom, camera.zoom);
  drawGrid(context, camera, rect.width, rect.height);
  drawEdges(context, view.edges, selection);
  drawNodes(context, visibleNodes(view, visibleBounds(camera, rect.width, rect.height)), selection);
  context.restore();
}

function resize(canvas: HTMLCanvasElement): DOMRect {
  const ratio = window.devicePixelRatio || 1;
  const rect = canvas.getBoundingClientRect();
  const width = Math.floor(rect.width * ratio);
  const height = Math.floor(rect.height * ratio);
  if (canvas.width !== width || canvas.height !== height) {
    canvas.width = width;
    canvas.height = height;
  }
  canvas.getContext("2d")?.setTransform(ratio, 0, 0, ratio, 0, 0);
  return rect;
}

function visibleBounds(camera: Camera, width: number, height: number) {
  const topLeft = screenToWorld(camera, { x: 0, y: 0 });
  const bottomRight = screenToWorld(camera, { x: width, y: height });
  return {
    x: topLeft.x - 200,
    y: topLeft.y - 200,
    width: bottomRight.x - topLeft.x + 400,
    height: bottomRight.y - topLeft.y + 400
  };
}
