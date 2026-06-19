import type { Point, SceneEdge } from "../../types";
import type { HitIndex } from "../spatial-index";
import { edgesNear } from "../spatial-index";
import { edgeDistance } from "./edge-distance";

const EDGE_RADIUS = 10;

export function edgeAt(index: HitIndex, point: Point): SceneEdge | undefined {
  return edgesNear(index, point, EDGE_RADIUS).find((edge) => {
    return edgeDistance(edge, point) < EDGE_RADIUS;
  });
}
