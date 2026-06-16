export type CoverageOverlay = {
  coverage_version: string;
  design_ir: {
    path: string;
    hash?: string;
  };
  traces?: Array<{
    path: string;
    hash?: string;
  }>;
  subjects: CoverageSubject[];
  metadata?: Record<string, unknown>;
};

export type CoverageSubject = {
  subject: string;
  kind: "state" | "transition" | "event" | "guard" | "action" | "artifact";
  status: CoverageStatus;
  count?: number;
  failure_rate?: number;
  last_seen?: string;
  diagnostics?: string[];
};

export type CoverageStatus = "covered" | "uncovered" | "failed" | "flaky" | "deployed" | "not_deployed";
