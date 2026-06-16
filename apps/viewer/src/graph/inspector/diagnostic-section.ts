import type { CoreIr, InspectorSection } from "../../types";

export function diagnosticSection(ir: CoreIr, subject: string): InspectorSection | undefined {
  const diagnostics = (ir.diagnostics ?? []).filter((diagnostic) => diagnostic.subjects?.includes(subject));
  if (diagnostics.length === 0) {
    return undefined;
  }
  return {
    title: "Diagnostics",
    rows: diagnostics.flatMap((diagnostic) => [
      { label: diagnostic.code, value: `${diagnostic.severity}: ${diagnostic.message}`, subject: diagnostic.id },
      ...(diagnostic.suggestion ? [{ label: "Suggestion", value: diagnostic.suggestion }] : [])
    ])
  };
}
