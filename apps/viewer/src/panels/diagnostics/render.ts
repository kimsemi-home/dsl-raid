import type { AppStore } from "../../store/app-store";
import type { Diagnostic } from "../../types";
import { escapeHtml } from "../html";
import { bindSubjectButtons, type SelectSubject } from "../subject-buttons";
import { diagnosticSelection, relatedAttribute } from "./subjects";

export function renderDiagnostics(element: HTMLElement, store: AppStore, onSelect: SelectSubject): void {
  if (!store.showDiagnostics) {
    element.innerHTML = `<p class="muted">Diagnostics hidden.</p>`;
    return;
  }
  const diagnostics = store.ir.diagnostics ?? [];
  if (diagnostics.length === 0) {
    element.innerHTML = `<p class="muted">No diagnostics in this IR. CLI validation can still produce assertion-level reports.</p>`;
    return;
  }
  element.innerHTML = diagnostics.map((item) => diagnosticHtml(item, store.selection.selected)).join("");
  bindSubjectButtons(element, onSelect);
}

function diagnosticHtml(diagnostic: Diagnostic, selected?: string): string {
  const selection = diagnosticSelection(diagnostic);
  const active = selected === diagnostic.id ? " active" : "";
  const suggestion = diagnostic.suggestion ? `<small>${escapeHtml(diagnostic.suggestion)}</small>` : "";
  return `
    <button class="diagnostic ${diagnostic.severity}${active}"
      data-subject="${escapeHtml(selection.subject)}"
      data-related-subjects="${escapeHtml(relatedAttribute(selection.related))}">
      <strong>${escapeHtml(diagnostic.code)}</strong>
      <span>${escapeHtml(diagnostic.message)}</span>
      ${suggestion}
    </button>
  `;
}
