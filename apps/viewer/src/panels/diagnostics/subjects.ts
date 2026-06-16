import type { Diagnostic } from "../../types";

export type DiagnosticSelection = {
  subject: string;
  related: string[];
};

export function diagnosticSelection(diagnostic: Diagnostic): DiagnosticSelection {
  return {
    subject: diagnostic.id,
    related: diagnostic.subjects ?? []
  };
}

export function relatedAttribute(subjects: string[]): string {
  return subjects.join(" ");
}
