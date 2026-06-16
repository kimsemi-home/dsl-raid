import type { SceneEdge, SceneNode, ViewModel } from "../types";

export type TerminalPathStep = {
  subject: string;
  label: string;
  kind: "state" | "transition";
};

export function terminalPath(view: ViewModel, selected: string | undefined): TerminalPathStep[] {
  const edge = view.edges.find((candidate) => candidate.subject === selected);
  if (edge) {
    return [edgeStep(edge), ...nodePath(view, edge.to, new Set())];
  }
  const node = view.nodes.find((candidate) => candidate.subject === selected);
  return node ? nodePath(view, node.id, new Set()) : [];
}

function nodePath(view: ViewModel, nodeId: string, visited: Set<string>): TerminalPathStep[] {
  const node = view.nodes.find((candidate) => candidate.id === nodeId);
  if (!node || visited.has(node.id)) {
    return [];
  }
  visited.add(node.id);
  const edge = firstOutgoing(view, node.id);
  if (!edge) {
    return [nodeStep(node)];
  }
  return [nodeStep(node), edgeStep(edge), ...nodePath(view, edge.to, visited)];
}

function firstOutgoing(view: ViewModel, nodeId: string): SceneEdge | undefined {
  return view.edges
    .filter((edge) => edge.from === nodeId)
    .sort((a, b) => a.subject.localeCompare(b.subject))[0];
}

function nodeStep(node: SceneNode): TerminalPathStep {
  return { subject: node.subject, label: node.label, kind: "state" };
}

function edgeStep(edge: SceneEdge): TerminalPathStep {
  return { subject: edge.subject, label: edge.label ?? edge.subject, kind: "transition" };
}
