import type { Command, InspectorPanel } from "../../types";

export function commandPanel(command: Command): InspectorPanel {
  return {
    subject: command.id,
    title: command.name,
    sections: [
      {
        title: "Command",
        rows: [
          { label: "Capability", value: command.capability ?? "none", subject: command.capability },
          { label: "Visibility", value: command.visibility ?? "unspecified" },
          { label: "Tags", value: (command.tags ?? []).join(", ") || "none" }
        ]
      }
    ]
  };
}
