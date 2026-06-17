import { subjectsForWorkspaceSearch } from "../../graph/workspace-search";
import type { AppStore } from "../../store/app-store";
import { escapeHtml } from "../html";
import { bindSubjectButtons, type SelectSubject } from "../subject-buttons";

export function renderSearch(input: HTMLInputElement, results: HTMLElement, store: AppStore, onSelect: SelectSubject): void {
  const query = input.value.trim().toLowerCase();
  const subjects = subjectsForWorkspaceSearch(store)
    .filter((item) => !query || item.subject.toLowerCase().includes(query) || item.label.toLowerCase().includes(query))
    .slice(0, 16);
  results.innerHTML = subjects.map(itemHtml).join("");
  bindSubjectButtons(results, onSelect);
}

function itemHtml(item: { subject: string; label: string; kind: string }): string {
  return `
    <button data-subject="${escapeHtml(item.subject)}">
      <span>${escapeHtml(item.label)}</span>
      <small>${escapeHtml(item.kind)}</small>
    </button>
  `;
}
