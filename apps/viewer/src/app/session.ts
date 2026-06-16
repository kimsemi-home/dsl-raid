import { projectIr } from "../graph/projection";
import { sampleIr } from "../sample-ir";
import { createInitialCamera, type AppStore } from "../store/app-store";
import type { CoreIr, CoverageOverlay } from "../types";

export type ViewerSession = {
  store: AppStore;
};

export function createSession(): ViewerSession {
  const activeProjectionId = sampleIr.projections?.[0]?.id;
  return {
    store: {
      ir: sampleIr,
      view: projectIr(sampleIr, activeProjectionId),
      activeProjectionId,
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
    activeProjectionId: ir.projections?.[0]?.id,
    view: projectIr(ir, ir.projections?.[0]?.id, coverage),
    selection: { selected: ir.fsms?.[0]?.id }
  };
}

export function setCoverage(session: ViewerSession, coverage: CoverageOverlay): void {
  session.store = {
    ...session.store,
    coverage,
    view: projectIr(session.store.ir, session.store.activeProjectionId, coverage)
  };
}

export function setProjection(session: ViewerSession, projectionId: string): void {
  const projection = session.store.ir.projections?.find((candidate) => candidate.id === projectionId);
  if (!projection) {
    return;
  }
  session.store = {
    ...session.store,
    activeProjectionId: projection.id,
    view: projectIr(session.store.ir, projection.id, session.store.coverage),
    selection: { selected: projection.source }
  };
}
