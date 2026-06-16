import "./styles.css";
import { mustQuery } from "./app/dom";
import { mountShell } from "./app/shell";
import { startViewer } from "./app/viewer";

const root = mustQuery<HTMLDivElement>("#app");
startViewer(mountShell(root));
