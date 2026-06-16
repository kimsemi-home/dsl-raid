import type { AppStore } from "../../store/app-store";
import type { Diagnostic } from "../../types";
import { escapeHtml } from "../html";

export function renderDiagnostics(element: HTMLElement, store: AppStore): void {
  if (!store.showDiagnostics) {
    element.innerHTML = `<p class="muted">Diagnostics hidden.</p>`;
    return;
  }
  const diagnostics = store.ir.diagnostics ?? [];
  if (diagnostics.length === 0) {
    element.innerHTML = `<p class="muted">No diagnostics in this IR. CLI validation can still produce assertion-level reports.</p>`;
    return;
  }
  element.innerHTML = diagnostics.map(diagnosticHtml).join("");
}

function diagnosticHtml(diagnostic: Diagnostic): string {
  return `
    <button class="diagnostic ${diagnostic.severity}" data-subject="${escapeHtml(diagnostic.subjects?.[0] ?? "")}">
      <strong>${escapeHtml(diagnostic.code)}</strong>
      <span>${escapeHtml(diagnostic.message)}</span>
    </button>
  `;
}
