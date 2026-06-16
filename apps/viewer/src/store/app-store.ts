import type { Camera, CoreIr, CoverageOverlay, SelectionState, SourceMapDocument, ViewModel } from "../types";

export type AppStore = {
  ir: CoreIr;
  coverage?: CoverageOverlay;
  sourceMap?: SourceMapDocument;
  view: ViewModel;
  activeProjectionId?: string;
  camera: Camera;
  selection: SelectionState;
  focusDepth: 1 | 2;
  showDiagnostics: boolean;
};

export function createInitialCamera(): Camera {
  return {
    zoom: 1,
    panX: 40,
    panY: 30
  };
}
