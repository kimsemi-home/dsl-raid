import type { ContextObject, InspectorPanel } from "../../types";

export function contextPanel(context: ContextObject): InspectorPanel {
  return {
    subject: context.id,
    title: context.name,
    sections: [
      {
        title: "Context",
        rows: [
          { label: "Kind", value: context.kind },
          { label: "Owns", value: String((context.owns ?? []).length) }
        ]
      },
      {
        title: "Owned Subjects",
        rows: ownedRows(context)
      }
    ]
  };
}

function ownedRows(context: ContextObject) {
  const subjects = context.owns ?? [];
  if (subjects.length === 0) {
    return [{ label: "Subject", value: "none" }];
  }
  return subjects.map((subject) => ({
    label: "Subject",
    value: subject,
    subject
  }));
}
