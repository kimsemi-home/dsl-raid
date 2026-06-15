import type { Camera, CoreIr, SelectionState, ViewModel } from "../types";

export type AppStore = {
  ir: CoreIr;
  view: ViewModel;
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
