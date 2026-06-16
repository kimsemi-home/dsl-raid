import type { ViewerActions } from "../app/action-types";
import type { ViewerElements } from "../app/elements";
import { readJsonFile } from "../app/file-json";
import type { CoreIr, CoverageOverlay, RuntimeTrace, SourceMapDocument } from "../types";

export function bindFileControls(elements: ViewerElements, actions: ViewerActions): void {
  elements.fileInput.addEventListener("change", async () => {
    const ir = await readJsonFile<CoreIr>(elements.fileInput);
    if (ir) {
      actions.setIr(ir);
    }
  });
  elements.coverageInput.addEventListener("change", async () => {
    const coverage = await readJsonFile<CoverageOverlay>(elements.coverageInput);
    if (coverage) {
      actions.setCoverage(coverage);
    }
  });
  elements.traceInput.addEventListener("change", async () => {
    const trace = await readJsonFile<RuntimeTrace>(elements.traceInput);
    if (trace) {
      actions.setTrace(trace);
    }
  });
  elements.sourceMapInput.addEventListener("change", async () => {
    const sourceMap = await readJsonFile<SourceMapDocument>(elements.sourceMapInput);
    if (sourceMap) {
      actions.setSourceMap(sourceMap);
    }
  });
}
