import type { Artifact, CoreIr, InspectorRow } from "../types";

export function artifactsForSubject(ir: CoreIr, subject: string): Artifact[] {
  const derivationIds = new Set(
    (ir.derivations ?? [])
      .filter((derivation) => derivation.source === subject || derivation.targets?.some((target) => target.artifact === subject))
      .map((derivation) => derivation.id)
  );
  return (ir.artifacts ?? []).filter((artifact) => artifact.generated_by && derivationIds.has(artifact.generated_by));
}

export function artifactRow(artifact: Artifact): InspectorRow {
  return {
    label: artifact.kind,
    value: artifact.path,
    subject: artifact.id
  };
}
