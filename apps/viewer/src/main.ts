import "./styles.css";
import { screenToWorld, zoomAt } from "./canvas/camera";
import { hitTest } from "./canvas/hit-test";
import { renderCanvas } from "./canvas/renderer";
import { projectIr, subjectsForSearch } from "./graph/projection";
import { sampleIr } from "./sample-ir";
import { createInitialCamera, type AppStore } from "./store/app-store";
import type { CoreIr, CoverageOverlay, CoverageStatus, InspectorPanel, Point } from "./types";

const root = document.querySelector<HTMLDivElement>("#app");
if (!root) {
  throw new Error("Missing #app root");
}

root.innerHTML = `
  <div class="shell">
    <aside class="sidebar" aria-label="Project navigation">
      <div class="brand">
        <div class="brand-mark">DS</div>
        <div>
          <strong>DSLRaid</strong>
          <span>Executable Architecture Browser</span>
        </div>
      </div>
      <label class="file-button">
        <input id="file-input" type="file" accept="application/json,.json" />
        Load IR
      </label>
      <label class="file-button">
        <input id="coverage-input" type="file" accept="application/json,.json" />
        Load Coverage
      </label>
      <section>
        <h2>Project</h2>
        <div id="project-tree" class="tree"></div>
      </section>
      <section>
        <h2>Coverage</h2>
        <div id="coverage-summary" class="coverage-summary"></div>
      </section>
      <section>
        <h2>Search</h2>
        <input id="search-input" class="search" type="search" placeholder="state, transition, artifact" />
        <div id="search-results" class="search-results"></div>
      </section>
    </aside>
    <main class="viewport-wrap">
      <div class="toolbar" aria-label="Viewer toolbar">
        <button id="zoom-out" title="Zoom out">-</button>
        <button id="zoom-in" title="Zoom in">+</button>
        <button id="fit" title="Fit graph">Fit</button>
        <label><input id="diagnostic-toggle" type="checkbox" checked /> Diagnostics</label>
        <label><input id="focus-toggle" type="checkbox" /> 1-hop focus</label>
      </div>
      <div class="canvas-stage">
        <canvas id="graph-canvas" aria-label="FSM projection canvas"></canvas>
      </div>
      <div id="status" class="status"></div>
    </main>
    <aside class="inspector" aria-label="Inspector">
      <h2>Inspector</h2>
      <div id="inspector-content"></div>
    </aside>
    <section class="bottom-panel" aria-label="Diagnostics">
      <h2>Diagnostics</h2>
      <div id="diagnostic-list"></div>
    </section>
  </div>
`;

const canvas = mustQuery<HTMLCanvasElement>("#graph-canvas");
const status = mustQuery<HTMLDivElement>("#status");
const inspector = mustQuery<HTMLDivElement>("#inspector-content");
const projectTree = mustQuery<HTMLDivElement>("#project-tree");
const diagnostics = mustQuery<HTMLDivElement>("#diagnostic-list");
const searchInput = mustQuery<HTMLInputElement>("#search-input");
const searchResults = mustQuery<HTMLDivElement>("#search-results");
const fileInput = mustQuery<HTMLInputElement>("#file-input");
const coverageInput = mustQuery<HTMLInputElement>("#coverage-input");
const coverageSummary = mustQuery<HTMLDivElement>("#coverage-summary");

let store: AppStore = {
  ir: sampleIr,
  view: projectIr(sampleIr),
  camera: createInitialCamera(),
  selection: {},
  focusDepth: 2,
  showDiagnostics: true
};

let dragging = false;
let lastMouse: Point | undefined;
let renderQueued = false;
const coverageStatusOrder: CoverageStatus[] = ["covered", "uncovered", "failed", "flaky", "deployed", "not_deployed"];

void loadDefaultIr();
bindControls();
syncPanels();
queueRender();

async function loadDefaultIr(): Promise<void> {
  try {
    const response = await fetch("./examples/runscope.raid.json", { cache: "no-cache" });
    if (!response.ok) {
      throw new Error(response.statusText);
    }
    const ir = (await response.json()) as CoreIr;
    const coverage = await loadDefaultCoverage();
    setIr(ir, coverage);
  } catch {
    setIr(sampleIr);
  }
}

async function loadDefaultCoverage(): Promise<CoverageOverlay | undefined> {
  try {
    const response = await fetch("./examples/run-001.coverage.json", { cache: "no-cache" });
    if (!response.ok) {
      return undefined;
    }
    return (await response.json()) as CoverageOverlay;
  } catch {
    return undefined;
  }
}

function setIr(ir: CoreIr, coverage?: CoverageOverlay): void {
  store = {
    ...store,
    ir,
    coverage,
    view: projectIr(ir, undefined, coverage),
    selection: { selected: ir.fsms?.[0]?.id }
  };
  fitGraph();
  syncPanels();
  queueRender();
}

function bindControls(): void {
  canvas.addEventListener("pointerdown", (event) => {
    canvas.setPointerCapture(event.pointerId);
    dragging = true;
    lastMouse = { x: event.clientX, y: event.clientY };
  });
  canvas.addEventListener("pointerup", (event) => {
    canvas.releasePointerCapture(event.pointerId);
    dragging = false;
    const point = relativePoint(event);
    const hit = hitTest(store.view, screenToWorld(store.camera, point));
    store.selection.selected = hit?.subject;
    syncPanels();
    queueRender();
  });
  canvas.addEventListener("pointermove", (event) => {
    const point = relativePoint(event);
    const world = screenToWorld(store.camera, point);
    const hit = hitTest(store.view, world);
    store.selection.hovered = hit?.subject;
    if (dragging && lastMouse && !hit) {
      store.camera.panX += event.clientX - lastMouse.x;
      store.camera.panY += event.clientY - lastMouse.y;
      lastMouse = { x: event.clientX, y: event.clientY };
    }
    updateStatus(world);
    queueRender();
  });
  canvas.addEventListener(
    "wheel",
    (event) => {
      event.preventDefault();
      const point = relativePoint(event);
      const factor = event.deltaY > 0 ? 0.92 : 1.08;
      store.camera = zoomAt(store.camera, point, store.camera.zoom * factor);
      queueRender();
    },
    { passive: false }
  );
  document.querySelector<HTMLButtonElement>("#zoom-out")?.addEventListener("click", () => {
    store.camera = zoomAt(store.camera, { x: canvas.clientWidth / 2, y: canvas.clientHeight / 2 }, store.camera.zoom * 0.85);
    queueRender();
  });
  document.querySelector<HTMLButtonElement>("#zoom-in")?.addEventListener("click", () => {
    store.camera = zoomAt(store.camera, { x: canvas.clientWidth / 2, y: canvas.clientHeight / 2 }, store.camera.zoom * 1.15);
    queueRender();
  });
  document.querySelector<HTMLButtonElement>("#fit")?.addEventListener("click", () => {
    fitGraph();
    queueRender();
  });
  document.querySelector<HTMLInputElement>("#diagnostic-toggle")?.addEventListener("change", (event) => {
    store.showDiagnostics = (event.target as HTMLInputElement).checked;
    syncDiagnostics();
  });
  document.querySelector<HTMLInputElement>("#focus-toggle")?.addEventListener("change", (event) => {
    store.focusDepth = (event.target as HTMLInputElement).checked ? 1 : 2;
    syncPanels();
  });
  searchInput.addEventListener("input", syncSearch);
  fileInput.addEventListener("change", async () => {
    const file = fileInput.files?.[0];
    if (!file) {
      return;
    }
    const ir = JSON.parse(await file.text()) as CoreIr;
    setIr(ir);
  });
  coverageInput.addEventListener("change", async () => {
    const file = coverageInput.files?.[0];
    if (!file) {
      return;
    }
    const coverage = JSON.parse(await file.text()) as CoverageOverlay;
    store = {
      ...store,
      coverage,
      view: projectIr(store.ir, undefined, coverage)
    };
    syncPanels();
    queueRender();
  });
  window.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      store.selection.selected = undefined;
      syncPanels();
      queueRender();
    }
    if (event.key.toLowerCase() === "f") {
      fitGraph();
      queueRender();
    }
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      searchInput.focus();
    }
  });
  window.addEventListener("resize", queueRender);
}

function syncPanels(): void {
  syncProjectTree();
  syncCoverageSummary();
  syncInspector();
  syncDiagnostics();
  syncSearch();
}

function syncProjectTree(): void {
  const fsmRows = (store.ir.fsms ?? [])
    .map(
      (fsm) => `<button class="tree-row" data-subject="${escapeHtml(fsm.id)}">
        <span>${escapeHtml(fsm.name)}</span><small>${(fsm.states ?? []).length} states</small>
      </button>`
    )
    .join("");
  projectTree.innerHTML = `
    <div class="project-name">${escapeHtml(store.ir.project.name)}</div>
    ${fsmRows}
  `;
  projectTree.querySelectorAll<HTMLButtonElement>("[data-subject]").forEach((button) => {
    button.addEventListener("click", () => {
      store.selection.selected = button.dataset.subject;
      syncPanels();
      queueRender();
    });
  });
}

function syncCoverageSummary(): void {
  const subjects = store.coverage?.subjects ?? [];
  if (!store.coverage || subjects.length === 0) {
    coverageSummary.innerHTML = `<p class="muted">No coverage overlay loaded.</p>`;
    return;
  }
  const counts = subjects.reduce<Record<string, number>>((accumulator, subject) => {
    accumulator[subject.status] = (accumulator[subject.status] ?? 0) + 1;
    return accumulator;
  }, {});
  const statuses = [
    ...coverageStatusOrder.filter((status) => counts[status]),
    ...Object.keys(counts)
      .filter((status) => !coverageStatusOrder.includes(status as CoverageStatus))
      .sort()
  ];
  coverageSummary.innerHTML = `
    <div class="coverage-grid">
      ${statuses.map((status) => coverageSummaryItem(status, counts[status] ?? 0)).join("")}
    </div>
  `;
}

function coverageSummaryItem(label: string, count: number): string {
  const displayLabel = label.replaceAll("_", " ");
  return `<div class="coverage-pill ${escapeHtml(label)}"><strong>${count}</strong><span>${escapeHtml(displayLabel)}</span></div>`;
}

function syncInspector(): void {
  const subject = store.selection.selected;
  const panel = subject ? store.view.inspector_panels.find((candidate) => candidate.subject === subject) : undefined;
  inspector.innerHTML = panel ? panelHtml(panel) : `<p class="muted">Select a state or transition to inspect source, tests, generated artifacts, policies, and diagnostics.</p>`;
  inspector.querySelectorAll<HTMLButtonElement>("[data-subject]").forEach((button) => {
    button.addEventListener("click", () => {
      store.selection.selected = button.dataset.subject;
      syncPanels();
      queueRender();
    });
  });
}

function panelHtml(panel: InspectorPanel): string {
  return `
    <div class="subject">${escapeHtml(panel.subject)}</div>
    <h3>${escapeHtml(panel.title)}</h3>
    ${panel.sections
      .map(
        (section) => `
          <section class="inspector-section">
            <h4>${escapeHtml(section.title)}</h4>
            ${section.rows
              .map(
                (row) => `
                  <div class="row">
                    <span>${escapeHtml(row.label)}</span>
                    ${
                      row.subject
                        ? `<button data-subject="${escapeHtml(row.subject)}">${escapeHtml(row.value)}</button>`
                        : `<strong>${escapeHtml(row.value)}</strong>`
                    }
                  </div>`
              )
              .join("")}
          </section>`
      )
      .join("")}
  `;
}

function syncDiagnostics(): void {
  if (!store.showDiagnostics) {
    diagnostics.innerHTML = `<p class="muted">Diagnostics hidden.</p>`;
    return;
  }
  const coreDiagnostics = store.ir.diagnostics ?? [];
  if (coreDiagnostics.length === 0) {
    diagnostics.innerHTML = `<p class="muted">No diagnostics in this IR. CLI validation can still produce assertion-level reports.</p>`;
    return;
  }
  diagnostics.innerHTML = coreDiagnostics
    .map(
      (diagnostic) => `
        <button class="diagnostic ${diagnostic.severity}" data-subject="${escapeHtml(diagnostic.subjects?.[0] ?? "")}">
          <strong>${escapeHtml(diagnostic.code)}</strong>
          <span>${escapeHtml(diagnostic.message)}</span>
        </button>`
    )
    .join("");
}

function syncSearch(): void {
  const query = searchInput.value.trim().toLowerCase();
  const subjects = subjectsForSearch(store.view)
    .filter((item) => !query || item.subject.toLowerCase().includes(query) || item.label.toLowerCase().includes(query))
    .slice(0, 16);
  searchResults.innerHTML = subjects
    .map(
      (item) => `
        <button data-subject="${escapeHtml(item.subject)}">
          <span>${escapeHtml(item.label)}</span>
          <small>${escapeHtml(item.kind)}</small>
        </button>`
    )
    .join("");
  searchResults.querySelectorAll<HTMLButtonElement>("[data-subject]").forEach((button) => {
    button.addEventListener("click", () => {
      store.selection.selected = button.dataset.subject;
      syncPanels();
      queueRender();
    });
  });
}

function updateStatus(world: Point): void {
  status.textContent = `zoom ${store.camera.zoom.toFixed(2)} / world ${world.x.toFixed(0)}, ${world.y.toFixed(0)}`;
}

function fitGraph(): void {
  if (store.view.nodes.length === 0) {
    store.camera = createInitialCamera();
    return;
  }
  const minX = Math.min(...store.view.nodes.map((node) => node.x));
  const minY = Math.min(...store.view.nodes.map((node) => node.y));
  const maxX = Math.max(...store.view.nodes.map((node) => node.x + node.width));
  const maxY = Math.max(...store.view.nodes.map((node) => node.y + node.height));
  const graphWidth = Math.max(1, maxX - minX);
  const graphHeight = Math.max(1, maxY - minY);
  const zoom = Math.min(1.35, Math.max(0.45, Math.min((canvas.clientWidth - 80) / graphWidth, (canvas.clientHeight - 80) / graphHeight)));
  store.camera = {
    zoom,
    panX: 40 - minX * zoom,
    panY: 40 - minY * zoom
  };
}

function queueRender(): void {
  if (renderQueued) {
    return;
  }
  renderQueued = true;
  requestAnimationFrame(() => {
    renderQueued = false;
    renderCanvas(canvas, store.view, store.camera, store.selection);
  });
}

function relativePoint(event: MouseEvent | PointerEvent | WheelEvent): Point {
  const rect = canvas.getBoundingClientRect();
  return {
    x: event.clientX - rect.left,
    y: event.clientY - rect.top
  };
}

function escapeHtml(value: string): string {
  return value.replaceAll("&", "&amp;").replaceAll("<", "&lt;").replaceAll(">", "&gt;").replaceAll('"', "&quot;");
}

function mustQuery<T extends Element>(selector: string): T {
  const element = document.querySelector<T>(selector);
  if (!element) {
    throw new Error(`Missing required DOM node: ${selector}`);
  }
  return element;
}
