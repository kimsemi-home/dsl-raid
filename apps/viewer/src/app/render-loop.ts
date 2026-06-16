import { renderCanvas } from "../canvas/renderer";
import type { AppStore } from "../store/app-store";
import type { ViewerElements } from "./elements";

export function createRenderLoop(elements: ViewerElements, getStore: () => AppStore): () => void {
  let queued = false;
  return () => {
    if (queued) {
      return;
    }
    queued = true;
    requestAnimationFrame(() => {
      queued = false;
      const store = getStore();
      renderCanvas(elements.canvas, store.view, store.camera, store.selection);
    });
  };
}
