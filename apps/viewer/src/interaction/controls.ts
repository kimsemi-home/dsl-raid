import type { ViewerActions } from "../app/actions";
import type { ViewerElements } from "../app/elements";
import type { ViewerSession } from "../app/session";
import { bindCanvasControls } from "./canvas-controls";
import { bindFileControls } from "./file-controls";
import { bindKeyboardControls } from "./keyboard-controls";
import { bindSearchControls } from "./search-controls";
import { bindToolbarControls } from "./toolbar-controls";
import { bindWindowControls } from "./window-controls";

export function bindControls(elements: ViewerElements, session: ViewerSession, actions: ViewerActions): void {
  bindCanvasControls(elements, session, actions);
  bindToolbarControls(elements, actions);
  bindFileControls(elements, actions);
  bindKeyboardControls(elements, actions);
  bindSearchControls(elements, actions);
  bindWindowControls(actions);
}
