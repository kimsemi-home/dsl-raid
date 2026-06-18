import { renderPanels } from "../../panels/render";
import type { ViewerActions } from "../action-types";
import type { ViewerElements } from "../elements";
import type { ViewerSession } from "../session";

type PanelActions = Pick<ViewerActions, "queueRender" | "syncPanels">;

export function panelActions(
  session: ViewerSession,
  elements: ViewerElements,
  actions: ViewerActions,
  queueRender: () => void
): PanelActions {
  return {
    syncPanels: () => renderPanels(elements, session.store, actions),
    queueRender
  };
}
