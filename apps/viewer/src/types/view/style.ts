import type { CoverageStatus } from "../coverage";

export type StyleToken = {
  tone?: "default" | "success" | "warning" | "danger" | "muted";
  emphasis?: "normal" | "strong" | "faint";
  coverage?: CoverageStatus;
};
