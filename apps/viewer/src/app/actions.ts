import type { ViewerActions } from "./action-types";
import { dataActions } from "./actions/data";
import { displayActions } from "./actions/display";
import { navigationActions } from "./actions/navigation";
import { panelActions } from "./actions/panels";
import { viewportActions } from "./actions/viewport";
import type { ViewerElements } from "./elements";
import type { ViewerSession } from "./session";

export function createActions(session: ViewerSession, elements: ViewerElements, queueRender: () => void): ViewerActions {
  const actions = {} as ViewerActions;
  const refresh = () => { actions.syncPanels(); queueRender(); };
  Object.assign(
    actions,
    dataActions(session, actions, refresh),
    navigationActions(session, actions, refresh),
    viewportActions(session, elements, refresh, queueRender),
    displayActions(session, actions, refresh),
    panelActions(session, elements, actions, queueRender)
  );
  return actions;
}
