import type { Point, SceneEdge, SceneNode } from "../../types";
import { collect, type Rect } from "../spatial-cells";
import { pointRect } from "./geometry";
import type { HitIndex } from "./model";

export function nodesAt(index: HitIndex, point: Point): SceneNode[] {
  const candidates = collect(index.nodeCells, pointRect(point, 0), index.cellSize);
  return index.nodes.filter((node) => candidates.has(node));
}

export function edgesNear(index: HitIndex, point: Point, radius: number): SceneEdge[] {
  const candidates = collect(index.edgeCells, pointRect(point, radius), index.cellSize);
  return index.edges.filter((edge) => candidates.has(edge));
}

export function nodesInBounds(index: HitIndex, bounds: Rect): SceneNode[] {
  const candidates = collect(index.nodeCells, bounds, index.cellSize);
  return index.nodes.filter((node) => candidates.has(node));
}
