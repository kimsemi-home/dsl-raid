import type { Derivation, InspectorPanel } from "../../types";

export function derivationPanel(derivation: Derivation): InspectorPanel {
  return {
    subject: derivation.id,
    title: derivation.id,
    sections: [
      {
        title: "Derivation",
        rows: [
          { label: "Source", value: derivation.source, subject: derivation.source },
          { label: "Rule", value: derivation.rule.id },
          { label: "Kind", value: derivation.rule.kind },
          { label: "Generator", value: derivation.rule.generator ?? "unspecified" },
          { label: "Version", value: derivation.rule.version ?? "unspecified" }
        ]
      },
      {
        title: "Targets",
        rows: targetRows(derivation)
      }
    ]
  };
}

function targetRows(derivation: Derivation) {
  const targets = derivation.targets ?? [];
  if (targets.length === 0) {
    return [{ label: "Target", value: "none" }];
  }
  return targets.map((target) => ({
    label: target.role,
    value: target.artifact,
    subject: target.artifact
  }));
}
