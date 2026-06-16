import type { InspectorPanel, Requirement } from "../../types";

export function requirementPanel(requirement: Requirement): InspectorPanel {
  return {
    subject: requirement.id,
    title: requirement.name,
    sections: [
      {
        title: "Requirement",
        rows: [
          { label: "Description", value: requirement.description ?? "not documented" },
          { label: "Visibility", value: requirement.visibility ?? "unspecified" },
          { label: "Tags", value: (requirement.tags ?? []).join(", ") || "none" }
        ]
      },
      {
        title: "Satisfied By",
        rows: satisfiedRows(requirement)
      }
    ]
  };
}

function satisfiedRows(requirement: Requirement) {
  const subjects = requirement.satisfied_by ?? [];
  if (subjects.length === 0) {
    return [{ label: "Subject", value: "none" }];
  }
  return subjects.map((subject) => ({
    label: "Subject",
    value: subject,
    subject
  }));
}
