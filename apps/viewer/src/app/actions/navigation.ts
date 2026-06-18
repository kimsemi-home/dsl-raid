import type { ViewerActions } from "../action-types";
import { setCompositionLimit } from "../composition-limit";
import { followSelectedSubject } from "../follow-selected";
import { openFsm } from "../open-fsm";
import { selectRelativeSubject } from "../select-relative";
import { selectSubject } from "../select-subject";
import * as viewerSession from "../session";

type NavigationActions = Pick<ViewerActions, "followSelected" | "openFsm" | "openProjection" | "select" | "selectRelative" | "setCompositionLimit">;

export function navigationActions(session: viewerSession.ViewerSession, actions: ViewerActions, refresh: () => void): NavigationActions {
  return {
    openProjection: (projectionId: string) => {
      viewerSession.setProjection(session, projectionId);
      actions.fit();
    },
    openFsm: (fsmId: string) => openFsm(actions, session, fsmId),
    select: (subject: string | undefined, related?: string[]) => (selectSubject(session, subject, related) ? actions.fit() : refresh()),
    selectRelative: (step: -1 | 1) => {
      selectRelativeSubject(session, step);
      refresh();
    },
    followSelected: () => {
      followSelectedSubject(session);
      refresh();
    },
    setCompositionLimit: (limit: number) => {
      setCompositionLimit(session, limit);
      actions.fit();
    }
  };
}
