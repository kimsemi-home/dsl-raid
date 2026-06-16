import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { test } from "node:test";

test("RunScope source map links design subjects to generated Rust and Go", async () => {
  const sourceMapUrl = new URL("../../../examples/runscope/runscope.sourcemap.json", import.meta.url);
  const sourceMap = JSON.parse(await readFile(sourceMapUrl, "utf8"));
  const mappings = new Map(sourceMap.mappings.map((mapping) => [mapping.ir_subject, mapping]));
  const runtime = mappings.get("fsm:runtime");
  const completed = mappings.get("transition:runtime.running_to_completed");

  assert.equal(sourceMap.source_map_version, "0.1.0");
  assert.equal(runtime.dsl_location.uri, "lisp/runtime/runscope.lisp");
  assert.ok(completed.generated_locations.some((location) => location.location.uri.endsWith(".rs")));
  assert.ok(completed.generated_locations.some((location) => location.location.uri.endsWith(".go")));
});
