import type { SceneNode, ViewModel } from "../../types";
import { hitIndexFor, nodesInBounds } from "../spatial-index";
import type { Rect } from "../spatial-cells";

export function visibleNodes(view: ViewModel, bounds: Rect): SceneNode[] {
  return nodesInBounds(hitIndexFor(view), bounds).filter((node) => {
    return intersects(node, bounds);
  });
}

function intersects(node: SceneNode, bounds: Rect): boolean {
  return (
    node.x + node.width >= bounds.x &&
    node.x <= bounds.x + bounds.width &&
    node.y + node.height >= bounds.y &&
    node.y <= bounds.y + bounds.height
  );
}
