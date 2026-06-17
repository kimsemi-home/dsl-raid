import type { Point, SceneEdge, SceneNode, ViewModel } from "../types";
import { collect, insert, type Rect } from "./spatial-cells";

export type HitIndex = {
  cellSize: number;
  nodeCells: Map<string, Set<SceneNode>>;
  edgeCells: Map<string, Set<SceneEdge>>;
  nodes: SceneNode[];
  edges: SceneEdge[];
};

const CELL_SIZE = 256;
const cache = new WeakMap<ViewModel, HitIndex>();

export function hitIndexFor(view: ViewModel): HitIndex {
  const cached = cache.get(view);
  if (cached) {
    return cached;
  }
  const index = buildHitIndex(view);
  cache.set(view, index);
  return index;
}

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

function buildHitIndex(view: ViewModel): HitIndex {
  const index: HitIndex = {
    cellSize: CELL_SIZE,
    nodeCells: new Map(),
    edgeCells: new Map(),
    nodes: view.nodes,
    edges: view.edges
  };
  view.nodes.forEach((node) => insert(index.nodeCells, node, node, index.cellSize));
  view.edges.forEach((edge) => insert(index.edgeCells, edgeBounds(edge), edge, index.cellSize));
  return index;
}

function edgeBounds(edge: SceneEdge): Rect {
  if (edge.route.length === 0) {
    return { x: 0, y: 0, width: 0, height: 0 };
  }
  const xs = edge.route.map((point) => point.x);
  const ys = edge.route.map((point) => point.y);
  const x = Math.min(...xs);
  const y = Math.min(...ys);
  return { x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y };
}

function pointRect(point: Point, radius: number): Rect {
  return { x: point.x - radius, y: point.y - radius, width: radius * 2, height: radius * 2 };
}
