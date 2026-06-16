import type { AppStore } from "../store/app-store";

export function activeFsmId(store: AppStore): string | undefined {
  const projection = store.ir.projections?.find((item) => item.id === store.activeProjectionId);
  if (projection?.source.startsWith("fsm:")) {
    return projection.source;
  }
  const selected = store.selection.selected;
  return store.ir.fsms?.some((fsm) => fsm.id === selected) ? selected : undefined;
}
