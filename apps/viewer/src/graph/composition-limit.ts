import type { Composition, ProjectionOptions } from "../types";

export function compositionLimit(composition: Composition, options?: ProjectionOptions): number {
  const coreLimit = composition.state_space?.max_materialized_states ?? 48;
  const uiLimit = options?.compositionLimit;
  if (!uiLimit || uiLimit < 1) {
    return coreLimit;
  }
  return Math.min(coreLimit, uiLimit);
}
