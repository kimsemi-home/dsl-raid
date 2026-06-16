import type { CoreIr, CoverageOverlay, Projection, ViewModel } from "../types";
import { coverageIndex, projectFsm } from "./fsm-projector";
export { subjectsForSearch } from "./search";

export function projectIr(ir: CoreIr, projectionId?: string, coverage?: CoverageOverlay): ViewModel {
  const projection = selectProjection(ir, projectionId);
  const fsm = (ir.fsms ?? []).find((candidate) => candidate.id === projection.source);
  if (!fsm) {
    throw new Error(`Projection source is not an FSM: ${projection.source}`);
  }
  return projectFsm(ir, projection, fsm, coverageIndex(coverage));
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
