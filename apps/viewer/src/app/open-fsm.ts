import type { ViewerActions } from "./action-types";
import type { ViewerSession } from "./session";

export function openFsm(actions: ViewerActions, session: ViewerSession, fsmId: string): void {
  const projection = session.store.ir.projections?.find((candidate) => candidate.source === fsmId);
  if (projection) {
    actions.openProjection(projection.id);
  } else {
    actions.select(fsmId);
  }
}
