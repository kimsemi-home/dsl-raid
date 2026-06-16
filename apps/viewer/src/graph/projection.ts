import type { CoreIr, CoverageOverlay, Projection, ProjectionOptions, RuntimeTrace, ViewModel } from "../types";
import { projectComposition } from "./composition-projector";
import { coverageIndex, projectFsm } from "./fsm-projector";
import { traceIndex } from "./trace";
export { subjectsForSearch } from "./search";

export function projectIr(ir: CoreIr, projectionId?: string, coverage?: CoverageOverlay, trace?: RuntimeTrace, options?: ProjectionOptions): ViewModel {
  const projection = selectProjection(ir, projectionId);
  const fsm = (ir.fsms ?? []).find((candidate) => candidate.id === projection.source);
  if (fsm) {
    return projectFsm(ir, projection, fsm, coverageIndex(coverage), traceIndex(trace));
  }
  const composition = (ir.compositions ?? []).find((candidate) => candidate.id === projection.source);
  if (composition) {
    return projectComposition(ir, projection, composition, options);
  }
  throw new Error(`Projection source is not supported: ${projection.source}`);
}

function selectProjection(ir: CoreIr, projectionId?: string): Projection {
  const projections = ir.projections ?? [];
  const projection = projectionId
    ? projections.find((candidate) => candidate.id === projectionId)
    : projections[0];
  if (!projection) {
    throw new Error("No projection is available in this IR.");
  }
  return projection;
}
