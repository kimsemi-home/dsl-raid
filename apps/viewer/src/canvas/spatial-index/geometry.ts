import type { Point, SceneEdge } from "../../types";
import type { Rect } from "../spatial-cells";

export function edgeBounds(edge: SceneEdge): Rect {
  if (edge.route.length === 0) {
    return { x: 0, y: 0, width: 0, height: 0 };
  }
  const xs = edge.route.map((point) => point.x);
  const ys = edge.route.map((point) => point.y);
  const x = Math.min(...xs);
  const y = Math.min(...ys);
  return { x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y };
}

export function pointRect(point: Point, radius: number): Rect {
  return {
    x: point.x - radius,
    y: point.y - radius,
    width: radius * 2,
    height: radius * 2
  };
}
