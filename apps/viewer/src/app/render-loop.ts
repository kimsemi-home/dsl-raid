import { renderCanvas } from "../canvas/renderer";
import type { AppStore } from "../store/app-store";
import type { ViewerElements } from "./elements";
import { visibleView } from "./visible-view";

export function createRenderLoop(elements: ViewerElements, getStore: () => AppStore): () => void {
  return () => {
    const store = getStore();
    renderCanvas(elements.canvas, visibleView(store), store.camera, store.selection);
  };
}
