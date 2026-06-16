import type { Composition, CoreIr, Projection, ViewModel } from "../types";
import { architecturePanels } from "./architecture-panels";
import { compositionEdges } from "./composition-edges";
import { materializeComposition } from "./composition-materialize";
import { compositionNodes } from "./composition-nodes";
import { compositionPanels } from "./composition-panels";

export function projectComposition(
  ir: CoreIr,
  projection: Projection,
  composition: Composition
): ViewModel {
  const fsms = (composition.inputs ?? []).flatMap((id) => (ir.fsms ?? []).find((fsm) => fsm.id === id) ?? []);
  const result = materializeComposition(composition, fsms);
  const nodes = compositionNodes(result.nodes);
  const edges = compositionEdges(result.edges, nodes);
  const panels = compositionPanels(composition, result);
  return {
    view_version: "0.1.0",
    source: {
      core_ir: "loaded",
      projection: projection.id
    },
    layout: {
      engine: "bounded-reachable-product",
      version: "0.1.0"
    },
    nodes,
    edges,
    inspector_panels: [
      ...panels,
      ...architecturePanels(ir).filter((panel) => panel.subject !== composition.id)
    ]
  };
}
