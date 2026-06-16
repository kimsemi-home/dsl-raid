import assert from "node:assert/strict";
import { test } from "node:test";
import { loadTs } from "./helpers/load-ts.mjs";

test("cursorSubject moves through visible subjects with wraparound", async () => {
  const { cursorSubject } = await loadTs("src/graph/subject-cursor.ts");
  const subjects = ["state:a", "state:b", "transition:c"];

  assert.equal(cursorSubject(subjects, undefined, 1), "state:a");
  assert.equal(cursorSubject(subjects, undefined, -1), "transition:c");
  assert.equal(cursorSubject(subjects, "state:b", 1), "transition:c");
  assert.equal(cursorSubject(subjects, "state:a", -1), "transition:c");
});

test("cursorSubject returns undefined for empty subject lists", async () => {
  const { cursorSubject } = await loadTs("src/graph/subject-cursor.ts");

  assert.equal(cursorSubject([], "state:a", 1), undefined);
});
