import type { ViewerActions } from "../app/actions";

export function bindWindowControls(actions: ViewerActions): void {
  window.addEventListener("resize", actions.queueRender);
}
