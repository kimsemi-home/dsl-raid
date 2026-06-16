import type { CoverageOverlay, CoreIr, Point, SourceMapDocument } from "../types";

export type ViewerActions = {
  setIr: (ir: CoreIr, coverage?: CoverageOverlay, sourceMap?: SourceMapDocument) => void;
  setCoverage: (coverage: CoverageOverlay) => void;
  setSourceMap: (sourceMap: SourceMapDocument) => void;
  openProjection: (projectionId: string) => void;
  openFsm: (fsmId: string) => void;
  select: (subject: string | undefined) => void;
  hover: (subject: string | undefined) => void;
  pan: (dx: number, dy: number) => void;
  zoom: (point: Point, factor: number) => void;
  fit: () => void;
  setDiagnosticsVisible: (visible: boolean) => void;
  setFocusDepth: (depth: 1 | 2) => void;
  updateStatus: (world: Point) => void;
  syncPanels: () => void;
  queueRender: () => void;
};
