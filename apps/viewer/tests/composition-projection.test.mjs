import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("composition projection materializes bounded tuple view", async () => {
  const { projectIr } = await loadTs("src/graph/projection.ts");
  const view = projectIr(fixtureIr(), "view:runscope");
  const subjects = view.inspector_panels.map((panel) => panel.subject);

  assert.equal(view.layout.engine, "bounded-reachable-product");
  assert.ok(view.nodes.some((node) => node.subject.startsWith("state_tuple:")));
  assert.ok(view.edges.some((edge) => edge.subject.startsWith("tuple_transition:")));
  assert.ok(subjects.includes("composition:runscope"));
});

test("composition projection accepts lazy materialization limit", async () => {
  const { projectIr } = await loadTs("src/graph/projection.ts");
  const view = projectIr(fixtureIr(), "view:runscope", undefined, undefined, { compositionLimit: 2 });
  const panel = view.inspector_panels.find((item) => item.subject === "composition:runscope");
  const rows = Object.fromEntries(panel.sections[0].rows.map((row) => [row.label, row.value]));

  assert.equal(view.nodes.length, 2);
  assert.equal(rows.Materialized, "2");
  assert.equal(rows.Truncated, "true");
});

function fixtureIr() {
  return {
    ir_version: "0.1.0",
    project: { id: "runscope", name: "RunScope" },
    fsms: [fsm("runtime", "RuntimeFSM"), fsm("agent", "AgentFSM")],
    compositions: [{
      id: "composition:runscope",
      name: "RunScopeFSM",
      kind: "product",
      inputs: ["fsm:runtime", "fsm:agent"],
      state_space: { kind: "lazy", max_materialized_states: 12 }
    }],
    projections: [{
      id: "view:runscope",
      kind: "projection",
      source: "composition:runscope",
      show: ["states", "transitions"]
    }]
  };
}

function fsm(id, name) {
  return {
    id: `fsm:${id}`,
    name,
    states: [
      { id: "idle", kind: "atomic", initial: true },
      { id: "done", kind: "atomic", terminal: true }
    ],
    transitions: [{ id: "finish", from: "idle", to: "done" }]
  };
}
