import type { SceneEdge, SceneNode } from "../../types";

export type HitIndex = {
  cellSize: number;
  nodeCells: Map<string, Set<SceneNode>>;
  edgeCells: Map<string, Set<SceneEdge>>;
  nodes: SceneNode[];
  edges: SceneEdge[];
};
