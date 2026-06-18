import type { CoverageOverlay, CoreIr, RuntimeTrace, SourceMapDocument } from "../../types";
import type { ViewerActions } from "../action-types";
import * as viewerSession from "../session";
import { syncSourceMap, syncTrace } from "../sync-metadata";

type DataActions = Pick<ViewerActions, "setCoverage" | "setIr" | "setSourceMap" | "setTrace">;

export function dataActions(
  session: viewerSession.ViewerSession,
  actions: ViewerActions,
  refresh: () => void
): DataActions {
  return {
    setIr: (ir: CoreIr, coverage?: CoverageOverlay, sourceMap?: SourceMapDocument, trace?: RuntimeTrace) => {
      viewerSession.setIr(session, ir, coverage, sourceMap, trace);
      actions.fit();
    },
    setCoverage: (coverage: CoverageOverlay) => {
      viewerSession.setCoverage(session, coverage);
      refresh();
    },
    setTrace: (trace: RuntimeTrace) => syncTrace(session, actions, trace),
    setSourceMap: (sourceMap: SourceMapDocument) => syncSourceMap(session, actions, sourceMap)
  };
}
