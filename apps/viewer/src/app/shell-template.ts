export function shellTemplate(): string {
  return `
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
        <label class="file-button">
          <input id="source-map-input" type="file" accept="application/json,.json" />
          Load Source Map
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
}
