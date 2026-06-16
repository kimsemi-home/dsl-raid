import { sampleIr } from "../sample-ir";
import type { CoreIr, CoverageOverlay, SourceMapDocument } from "../types";

export type DefaultIrBundle = {
  ir: CoreIr;
  coverage?: CoverageOverlay;
  sourceMap?: SourceMapDocument;
};

export async function loadDefaultIr(): Promise<DefaultIrBundle> {
  const ir = await loadJson<CoreIr>("./examples/runscope.raid.json");
  if (!ir) {
    return { ir: sampleIr };
  }
  return {
    ir,
    coverage: await loadJson<CoverageOverlay>("./examples/run-001.coverage.json"),
    sourceMap: await loadJson<SourceMapDocument>("./examples/runscope.sourcemap.json")
  };
}

async function loadJson<T>(path: string): Promise<T | undefined> {
  try {
    const response = await fetch(path, { cache: "no-cache" });
    if (!response.ok) {
      return undefined;
    }
    return (await response.json()) as T;
  } catch {
    return undefined;
  }
}
