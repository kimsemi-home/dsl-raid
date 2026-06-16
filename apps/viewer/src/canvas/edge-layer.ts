import type { SceneEdge, SelectionState } from "../types";
import { drawLabel } from "./label";
import { activeLineWidth, activeStroke, subjectVisualState } from "./selection";

export function drawEdges(context: CanvasRenderingContext2D, edges: SceneEdge[], selection: SelectionState): void {
  for (const edge of edges) {
    if (edge.route.length < 2) {
      continue;
    }
    const state = subjectVisualState(selection, edge.subject);
    const tone = edge.style?.tone ?? "default";
    context.strokeStyle = activeStroke(tone, state);
    context.lineWidth = activeLineWidth(state);
    context.globalAlpha = edge.style?.emphasis === "faint" ? 0.45 : 1;
    context.beginPath();
    context.moveTo(edge.route[0].x, edge.route[0].y);
    for (const point of edge.route.slice(1)) {
      context.lineTo(point.x, point.y);
    }
    context.stroke();
    drawArrow(context, edge);
    drawEdgeLabel(context, edge);
    context.globalAlpha = 1;
  }
}

function drawArrow(context: CanvasRenderingContext2D, edge: SceneEdge): void {
  const end = edge.route[edge.route.length - 1];
  const previous = edge.route[edge.route.length - 2];
  const angle = Math.atan2(end.y - previous.y, end.x - previous.x);
  context.save();
  context.translate(end.x, end.y);
  context.rotate(angle);
  context.beginPath();
  context.moveTo(0, 0);
  context.lineTo(-10, -5);
  context.lineTo(-10, 5);
  context.closePath();
  context.fillStyle = context.strokeStyle;
  context.fill();
  context.restore();
}

function drawEdgeLabel(context: CanvasRenderingContext2D, edge: SceneEdge): void {
  if (!edge.label) {
    return;
  }
  const start = edge.route[0];
  const end = edge.route[edge.route.length - 1];
  drawLabel(context, edge.label, (start.x + end.x) / 2, (start.y + end.y) / 2 - 12);
}
