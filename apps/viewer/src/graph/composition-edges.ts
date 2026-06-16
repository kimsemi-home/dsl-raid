import type { SceneEdge, SceneNode } from "../types";
import type { TupleEdge } from "./composition-types";
import { local } from "./composition-tuples";

export function compositionEdges(edges: TupleEdge[], nodes: SceneNode[]): SceneEdge[] {
  return edges
    .sort((left, right) => left.subject.localeCompare(right.subject))
    .flatMap((edge) => {
      const from = nodes.find((node) => node.subject === edge.from);
      const to = nodes.find((node) => node.subject === edge.to);
      if (!from || !to) {
        return [];
      }
      return [{
        id: `layout:${local(edge.subject)}`,
        subject: edge.subject,
        from: from.id,
        to: to.id,
        label: edge.event ? local(edge.event) : local(edge.members[0] ?? "epsilon"),
        route: [
          { x: from.x + from.width, y: from.y + from.height / 2 },
          { x: to.x, y: to.y + to.height / 2 }
        ],
        style: { tone: "default", emphasis: "normal" }
      }];
    });
}
