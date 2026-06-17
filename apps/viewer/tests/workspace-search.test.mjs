import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";
import { workspaceStore } from "./helpers/workspace-store.mjs";

test("workspace search includes inactive FSM states and transitions", async () => {
  const { subjectsForWorkspaceSearch } = await loadTs("src/graph/workspace-search.ts");
  const subjects = subjectsForWorkspaceSearch(workspaceStore()).map((item) => item.subject);

  assert.ok(subjects.includes("fsm:agent"));
  assert.ok(subjects.includes("state:agent.planning"));
  assert.ok(subjects.includes("transition:agent.plan"));
});

test("selectSubject opens the owning FSM projection", async () => {
  const { selectSubject } = await loadTs("src/app/select-subject.ts");
  const session = { store: workspaceStore() };

  const changed = selectSubject(session, "state:agent.planning");

  assert.equal(changed, true);
  assert.equal(session.store.activeProjectionId, "view:agent");
  assert.equal(session.store.selection.selected, "state:agent.planning");
  assert.equal(session.store.view.source.projection, "view:agent");
});

test("selectSubject keeps projection stable for active FSM subjects", async () => {
  const { selectSubject } = await loadTs("src/app/select-subject.ts");
  const session = { store: workspaceStore() };

  const changed = selectSubject(session, "state:runtime.running");

  assert.equal(changed, false);
  assert.equal(session.store.activeProjectionId, "view:runtime");
  assert.equal(session.store.selection.selected, "state:runtime.running");
});
