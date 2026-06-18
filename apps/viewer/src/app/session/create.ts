import { projectIr } from "../../graph/projection";
import { sampleIr } from "../../sample-ir";
import { createInitialCamera, DEFAULT_COMPOSITION_LIMIT } from "../../store/app-store";
import type { ViewerSession } from "./model";

export function createSession(): ViewerSession {
  const activeProjectionId = sampleIr.projections?.[0]?.id;
  return {
    store: {
      ir: sampleIr,
      view: projectIr(sampleIr, activeProjectionId),
      activeProjectionId,
      camera: createInitialCamera(),
      selection: {},
      focusDepth: 2,
      showDiagnostics: true,
      diagnosticSeverity: "all",
      compositionLimit: DEFAULT_COMPOSITION_LIMIT
    }
  };
}
