import type { AppStore } from "../store/app-store";
import type { CoverageStatus } from "../types";
import { escapeHtml } from "./html";

const order: CoverageStatus[] = ["covered", "uncovered", "failed", "flaky", "deployed", "not_deployed"];

export function renderCoverageSummary(element: HTMLElement, store: AppStore): void {
  const subjects = store.coverage?.subjects ?? [];
  if (!store.coverage || subjects.length === 0) {
    element.innerHTML = `<p class="muted">No coverage overlay loaded.</p>`;
    return;
  }
  const counts = countStatuses(subjects);
  const statuses = [
    ...order.filter((status) => counts[status]),
    ...Object.keys(counts)
      .filter((status) => !order.includes(status as CoverageStatus))
      .sort()
  ];
  element.innerHTML = `<div class="coverage-grid">${statuses.map((status) => item(status, counts[status] ?? 0)).join("")}</div>`;
}

function countStatuses(subjects: Array<{ status: string }>): Record<string, number> {
  return subjects.reduce<Record<string, number>>((accumulator, subject) => {
    accumulator[subject.status] = (accumulator[subject.status] ?? 0) + 1;
    return accumulator;
  }, {});
}

function item(label: string, count: number): string {
  return `<div class="coverage-pill ${escapeHtml(label)}"><strong>${count}</strong><span>${escapeHtml(label.replaceAll("_", " "))}</span></div>`;
}
