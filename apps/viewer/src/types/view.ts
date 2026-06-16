import type { CoverageStatus } from "./coverage";

export type ViewModel = {
  view_version: string;
  source: {
    core_ir: string;
    projection: string;
    hash?: string;
  };
  layout: {
    engine: string;
    version: string;
  };
  nodes: SceneNode[];
  edges: SceneEdge[];
  inspector_panels: InspectorPanel[];
};

export type SceneNode = {
  id: string;
  subject: string;
  x: number;
  y: number;
  width: number;
  height: number;
  label: string;
  badges: string[];
  style?: StyleToken;
};

export type SceneEdge = {
  id: string;
  subject: string;
  from: string;
  to: string;
  label?: string;
  route: Point[];
  style?: StyleToken;
};

export type Point = {
  x: number;
  y: number;
};

export type StyleToken = {
  tone?: "default" | "success" | "warning" | "danger" | "muted";
  emphasis?: "normal" | "strong" | "faint";
  coverage?: CoverageStatus;
};

export type InspectorPanel = {
  subject: string;
  title: string;
  sections: InspectorSection[];
};

export type InspectorSection = {
  title: string;
  rows: InspectorRow[];
};

export type InspectorRow = {
  label: string;
  value: string;
  subject?: string;
};
