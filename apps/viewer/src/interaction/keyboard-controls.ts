import type { ViewerActions } from "../app/action-types";
import type { ViewerElements } from "../app/elements";

export function bindKeyboardControls(elements: ViewerElements, actions: ViewerActions): void {
  window.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      actions.select(undefined);
    }
    if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
      event.preventDefault();
      elements.searchInput.focus();
    }
    if (isTextEntry(event.target)) {
      return;
    }
    if (event.key.toLowerCase() === "f") {
      actions.fit();
    }
    if (event.key === "ArrowDown" || event.key.toLowerCase() === "j") {
      event.preventDefault();
      actions.selectRelative(1);
    }
    if (event.key === "ArrowUp" || event.key.toLowerCase() === "k") {
      event.preventDefault();
      actions.selectRelative(-1);
    }
  });
}

function isTextEntry(target: EventTarget | null): boolean {
  return target instanceof HTMLElement && target.matches("input, textarea, [contenteditable='true']");
}
