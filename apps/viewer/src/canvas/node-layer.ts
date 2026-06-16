import type { SceneNode, SelectionState } from "../types";
import { roundedRect } from "./shape";
import { badgeFill, toneStroke } from "./style";

export function drawNodes(context: CanvasRenderingContext2D, nodes: SceneNode[], selection: SelectionState): void {
  for (const node of nodes) {
    const selected = selection.selected === node.subject;
    const hovered = selection.hovered === node.subject;
    const tone = node.style?.tone ?? "default";
    context.globalAlpha = node.style?.emphasis === "faint" ? 0.68 : 1;
    context.fillStyle = "#fffefa";
    context.strokeStyle = selected ? "#0f766e" : hovered ? "#b45309" : toneStroke(tone);
    context.lineWidth = selected || hovered || node.style?.emphasis === "strong" ? 3 : 2;
    roundedRect(context, node.x, node.y, node.width, node.height, 8);
    context.fill();
    context.stroke();
    drawNodeText(context, node);
    drawBadges(context, node, tone);
    context.globalAlpha = 1;
  }
}

function drawNodeText(context: CanvasRenderingContext2D, node: SceneNode): void {
  context.fillStyle = "#171717";
  context.font = "700 15px Inter, ui-sans-serif, system-ui";
  context.fillText(node.label, node.x + 14, node.y + 24);
}

function drawBadges(context: CanvasRenderingContext2D, node: SceneNode, tone: string): void {
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
