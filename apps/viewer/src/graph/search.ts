import type { ViewModel } from "../types";

export type SearchSubject = {
  subject: string;
  label: string;
  kind: string;
};

export function subjectsForSearch(view: ViewModel): SearchSubject[] {
  const nodes = view.nodes.map((node) => ({
    subject: node.subject,
    label: node.label,
    kind: "state"
  }));
  const edges = view.edges.map((edge) => ({
    subject: edge.subject,
    label: edge.label ?? edge.subject,
    kind: "transition"
  }));
  return [...nodes, ...edges].sort((a, b) => a.subject.localeCompare(b.subject));
}
