import type { CoreIr, InspectorPanel } from "../../types";

export function projectPanel(ir: CoreIr): InspectorPanel {
  return {
    subject: `project:${ir.project.id}`,
    title: ir.project.name,
    sections: [
      {
        title: "Project",
        rows: [
          { label: "ID", value: ir.project.id },
          { label: "Visibility", value: ir.project.visibility ?? "unspecified" },
          { label: "Tags", value: (ir.project.tags ?? []).join(", ") || "none" }
        ]
      },
      {
        title: "Inventory",
        rows: [
          { label: "FSMs", value: String((ir.fsms ?? []).length) },
          { label: "Policies", value: String((ir.policies ?? []).length) },
          { label: "Artifacts", value: String((ir.artifacts ?? []).length) },
          { label: "Derivations", value: String((ir.derivations ?? []).length) }
        ]
      }
    ]
  };
}
