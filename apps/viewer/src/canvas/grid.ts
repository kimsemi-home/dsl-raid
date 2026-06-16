import type { Camera } from "../types";
import { screenToWorld } from "./camera";

export function drawGrid(context: CanvasRenderingContext2D, camera: Camera, width: number, height: number): void {
  const spacing = 40;
  const start = screenToWorld(camera, { x: 0, y: 0 });
  const end = screenToWorld(camera, { x: width, y: height });
  context.strokeStyle = "#ddd8cc";
  context.lineWidth = 1 / camera.zoom;
  context.beginPath();
  for (let x = Math.floor(start.x / spacing) * spacing; x < end.x; x += spacing) {
    context.moveTo(x, start.y);
    context.lineTo(x, end.y);
  }
  for (let y = Math.floor(start.y / spacing) * spacing; y < end.y; y += spacing) {
    context.moveTo(start.x, y);
    context.lineTo(end.x, y);
  }
  context.stroke();
}
