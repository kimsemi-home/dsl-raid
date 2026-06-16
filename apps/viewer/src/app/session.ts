import { projectIr } from "../graph/projection";
import { sampleIr } from "../sample-ir";
import { createInitialCamera, type AppStore } from "../store/app-store";
import type { CoreIr, CoverageOverlay, RuntimeTrace, SourceMapDocument } from "../types";
import { projectStore } from "./project-store";

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

export function setIr(session: ViewerSession, ir: CoreIr, coverage?: CoverageOverlay, sourceMap?: SourceMapDocument, trace?: RuntimeTrace): void {
  session.store = {
    ...session.store,
    ir,
    coverage,
    sourceMap,
    trace,
    activeProjectionId: ir.projections?.[0]?.id,
    view: projectIr(ir, ir.projections?.[0]?.id, coverage, trace),
    selection: { selected: ir.fsms?.[0]?.id }
  };
}

export function setTrace(session: ViewerSession, trace: RuntimeTrace): void {
  session.store = projectStore({
    ...session.store,
    trace
  });
}

export function setCoverage(session: ViewerSession, coverage: CoverageOverlay): void {
  session.store = projectStore({
    ...session.store,
    coverage
  });
}

export function setSourceMap(session: ViewerSession, sourceMap: SourceMapDocument): void {
  session.store = {
    ...session.store,
    sourceMap
  };
}

export function setProjection(session: ViewerSession, projectionId: string): void {
  const projection = session.store.ir.projections?.find((candidate) => candidate.id === projectionId);
  if (!projection) {
    return;
  }
  session.store = projectStore({
    ...session.store,
    activeProjectionId: projection.id,
    selection: { selected: projection.source }
  });
}
