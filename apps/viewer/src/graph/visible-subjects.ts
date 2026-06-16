import type { ViewModel } from "../types";

export type VisibleSubject = {
  subject: string;
  label: string;
  kind: string;
};

export type VisibleSubjectSummary = {
  states: number;
  transitions: number;
};

export function visibleSubjects(view: ViewModel): VisibleSubject[] {
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

export function visibleSubjectSummary(view: ViewModel): VisibleSubjectSummary {
  return {
    states: view.nodes.length,
    transitions: view.edges.length
  };
}
