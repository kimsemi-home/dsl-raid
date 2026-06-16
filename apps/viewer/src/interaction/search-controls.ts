import type { ViewerActions } from "../app/action-types";
import type { ViewerElements } from "../app/elements";

export function bindSearchControls(elements: ViewerElements, actions: ViewerActions): void {
  elements.searchInput.addEventListener("input", actions.syncPanels);
}
