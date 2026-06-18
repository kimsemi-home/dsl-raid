import type { CoverageOverlay, RuntimeTrace, SourceMapDocument } from "../../types";
import { projectStore } from "../project-store";
import type { ViewerSession } from "./model";

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
