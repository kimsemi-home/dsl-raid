import { cursorSubject, type CursorStep } from "../graph/subject-cursor";
import { visibleSubjects } from "../graph/visible-subjects";
import type { ViewerSession } from "./session";
import { visibleView } from "./visible-view";

export function selectRelativeSubject(session: ViewerSession, step: CursorStep): void {
  const subjects = visibleSubjects(visibleView(session.store)).map((item) => item.subject);
  const subject = cursorSubject(subjects, session.store.selection.selected, step);
  if (subject) {
    session.store.selection.selected = subject;
    session.store.selection.related = undefined;
  }
}
