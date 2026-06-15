import type { Camera, SceneEdge, SceneNode, SelectionState, ViewModel } from "../types";
import { screenToWorld } from "./camera";
import { visibleNodes } from "./hit-test";

export function renderCanvas(canvas: HTMLCanvasElement, view: ViewModel, camera: Camera, selection: SelectionState): void {
  const context = canvas.getContext("2d");
  if (!context) {
    return;
  }
  const ratio = window.devicePixelRatio || 1;
  const rect = canvas.getBoundingClientRect();
  if (canvas.width !== Math.floor(rect.width * ratio) || canvas.height !== Math.floor(rect.height * ratio)) {
    canvas.width = Math.floor(rect.width * ratio);
    canvas.height = Math.floor(rect.height * ratio);
  }
  context.setTransform(ratio, 0, 0, ratio, 0, 0);
  context.clearRect(0, 0, rect.width, rect.height);
  drawBackground(context, rect.width, rect.height);
  context.save();
  context.translate(camera.panX, camera.panY);
  context.scale(camera.zoom, camera.zoom);
  drawGrid(context, camera, rect.width, rect.height);
  drawEdges(context, view.edges, selection);
  const topLeft = screenToWorld(camera, { x: 0, y: 0 });
  const bottomRight = screenToWorld(camera, { x: rect.width, y: rect.height });
  drawNodes(
    context,
    visibleNodes(view, {
      x: topLeft.x - 200,
      y: topLeft.y - 200,
      width: bottomRight.x - topLeft.x + 400,
      height: bottomRight.y - topLeft.y + 400
    }),
    selection
  );
  context.restore();
}

function drawBackground(context: CanvasRenderingContext2D, width: number, height: number): void {
  context.fillStyle = "#f7f7f2";
  context.fillRect(0, 0, width, height);
}

function drawGrid(context: CanvasRenderingContext2D, camera: Camera, width: number, height: number): void {
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

function drawEdges(context: CanvasRenderingContext2D, edges: SceneEdge[], selection: SelectionState): void {
  for (const edge of edges) {
    if (edge.route.length < 2) {
      continue;
    }
    const selected = selection.selected === edge.subject;
    const hovered = selection.hovered === edge.subject;
    context.strokeStyle = selected ? "#0f766e" : hovered ? "#b45309" : "#58616a";
    context.lineWidth = selected || hovered ? 3 : 2;
    context.beginPath();
    context.moveTo(edge.route[0].x, edge.route[0].y);
    for (const point of edge.route.slice(1)) {
      context.lineTo(point.x, point.y);
    }
    context.stroke();
    drawArrow(context, edge);
    if (edge.label) {
      const start = edge.route[0];
      const end = edge.route[edge.route.length - 1];
      drawLabel(context, edge.label, (start.x + end.x) / 2, (start.y + end.y) / 2 - 12);
    }
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

function drawNodes(context: CanvasRenderingContext2D, nodes: SceneNode[], selection: SelectionState): void {
  for (const node of nodes) {
    const selected = selection.selected === node.subject;
    const hovered = selection.hovered === node.subject;
    const tone = node.style?.tone ?? "default";
    context.fillStyle = "#fffefa";
    context.strokeStyle = selected ? "#0f766e" : hovered ? "#b45309" : toneStroke(tone);
    context.lineWidth = selected || hovered || node.style?.emphasis === "strong" ? 3 : 2;
    roundedRect(context, node.x, node.y, node.width, node.height, 8);
    context.fill();
    context.stroke();

    context.fillStyle = "#171717";
    context.font = "700 15px Inter, ui-sans-serif, system-ui";
    context.fillText(node.label, node.x + 14, node.y + 24);

    let badgeX = node.x + 14;
    context.font = "10px Inter, ui-sans-serif, system-ui";
    for (const badge of node.badges.slice(0, 3)) {
      const badgeWidth = Math.max(46, context.measureText(badge).width + 16);
      context.fillStyle = badgeFill(tone, badge);
      roundedRect(context, badgeX, node.y + 34, badgeWidth, 18, 6);
      context.fill();
      context.fillStyle = "#2f3437";
      context.fillText(badge, badgeX + 8, node.y + 47);
      badgeX += badgeWidth + 6;
    }
  }
}

function drawLabel(context: CanvasRenderingContext2D, label: string, x: number, y: number): void {
  context.font = "12px Inter, ui-sans-serif, system-ui";
  const width = context.measureText(label).width + 14;
  context.fillStyle = "rgba(255, 254, 250, 0.92)";
  roundedRect(context, x - width / 2, y - 13, width, 20, 6);
  context.fill();
  context.fillStyle = "#334155";
  context.fillText(label, x - width / 2 + 7, y + 2);
}

function toneStroke(tone: string): string {
  switch (tone) {
    case "success":
      return "#0f766e";
    case "warning":
      return "#b45309";
    case "danger":
      return "#b91c1c";
    case "muted":
      return "#737373";
    default:
      return "#404040";
  }
}

function badgeFill(tone: string, badge: string): string {
  if (badge === "generated") {
    return "#dbeafe";
  }
  if (badge === "tested" || tone === "success") {
    return "#ccfbf1";
  }
  if (tone === "danger") {
    return "#fee2e2";
  }
  return "#ebe6da";
}

function roundedRect(context: CanvasRenderingContext2D, x: number, y: number, width: number, height: number, radius: number): void {
  const r = Math.min(radius, width / 2, height / 2);
  context.beginPath();
  context.moveTo(x + r, y);
  context.arcTo(x + width, y, x + width, y + height, r);
  context.arcTo(x + width, y + height, x, y + height, r);
  context.arcTo(x, y + height, x, y, r);
  context.arcTo(x, y, x + width, y, r);
  context.closePath();
}
