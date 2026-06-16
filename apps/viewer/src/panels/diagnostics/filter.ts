import type { Diagnostic, DiagnosticSeverityFilter } from "../../types";

export function filterDiagnostics(
  diagnostics: Diagnostic[],
  filter: DiagnosticSeverityFilter
): Diagnostic[] {
  if (filter === "all") {
    return diagnostics;
  }
  return diagnostics.filter((diagnostic) => diagnostic.severity === filter);
}

export function diagnosticFilterValue(value: string): DiagnosticSeverityFilter {
  return isDiagnosticFilter(value) ? value : "all";
}

function isDiagnosticFilter(value: string): value is DiagnosticSeverityFilter {
  return ["all", "hint", "info", "warning", "error"].includes(value);
}
