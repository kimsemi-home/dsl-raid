import type { Capability, InspectorPanel } from "../../types";

export function capabilityPanel(capability: Capability): InspectorPanel {
  return {
    subject: capability.id,
    title: capability.name,
    sections: [
      {
        title: "Capability",
        rows: [
          { label: "Kind", value: capability.kind },
          { label: "Owner", value: capability.owner ?? "none", subject: capability.owner },
          { label: "Visibility", value: capability.visibility ?? "unspecified" },
          { label: "Tags", value: (capability.tags ?? []).join(", ") || "none" }
        ]
      },
      {
        title: "Provides",
        rows: subjectRows(capability.provides ?? [])
      },
      {
        title: "Requires",
        rows: subjectRows(capability.requires ?? [])
      }
    ]
  };
}

function subjectRows(subjects: string[]) {
  if (subjects.length === 0) {
    return [{ label: "Subject", value: "none" }];
  }
  return subjects.map((subject) => ({
    label: "Subject",
    value: subject,
    subject
  }));
}
