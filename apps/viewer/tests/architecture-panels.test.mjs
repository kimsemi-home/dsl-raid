import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("architecturePanels covers Core IR top-level architecture subjects", async () => {
  const { projectIr } = await loadTs("src/graph/projection.ts");
  const subjects = projectIr(fixtureIr(), "view:runtime").inspector_panels.map((panel) => panel.subject);

  assert.ok(subjects.includes("context:runtime"));
  assert.ok(subjects.includes("requirement:runtime_runs_to_completion"));
  assert.ok(subjects.includes("capability:runtime_execution"));
  assert.ok(subjects.includes("command:runtime_start"));
});

function fixtureIr() {
  return {
    ir_version: "0.1.0",
    project: { id: "runscope", name: "RunScope" },
    contexts: [{ id: "context:runtime", name: "Runtime", kind: "bounded_context" }],
    requirements: [{ id: "requirement:runtime_runs_to_completion", name: "Runs to completion" }],
    capabilities: [{ id: "capability:runtime_execution", name: "Runtime execution", kind: "runtime" }],
    commands: [{ id: "command:runtime_start", name: "Start runtime" }],
    fsms: [fixtureFsm()],
    projections: [{ id: "view:runtime", kind: "projection", source: "fsm:runtime" }]
  };
}

function fixtureFsm() {
  return {
    id: "fsm:runtime",
    name: "RuntimeFSM",
    context: "context:runtime",
    states: [
      { id: "idle", kind: "atomic", initial: true },
      { id: "done", kind: "atomic", terminal: true }
    ],
    transitions: [{ id: "finish", from: "idle", to: "done" }]
  };
}
