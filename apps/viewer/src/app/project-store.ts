import { projectIr } from "../graph/projection";
import type { AppStore } from "../store/app-store";

export function projectStore(store: AppStore): AppStore {
  return {
    ...store,
    view: projectIr(store.ir, store.activeProjectionId, store.coverage, store.trace, {
      compositionLimit: store.compositionLimit
    })
  };
}
