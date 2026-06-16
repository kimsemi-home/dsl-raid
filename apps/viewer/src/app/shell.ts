import type { ViewerElements } from "./elements";
import { collectElements } from "./elements";
import { shellTemplate } from "./shell-template";

export function mountShell(root: HTMLDivElement): ViewerElements {
  root.innerHTML = shellTemplate();
  return collectElements();
}
