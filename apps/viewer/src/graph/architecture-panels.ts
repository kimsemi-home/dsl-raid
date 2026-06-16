import type { CoreIr, InspectorPanel } from "../types";
import { artifactPanel } from "./inspector/artifact-panel";
import { capabilityPanel } from "./inspector/capability-panel";
import { commandPanel } from "./inspector/command-panel";
import { contextPanel } from "./inspector/context-panel";
import { derivationPanel } from "./inspector/derivation-panel";
import { policyPanel } from "./inspector/policy-panel";
import { projectPanel } from "./inspector/project-panel";
import { requirementPanel } from "./inspector/requirement-panel";

export function architecturePanels(ir: CoreIr): InspectorPanel[] {
  return [
    projectPanel(ir),
    ...(ir.contexts ?? []).map(contextPanel),
    ...(ir.requirements ?? []).map(requirementPanel),
    ...(ir.capabilities ?? []).map(capabilityPanel),
    ...(ir.commands ?? []).map(commandPanel),
    ...(ir.policies ?? []).map(policyPanel),
    ...(ir.derivations ?? []).map(derivationPanel),
    ...(ir.artifacts ?? []).map((artifact) => artifactPanel(ir, artifact))
  ];
}
