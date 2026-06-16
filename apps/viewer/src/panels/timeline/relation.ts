import type { RuntimeEvent } from "../../types";

export function relatedEvents(events: RuntimeEvent[], subject?: string): RuntimeEvent[] {
  if (!subject) {
    return [];
  }
  return events.filter((event) => eventSubjects(event).includes(subject));
}

export function eventClass(event: RuntimeEvent, subject?: string): string {
  const classes = ["timeline-event", event.status ?? "ok"];
  if (subject && eventSubjects(event).includes(subject)) {
    classes.push("related");
  }
  return classes.join(" ");
}

export function eventSubject(event: RuntimeEvent): string {
  return event.subject ?? event.to ?? event.from ?? "";
}

function eventSubjects(event: RuntimeEvent): string[] {
  return [...new Set([event.subject, event.from, event.to].filter(Boolean) as string[])];
}
