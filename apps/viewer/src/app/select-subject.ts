import { projectionForSubject } from "../graph/fsm-subject";
import type { ViewerSession } from "./session";
import { projectStore } from "./project-store";

export function selectSubject(
  session: ViewerSession,
  subject: string | undefined,
  related?: string[]
): void {
  const projectionId = projectionForSubject(session.store.ir, subject);
  session.store = projectStore({
    ...session.store,
    activeProjectionId: projectionId ?? session.store.activeProjectionId,
    selection: {
      selected: subject,
      related: related?.length ? related : undefined
    }
  });
}
