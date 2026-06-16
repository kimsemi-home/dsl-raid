import type { InspectorPanel, SourceMapDocument } from "../../types";
import { escapeHtml } from "../html";
import { bindSubjectButtons, type SelectSubject } from "../subject-buttons";
import { sourceMapSection } from "./source-map";

export function renderInspector(
  element: HTMLElement,
  panel: InspectorPanel | undefined,
  sourceMap: SourceMapDocument | undefined,
  onSelect: SelectSubject
): void {
  element.innerHTML = panel ? panelHtml(panel, sourceMap) : emptyHtml();
  bindSubjectButtons(element, onSelect);
}

function emptyHtml(): string {
  return `<p class="muted">Select a state or transition to inspect source, tests, generated artifacts, policies, and diagnostics.</p>`;
}

function panelHtml(panel: InspectorPanel, sourceMap: SourceMapDocument | undefined): string {
  const sections = appendSourceMapSection(panel, sourceMap);
  return `
    <div class="subject">${escapeHtml(panel.subject)}</div>
    <h3>${escapeHtml(panel.title)}</h3>
    ${sections.map(sectionHtml).join("")}
  `;
}

function appendSourceMapSection(panel: InspectorPanel, sourceMap: SourceMapDocument | undefined): InspectorPanel["sections"] {
  const section = sourceMapSection(panel.subject, sourceMap);
  return section ? [...panel.sections, section] : panel.sections;
}

function sectionHtml(section: InspectorPanel["sections"][number]): string {
  return `
    <section class="inspector-section">
      <h4>${escapeHtml(section.title)}</h4>
      ${section.rows.map(rowHtml).join("")}
    </section>
  `;
}

function rowHtml(row: InspectorPanel["sections"][number]["rows"][number]): string {
  const value = row.subject
    ? `<button data-subject="${escapeHtml(row.subject)}">${escapeHtml(row.value)}</button>`
    : `<strong>${escapeHtml(row.value)}</strong>`;
  return `<div class="row"><span>${escapeHtml(row.label)}</span>${value}</div>`;
}
