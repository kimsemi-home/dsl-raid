import type { Camera, CoreIr, CoverageOverlay, RuntimeTrace, SelectionState, SourceMapDocument, ViewModel } from "../types";

export const DEFAULT_COMPOSITION_LIMIT = 48;

export type AppStore = {
  ir: CoreIr;
  coverage?: CoverageOverlay;
  trace?: RuntimeTrace;
  sourceMap?: SourceMapDocument;
  view: ViewModel;
  activeProjectionId?: string;
  camera: Camera;
  selection: SelectionState;
  focusDepth: 1 | 2;
  showDiagnostics: boolean;
  compositionLimit: number;
};

export function createInitialCamera(): Camera {
  return {
    zoom: 1,
    panX: 40,
    panY: 30
  };
}
