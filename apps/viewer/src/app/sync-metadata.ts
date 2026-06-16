import type { RuntimeTrace, SourceMapDocument } from "../types";
import type { ViewerActions } from "./action-types";
import type { ViewerSession } from "./session";
import * as viewerSession from "./session";

export function syncTrace(session: ViewerSession, actions: ViewerActions, trace: RuntimeTrace): void {
  viewerSession.setTrace(session, trace);
  actions.syncPanels();
}

export function syncSourceMap(session: ViewerSession, actions: ViewerActions, sourceMap: SourceMapDocument): void {
  viewerSession.setSourceMap(session, sourceMap);
  actions.syncPanels();
}
