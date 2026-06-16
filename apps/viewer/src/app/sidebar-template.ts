export function sidebarTemplate(): string {
  return `
    <aside class="sidebar" aria-label="Project navigation">
      <div class="brand">
        <div class="brand-mark">DS</div>
        <div>
          <strong>DSLRaid</strong>
          <span>Executable Architecture Browser</span>
        </div>
      </div>
      ${fileButton("file-input", "Load IR")}
      ${fileButton("coverage-input", "Load Coverage")}
      ${fileButton("trace-input", "Load Trace")}
      ${fileButton("source-map-input", "Load Source Map")}
      <section>
        <h2>Project</h2>
        <div id="project-tree" class="tree"></div>
      </section>
      <section>
        <h2>Visible FSM</h2>
        <div id="visible-subjects" class="visible-subjects"></div>
      </section>
      <section>
        <h2>Terminal Path</h2>
        <div id="terminal-path" class="terminal-path"></div>
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
  `;
}

function fileButton(id: string, label: string): string {
  return `
    <label class="file-button">
      <input id="${id}" type="file" accept="application/json,.json" />
      ${label}
    </label>
  `;
}
