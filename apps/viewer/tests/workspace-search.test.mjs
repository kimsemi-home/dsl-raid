import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("workspace search includes inactive FSM states and transitions", async () => {
  const { subjectsForWorkspaceSearch } = await loadTs("src/graph/workspace-search.ts");
  const subjects = subjectsForWorkspaceSearch(store()).map((item) => item.subject);

  assert.ok(subjects.includes("fsm:agent"));
  assert.ok(subjects.includes("state:agent.planning"));
  assert.ok(subjects.includes("transition:agent.plan"));
});

test("selectSubject opens the owning FSM projection", async () => {
  const { selectSubject } = await loadTs("src/app/select-subject.ts");
  const session = { store: store() };

  selectSubject(session, "state:agent.planning");

  assert.equal(session.store.activeProjectionId, "view:agent");
  assert.equal(session.store.selection.selected, "state:agent.planning");
  assert.equal(session.store.view.source.projection, "view:agent");
});

function store() {
  const ir = fixtureIr();
  return {
    ir,
    view: fixtureView("view:runtime"),
    activeProjectionId: "view:runtime",
    selection: {},
    camera: { zoom: 1, panX: 0, panY: 0 },
    focusDepth: 2,
    showDiagnostics: true,
    diagnosticSeverity: "all",
    compositionLimit: 48
  };
}

function fixtureIr() {
  return {
    ir_version: "0.1.0",
    project: { id: "project:runscope", name: "RunScope" },
    fsms: [fsm("runtime", "RuntimeFSM"), fsm("agent", "AgentFSM")],
    projections: [
      { id: "view:runtime", kind: "projection", source: "fsm:runtime" },
      { id: "view:agent", kind: "projection", source: "fsm:agent" }
    ]
  };
}

function fsm(id, name) {
  return {
    id: `fsm:${id}`,
    name,
    states: [
      { id: "idle", kind: "atomic", initial: true },
      { id: id === "agent" ? "planning" : "running", kind: "atomic", terminal: true }
    ],
    transitions: [{ id: id === "agent" ? "plan" : "run", from: "idle", to: id === "agent" ? "planning" : "running" }]
  };
}

function fixtureView(projection) {
  return {
    view_version: "0.1.0",
    source: { core_ir: "test", projection },
    layout: { engine: "test", version: "0.1.0" },
    nodes: [{ subject: "state:runtime.running", label: "running" }],
    edges: [],
    inspector_panels: []
  };
}
