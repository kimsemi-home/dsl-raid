import type { ViewModel } from "../../types";
import { insert } from "../spatial-cells";
import { CELL_SIZE } from "./constants";
import { edgeBounds } from "./geometry";
import type { HitIndex } from "./model";

export function buildHitIndex(view: ViewModel): HitIndex {
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
