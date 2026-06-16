import type { Diagnostic, StyleToken } from "../types";

export type DiagnosticMark = {
  badge: string;
  tone: NonNullable<StyleToken["tone"]>;
  rank: number;
};

const severity: Record<Diagnostic["severity"], DiagnosticMark> = {
  error: { badge: "diag:error", tone: "danger", rank: 4 },
  warning: { badge: "diag:warn", tone: "warning", rank: 3 },
  info: { badge: "diag:info", tone: "muted", rank: 2 },
  hint: { badge: "diag:hint", tone: "muted", rank: 1 }
};

export function diagnosticMarks(diagnostics?: Diagnostic[]): Map<string, DiagnosticMark> {
  const marks = new Map<string, DiagnosticMark>();
  for (const diagnostic of diagnostics ?? []) {
    for (const subject of diagnostic.subjects ?? []) {
      mergeMark(marks, subject, severity[diagnostic.severity]);
    }
  }
  return marks;
}

export function diagnosticBadges(mark?: DiagnosticMark): string[] {
  return mark ? [mark.badge] : [];
}

export function diagnosticTone(mark?: DiagnosticMark): DiagnosticMark["tone"] | undefined {
  return mark?.tone;
}

function mergeMark(marks: Map<string, DiagnosticMark>, subject: string, mark: DiagnosticMark): void {
  const current = marks.get(subject);
  if (!current || mark.rank > current.rank) {
    marks.set(subject, mark);
  }
}
