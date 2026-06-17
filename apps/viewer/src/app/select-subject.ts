import { projectionForSubject } from "../graph/fsm-subject";
import type { ViewerSession } from "./session";
import { projectStore } from "./project-store";

export function selectSubject(
  session: ViewerSession,
  subject: string | undefined,
  related?: string[]
): boolean {
  const projectionId = projectionForSubject(session.store.ir, subject);
  const activeProjectionId = projectionId ?? session.store.activeProjectionId;
  const changed = activeProjectionId !== session.store.activeProjectionId;
  session.store = projectStore({
    ...session.store,
    activeProjectionId,
    selection: {
      selected: subject,
      related: related?.length ? related : undefined
    }
  });
  return changed;
}
