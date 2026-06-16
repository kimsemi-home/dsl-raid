import type { SceneNode } from "../types";
import type { TupleNode } from "./composition-types";
import { local } from "./composition-tuples";

const width = 280;
const height = 76;
const colGap = 330;
const rowGap = 132;

export function compositionNodes(nodes: TupleNode[]): SceneNode[] {
  return nodes
    .sort((left, right) => left.subject.localeCompare(right.subject))
    .map((node, index) => ({
      id: `layout:${local(node.subject)}`,
      subject: node.subject,
      x: 70 + (index % 3) * colGap,
      y: 90 + Math.floor(index / 3) * rowGap,
      width,
      height,
      label: label(node),
      badges: badges(node),
      style: {
        tone: node.terminal ? "success" : "default",
        emphasis: node.initial || node.terminal ? "strong" : "normal"
      }
    }));
}

function label(node: TupleNode): string {
  return node.members.map((member) => local(member).replace(".", "=")).join(" / ");
}

function badges(node: TupleNode): string[] {
  return ["tuple", ...(node.initial ? ["initial"] : []), ...(node.terminal ? ["terminal"] : [])];
}
