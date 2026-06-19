import type { Point, SceneEdge } from "../../types";

export function edgeDistance(edge: SceneEdge, point: Point): number {
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
  const t = Math.max(0, Math.min(1, projection(point, start, dx, dy, lengthSquared)));
  return distance(point, { x: start.x + t * dx, y: start.y + t * dy });
}

function projection(point: Point, start: Point, dx: number, dy: number, lengthSquared: number) {
  return ((point.x - start.x) * dx + (point.y - start.y) * dy) / lengthSquared;
}

function distance(a: Point, b: Point): number {
  return Math.hypot(a.x - b.x, a.y - b.y);
}
