import { focusedView } from "../graph/neighborhood";
import type { AppStore } from "../store/app-store";
import type { ViewModel } from "../types";

export function visibleView(store: AppStore): ViewModel {
  return focusedView(store.view, store.selection.selected, store.focusDepth);
}
