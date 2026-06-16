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
  const panels = view.inspector_panels.map((panel) => ({
    subject: panel.subject,
    label: panel.title,
    kind: panelKind(panel.subject)
  }));
  return unique([...nodes, ...edges, ...panels]).sort((a, b) => a.subject.localeCompare(b.subject));
}

function unique(subjects: SearchSubject[]): SearchSubject[] {
  const seen = new Set<string>();
  return subjects.filter((item) => {
    if (seen.has(item.subject)) {
      return false;
    }
    seen.add(item.subject);
    return true;
  });
}

function panelKind(subject: string): string {
  return subject.includes(":") ? subject.split(":")[0] : "subject";
}
