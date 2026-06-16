import type { ViewerActions } from "../app/action-types";
import type { ViewerElements } from "../app/elements";

export function bindKeyboardControls(elements: ViewerElements, actions: ViewerActions): void {
  window.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      actions.select(undefined);
    }
    if (event.key.toLowerCase() === "f") {
      actions.fit();
    }
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      elements.searchInput.focus();
    }
  });
}
