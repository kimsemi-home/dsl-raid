import { sampleIr } from "../sample-ir";
import type { CoreIr, CoverageOverlay } from "../types";

export async function loadDefaultIr(): Promise<{ ir: CoreIr; coverage?: CoverageOverlay }> {
  try {
    const response = await fetch("./examples/runscope.raid.json", { cache: "no-cache" });
    if (!response.ok) {
      throw new Error(response.statusText);
    }
    return {
      ir: (await response.json()) as CoreIr,
      coverage: await loadDefaultCoverage()
    };
  } catch {
    return { ir: sampleIr };
  }
}

async function loadDefaultCoverage(): Promise<CoverageOverlay | undefined> {
  try {
    const response = await fetch("./examples/run-001.coverage.json", { cache: "no-cache" });
    if (!response.ok) {
      return undefined;
    }
    return (await response.json()) as CoverageOverlay;
  } catch {
    return undefined;
  }
}
