import { bindControls } from "../interaction/controls";
import { loadDefaultIr } from "./defaults";
import type { ViewerElements } from "./elements";
import { createActions } from "./actions";
import { createRenderLoop } from "./render-loop";
import { createSession } from "./session";

export function startViewer(elements: ViewerElements): void {
  const session = createSession();
  const queueRender = createRenderLoop(elements, () => session.store);
  const actions = createActions(session, elements, queueRender);
  bindControls(elements, session, actions);
  actions.syncPanels();
  queueRender();
  void loadDefaultIr().then(({ ir, coverage }) => actions.setIr(ir, coverage));
}
