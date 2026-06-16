export function workspaceTemplate(): string {
  return `
    <main class="viewport-wrap">
      <div class="toolbar" aria-label="Viewer toolbar">
        <button id="zoom-out" title="Zoom out">-</button>
        <button id="zoom-in" title="Zoom in">+</button>
        <button id="fit" title="Fit graph">Fit</button>
        <label><input id="diagnostic-toggle" type="checkbox" checked /> Diagnostics</label>
        <label><input id="focus-toggle" type="checkbox" /> 1-hop focus</label>
        <label>States <input id="composition-limit" type="number" min="1" step="24" value="48" /></label>
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
    <section class="bottom-panel" aria-label="Runtime evidence">
      <div>
        <h2>Diagnostics</h2>
        <div id="diagnostic-list"></div>
      </div>
      <div>
        <h2>Timeline</h2>
        <div id="timeline"></div>
      </div>
    </section>
  `;
}
