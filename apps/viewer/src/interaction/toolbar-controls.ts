import type { ViewerActions } from "../app/action-types";
import type { ViewerElements } from "../app/elements";

export function bindToolbarControls(elements: ViewerElements, actions: ViewerActions): void {
  elements.zoomOut.addEventListener("click", () => actions.zoom(center(elements.canvas), 0.85));
  elements.zoomIn.addEventListener("click", () => actions.zoom(center(elements.canvas), 1.15));
  elements.fit.addEventListener("click", actions.fit);
  elements.diagnosticToggle.addEventListener("change", () => {
    actions.setDiagnosticsVisible(elements.diagnosticToggle.checked);
  });
  elements.focusToggle.addEventListener("change", () => {
    actions.setFocusDepth(elements.focusToggle.checked ? 1 : 2);
  });
  elements.compositionLimit.addEventListener("change", () => syncCompositionLimit(elements, actions));
  elements.compositionLimit.addEventListener("input", () => syncCompositionLimit(elements, actions));
}

function center(canvas: HTMLCanvasElement) {
  return {
    x: canvas.clientWidth / 2,
    y: canvas.clientHeight / 2
  };
}

function syncCompositionLimit(elements: ViewerElements, actions: ViewerActions): void {
  actions.setCompositionLimit(Number.parseInt(elements.compositionLimit.value, 10));
}
