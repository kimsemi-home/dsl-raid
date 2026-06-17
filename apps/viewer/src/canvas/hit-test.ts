import type { Point, SceneEdge, SceneNode, ViewModel } from "../types";
import { edgesNear, hitIndexFor, nodesAt, nodesInBounds } from "./spatial-index";

export type HitResult =
  | { kind: "node"; subject: string; id: string }
  | { kind: "edge"; subject: string; id: string }
  | undefined;

export function hitTest(view: ViewModel, point: Point): HitResult {
  const index = hitIndexFor(view);
  const node = nodesAt(index, point)
    .reverse()
    .find((candidate) => point.x >= candidate.x && point.x <= candidate.x + candidate.width && point.y >= candidate.y && point.y <= candidate.y + candidate.height);
  if (node) {
    return { kind: "node", subject: node.subject, id: node.id };
  }

  const edge = edgesNear(index, point, 10).find((candidate) => edgeDistance(candidate, point) < 10);
  if (edge) {
    return { kind: "edge", subject: edge.subject, id: edge.id };
  }
  return undefined;
}

function edgeDistance(edge: SceneEdge, point: Point): number {
  let best = Number.POSITIVE_INFINITY;
  for (let index = 0; index < edge.route.length - 1; index += 1) {
    best = Math.min(best, segmentDistance(point, edge.route[index], edge.route[index + 1]));
  }
  return best;
}

function segmentDistance(point: Point, start: Point, end: Point): number {
  const dx = end.x - start.x;
  const dy = end.y - start.y;
  const lengthSquared = dx * dx + dy * dy;
  if (lengthSquared === 0) {
    return distance(point, start);
  }
  const t = Math.max(0, Math.min(1, ((point.x - start.x) * dx + (point.y - start.y) * dy) / lengthSquared));
  return distance(point, { x: start.x + t * dx, y: start.y + t * dy });
}

function distance(a: Point, b: Point): number {
  return Math.hypot(a.x - b.x, a.y - b.y);
}

export function visibleNodes(view: ViewModel, bounds: { x: number; y: number; width: number; height: number }): SceneNode[] {
  return nodesInBounds(hitIndexFor(view), bounds).filter((node) => node.x + node.width >= bounds.x && node.x <= bounds.x + bounds.width && node.y + node.height >= bounds.y && node.y <= bounds.y + bounds.height);
}
