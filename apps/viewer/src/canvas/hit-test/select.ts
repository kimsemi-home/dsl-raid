import type { Point, ViewModel } from "../../types";
import { hitIndexFor } from "../spatial-index";
import { edgeAt } from "./edge";
import type { HitResult } from "./model";
import { nodeAt } from "./node";

export function hitTest(view: ViewModel, point: Point): HitResult {
  const index = hitIndexFor(view);
  const node = nodeAt(index, point);
  if (node) {
    return { kind: "node", subject: node.subject, id: node.id };
  }

  const edge = edgeAt(index, point);
  if (edge) {
    return { kind: "edge", subject: edge.subject, id: edge.id };
  }
  return undefined;
}
