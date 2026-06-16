import { projectStore } from "./project-store";
import type { ViewerSession } from "./session";

export function setCompositionLimit(session: ViewerSession, limit: number): void {
  if (!Number.isFinite(limit)) {
    return;
  }
  session.store = projectStore({
    ...session.store,
    compositionLimit: Math.max(1, Math.floor(limit))
  });
}
