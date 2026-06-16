import type { SelectionState } from "../types";
import { toneStroke } from "./style";

export type SubjectVisualState = {
  selected: boolean;
  hovered: boolean;
  related: boolean;
};

export function subjectVisualState(selection: SelectionState, subject: string): SubjectVisualState {
  return {
    selected: selection.selected === subject,
    hovered: selection.hovered === subject,
    related: selection.related?.includes(subject) ?? false
  };
}

export function activeStroke(tone: string, state: SubjectVisualState): string {
  if (state.selected) {
    return "#0f766e";
  }
  if (state.hovered) {
    return "#b45309";
  }
  return state.related ? "#b91c1c" : toneStroke(tone);
}

export function activeLineWidth(state: SubjectVisualState, strong = false): number {
  return state.selected || state.hovered || state.related || strong ? 3 : 2;
}
