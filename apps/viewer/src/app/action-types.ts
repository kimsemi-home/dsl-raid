import type { CoverageOverlay, CoreIr, Point, RuntimeTrace, SourceMapDocument } from "../types";

export type ViewerActions = {
  setIr: (ir: CoreIr, coverage?: CoverageOverlay, sourceMap?: SourceMapDocument, trace?: RuntimeTrace) => void;
  setCoverage: (coverage: CoverageOverlay) => void;
  setTrace: (trace: RuntimeTrace) => void;
  setSourceMap: (sourceMap: SourceMapDocument) => void;
  openProjection: (projectionId: string) => void;
  openFsm: (fsmId: string) => void;
  select: (subject: string | undefined, related?: string[]) => void;
  selectRelative: (step: -1 | 1) => void;
  followSelected: () => void;
  hover: (subject: string | undefined) => void;
  pan: (dx: number, dy: number) => void;
  zoom: (point: Point, factor: number) => void;
  fit: () => void;
  setDiagnosticsVisible: (visible: boolean) => void;
  setFocusDepth: (depth: 1 | 2) => void;
  setCompositionLimit: (limit: number) => void;
  updateStatus: (world: Point) => void;
  syncPanels: () => void;
  queueRender: () => void;
};
