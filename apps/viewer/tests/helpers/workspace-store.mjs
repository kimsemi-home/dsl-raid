export function workspaceStore() {
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
  const terminal = id === "agent" ? "planning" : "running";
  return {
    id: `fsm:${id}`,
    name,
    states: [
      { id: "idle", kind: "atomic", initial: true },
      { id: terminal, kind: "atomic", terminal: true }
    ],
    transitions: [{ id: id === "agent" ? "plan" : "run", from: "idle", to: terminal }]
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
