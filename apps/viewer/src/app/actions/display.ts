import type { DiagnosticSeverityFilter } from "../../types";
import type { ViewerActions } from "../action-types";
import type { ViewerSession } from "../session";

type DisplayActions = Pick<ViewerActions, "setDiagnosticSeverity" | "setDiagnosticsVisible" | "setFocusDepth">;

export function displayActions(session: ViewerSession, actions: ViewerActions, refresh: () => void): DisplayActions {
  return {
    setDiagnosticsVisible: (visible: boolean) => {
      session.store.showDiagnostics = visible;
      actions.syncPanels();
    },
    setDiagnosticSeverity: (severity: DiagnosticSeverityFilter) => {
      session.store.diagnosticSeverity = severity;
      actions.syncPanels();
    },
    setFocusDepth: (depth: 1 | 2) => {
      session.store.focusDepth = depth;
      refresh();
    }
  };
}
