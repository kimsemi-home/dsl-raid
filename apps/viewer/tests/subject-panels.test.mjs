import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("projected inspector panels cover project, event, and policy subjects", async () => {
  const { projectIr } = await loadTs("src/graph/projection.ts");
  const view = projectIr(fixtureIr(), "view:runtime");
  const panels = new Map(view.inspector_panels.map((panel) => [panel.subject, panel]));

  assert.ok(panels.has("project:runscope"));
  assert.ok(panels.has("event:runtime.start_requested"));
  assert.ok(panels.has("policy:no_secret_leak"));
  assert.equal(row(panels.get("event:runtime.start_requested"), "Transitions"), "1");
  assert.equal(row(panels.get("policy:no_secret_leak"), "Subject"), "transition:runtime.starting_to_running");
});

function row(panel, label) {
  return panel.sections.flatMap((section) => section.rows).find((item) => item.label === label)?.value;
}

function fixtureIr() {
  return {
    ir_version: "0.1.0",
    project: { id: "runscope", name: "RunScope", visibility: "public" },
    policies: [
      {
        id: "policy:no_secret_leak",
        name: "No secret leak",
        kind: "security",
        applies_to: ["transition:runtime.starting_to_running"]
      }
    ],
    fsms: [fixtureFsm()],
    projections: [{ id: "view:runtime", kind: "projection", source: "fsm:runtime" }]
  };
}

function fixtureFsm() {
  return {
    id: "fsm:runtime",
    name: "RuntimeFSM",
    states: [
      { id: "idle", kind: "atomic", initial: true },
      { id: "starting", kind: "atomic" },
      { id: "running", kind: "atomic", terminal: true }
    ],
    events: [{ id: "start_requested", kind: "external" }],
    transitions: [
      {
        id: "idle_to_starting",
        from: "idle",
        to: "starting",
        on: "start_requested"
      },
      {
        id: "starting_to_running",
        from: "starting",
        to: "running",
        requires: ["policy:no_secret_leak"]
      }
    ]
  };
}
