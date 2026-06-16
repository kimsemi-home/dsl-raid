import type { ViewModel } from "../types";

export function followSubject(view: ViewModel, selected: string | undefined): string | undefined {
  const node = view.nodes.find((candidate) => candidate.subject === selected);
  if (node) {
    return firstOutgoing(view, node.id);
  }
  const edge = view.edges.find((candidate) => candidate.subject === selected);
  if (edge) {
    return view.nodes.find((candidate) => candidate.id === edge.to)?.subject;
  }
  return undefined;
}

function firstOutgoing(view: ViewModel, nodeId: string): string | undefined {
  return view.edges
    .filter((edge) => edge.from === nodeId)
    .sort((a, b) => a.subject.localeCompare(b.subject))[0]?.subject;
}
