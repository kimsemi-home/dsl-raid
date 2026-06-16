import { followSubject } from "../graph/follow-subject";
import type { ViewerSession } from "./session";
import { visibleView } from "./visible-view";

export function followSelectedSubject(session: ViewerSession): void {
  const subject = followSubject(visibleView(session.store), session.store.selection.selected);
  if (subject) {
    session.store.selection.selected = subject;
  }
}
