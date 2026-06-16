import { createInitialCamera } from "../store/app-store";
import type { Camera, ViewModel } from "../types";

export function fitGraph(view: ViewModel, canvas: HTMLCanvasElement): Camera {
  if (view.nodes.length === 0) {
    return createInitialCamera();
  }
  const minX = Math.min(...view.nodes.map((node) => node.x));
  const minY = Math.min(...view.nodes.map((node) => node.y));
  const maxX = Math.max(...view.nodes.map((node) => node.x + node.width));
  const maxY = Math.max(...view.nodes.map((node) => node.y + node.height));
  const graphWidth = Math.max(1, maxX - minX);
  const graphHeight = Math.max(1, maxY - minY);
  const availableWidth = Math.max(1, canvas.clientWidth - 80);
  const availableHeight = Math.max(1, canvas.clientHeight - 80);
  const zoom = Math.min(1.35, Math.max(0.45, Math.min(availableWidth / graphWidth, availableHeight / graphHeight)));
  return {
    zoom,
    panX: 40 - minX * zoom,
    panY: 40 - minY * zoom
  };
}
