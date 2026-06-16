import type { InspectorPanel, Policy } from "../../types";

export function policyPanel(policy: Policy): InspectorPanel {
  return {
    subject: policy.id,
    title: policy.name,
    sections: [
      {
        title: "Policy",
        rows: [
          { label: "Kind", value: policy.kind },
          { label: "Visibility", value: policy.visibility ?? "unspecified" },
          { label: "Tags", value: (policy.tags ?? []).join(", ") || "none" }
        ]
      },
      {
        title: "Applies To",
        rows: appliesRows(policy)
      }
    ]
  };
}

function appliesRows(policy: Policy) {
  const subjects = policy.applies_to ?? [];
  if (subjects.length === 0) {
    return [{ label: "Subject", value: "none" }];
  }
  return subjects.map((subject) => ({
    label: "Subject",
    value: subject,
    subject
  }));
}
