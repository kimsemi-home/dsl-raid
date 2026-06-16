import type { ViewerActions } from "../app/action-types";

export function bindWindowControls(actions: ViewerActions): void {
  window.addEventListener("resize", actions.queueRender);
}
