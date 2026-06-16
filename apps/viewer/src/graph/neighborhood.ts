import type { SceneEdge, ViewModel } from "../types";

export function focusedView(view: ViewModel, subject: string | undefined, depth: 1 | 2): ViewModel {
  const start = startNodes(view, subject);
  if (start.size === 0) {
    return view;
  }
  const visibleNodes = expandNodes(view.edges, start, depth);
  const edges = view.edges.filter((edge) => visibleNodes.has(edge.from) && visibleNodes.has(edge.to));
  const nodes = view.nodes.filter((node) => visibleNodes.has(node.id));
  return { ...view, nodes, edges };
}

function startNodes(view: ViewModel, subject: string | undefined): Set<string> {
  const nodes = new Set<string>();
  const node = view.nodes.find((candidate) => candidate.subject === subject);
  if (node) {
    nodes.add(node.id);
    return nodes;
  }
  const edge = view.edges.find((candidate) => candidate.subject === subject);
  if (edge) {
    nodes.add(edge.from);
    nodes.add(edge.to);
  }
  return nodes;
}

function expandNodes(edges: SceneEdge[], start: Set<string>, depth: 1 | 2): Set<string> {
  const visible = new Set(start);
  let frontier = [...start];
  for (let step = 0; step < depth; step += 1) {
    const next = new Set<string>();
    for (const edge of edges) {
      collectNeighbor(edge, frontier, next);
    }
    frontier = [...next].filter((id) => !visible.has(id));
    frontier.forEach((id) => visible.add(id));
  }
  return visible;
}

function collectNeighbor(edge: SceneEdge, frontier: string[], next: Set<string>): void {
  if (frontier.includes(edge.from)) {
    next.add(edge.to);
  }
  if (frontier.includes(edge.to)) {
    next.add(edge.from);
  }
}
