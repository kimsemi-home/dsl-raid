import { mustQuery } from "./dom";

export type ViewerElements = {
  canvas: HTMLCanvasElement;
  status: HTMLDivElement;
  inspector: HTMLDivElement;
  projectTree: HTMLDivElement;
  visibleSubjects: HTMLDivElement;
  diagnostics: HTMLDivElement;
  searchInput: HTMLInputElement;
  searchResults: HTMLDivElement;
  fileInput: HTMLInputElement;
  coverageInput: HTMLInputElement;
  traceInput: HTMLInputElement;
  sourceMapInput: HTMLInputElement;
  coverageSummary: HTMLDivElement;
  timeline: HTMLDivElement;
  zoomOut: HTMLButtonElement;
  zoomIn: HTMLButtonElement;
  fit: HTMLButtonElement;
  diagnosticToggle: HTMLInputElement;
  focusToggle: HTMLInputElement;
};

export function collectElements(): ViewerElements {
  return {
    canvas: mustQuery("#graph-canvas"),
    status: mustQuery("#status"),
    inspector: mustQuery("#inspector-content"),
    projectTree: mustQuery("#project-tree"),
    visibleSubjects: mustQuery("#visible-subjects"),
    diagnostics: mustQuery("#diagnostic-list"),
    searchInput: mustQuery("#search-input"),
    searchResults: mustQuery("#search-results"),
    fileInput: mustQuery("#file-input"),
    coverageInput: mustQuery("#coverage-input"),
    traceInput: mustQuery("#trace-input"),
    sourceMapInput: mustQuery("#source-map-input"),
    coverageSummary: mustQuery("#coverage-summary"),
    timeline: mustQuery("#timeline"),
    zoomOut: mustQuery("#zoom-out"),
    zoomIn: mustQuery("#zoom-in"),
    fit: mustQuery("#fit"),
    diagnosticToggle: mustQuery("#diagnostic-toggle"),
    focusToggle: mustQuery("#focus-toggle")
  };
}
