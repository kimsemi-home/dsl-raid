import { projectStore } from "../project-store";
import type { ViewerSession } from "./model";

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
