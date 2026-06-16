import { projectIr } from "../graph/projection";
import { sampleIr } from "../sample-ir";
import { createInitialCamera, type AppStore } from "../store/app-store";
import type { CoreIr, CoverageOverlay } from "../types";

export type ViewerSession = {
  store: AppStore;
};

export function createSession(): ViewerSession {
  return {
    store: {
      ir: sampleIr,
      view: projectIr(sampleIr),
      camera: createInitialCamera(),
      selection: {},
      focusDepth: 2,
      showDiagnostics: true
    }
  };
}

export function setIr(session: ViewerSession, ir: CoreIr, coverage?: CoverageOverlay): void {
  session.store = {
    ...session.store,
    ir,
    coverage,
    view: projectIr(ir, undefined, coverage),
    selection: { selected: ir.fsms?.[0]?.id }
  };
}

export function setCoverage(session: ViewerSession, coverage: CoverageOverlay): void {
  session.store = {
    ...session.store,
    coverage,
    view: projectIr(session.store.ir, undefined, coverage)
  };
}
