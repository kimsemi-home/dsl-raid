import { projectIr } from "../../graph/projection";
import type { CoreIr, CoverageOverlay, RuntimeTrace, SourceMapDocument } from "../../types";
import type { ViewerSession } from "./model";

export function setIr(
  session: ViewerSession,
  ir: CoreIr,
  coverage?: CoverageOverlay,
  sourceMap?: SourceMapDocument,
  trace?: RuntimeTrace
): void {
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
