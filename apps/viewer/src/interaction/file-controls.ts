import type { ViewerActions } from "../app/actions";
import type { ViewerElements } from "../app/elements";
import { readJsonFile } from "../app/file-json";
import type { CoreIr, CoverageOverlay } from "../types";

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
}
