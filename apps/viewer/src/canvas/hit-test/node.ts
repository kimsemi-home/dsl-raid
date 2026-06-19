import type { Point, SceneNode } from "../../types";
import type { HitIndex } from "../spatial-index";
import { nodesAt } from "../spatial-index";

export function nodeAt(index: HitIndex, point: Point): SceneNode | undefined {
  return nodesAt(index, point)
    .reverse()
    .find((node) => containsPoint(node, point));
}

function containsPoint(node: SceneNode, point: Point): boolean {
  return (
    point.x >= node.x &&
    point.x <= node.x + node.width &&
    point.y >= node.y &&
    point.y <= node.y + node.height
  );
}
