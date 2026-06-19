import type { ViewModel } from "../../types";
import { buildHitIndex } from "./build";
import type { HitIndex } from "./model";

const cache = new WeakMap<ViewModel, HitIndex>();

export function hitIndexFor(view: ViewModel): HitIndex {
  const cached = cache.get(view);
  if (cached) {
    return cached;
  }
  const index = buildHitIndex(view);
  cache.set(view, index);
  return index;
}
