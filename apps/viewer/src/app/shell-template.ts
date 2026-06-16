import { sidebarTemplate } from "./sidebar-template";
import { workspaceTemplate } from "./workspace-template";

export function shellTemplate(): string {
  return `
    <div class="shell">
      ${sidebarTemplate()}
      ${workspaceTemplate()}
    </div>
  `;
}
