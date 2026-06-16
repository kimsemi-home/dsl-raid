import type { CoverageOverlay, CoverageSubject, InspectorRow } from "../types";

export function coverageIndex(coverage?: CoverageOverlay): Map<string, CoverageSubject> {
  return new Map((coverage?.subjects ?? []).map((subject) => [subject.subject, subject]));
}

export function coverageBadges(coverage?: CoverageSubject): string[] {
  if (!coverage) {
    return [];
  }
  const badges: string[] = [coverage.status];
  if ((coverage.count ?? 0) > 0) {
    badges.push(`seen ${coverage.count}`);
  }
  if ((coverage.failure_rate ?? 0) > 0) {
    badges.push(`fail ${formatRate(coverage.failure_rate)}`);
  }
  return badges;
}

export function coverageLabel(label: string, coverage?: CoverageSubject): string {
  if (!coverage || (coverage.failure_rate ?? 0) <= 0) {
    return label;
  }
  return `${label} · fail ${formatRate(coverage.failure_rate)}`;
}

export function coverageTone(coverage?: CoverageSubject): "success" | "warning" | "danger" | "muted" | undefined {
  switch (coverage?.status) {
    case "covered":
    case "deployed":
      return "success";
    case "failed":
      return "danger";
    case "flaky":
      return "warning";
    case "uncovered":
    case "not_deployed":
      return "muted";
    default:
      return undefined;
  }
}

export function coverageRows(coverage?: CoverageSubject): InspectorRow[] {
  if (!coverage) {
    return [{ label: "Status", value: "not loaded" }];
  }
  return [
    { label: "Status", value: coverage.status },
    { label: "Count", value: String(coverage.count ?? 0) },
    { label: "Failure rate", value: formatRate(coverage.failure_rate) },
    { label: "Last seen", value: coverage.last_seen ?? "never" }
  ];
}

function formatRate(value: number | undefined): string {
  if (value === undefined) {
    return "n/a";
  }
  return `${Math.round(value * 1000) / 10}%`;
}
