import type { InspectorPanel } from "./inspector";
import type { SceneEdge, SceneNode } from "./scene";

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
