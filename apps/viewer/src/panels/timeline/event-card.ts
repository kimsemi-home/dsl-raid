import type { RuntimeEvent } from "../../types";
import { escapeHtml } from "../html";
import { eventClass, eventSubject } from "./relation";

export function eventHtml(event: RuntimeEvent, selected?: string): string {
  const subject = eventSubject(event);
  return `
    <button class="${escapeHtml(eventClass(event, selected))}" data-subject="${escapeHtml(subject)}">
      <span>${escapeHtml(timeLabel(event.timestamp))}</span>
      <strong>${escapeHtml(event.kind.replaceAll("_", " "))}</strong>
      <small>${escapeHtml(subject || event.id)}</small>
    </button>
  `;
}

function timeLabel(timestamp: string): string {
  return timestamp.split("T")[1]?.replace("Z", "") ?? timestamp;
}
