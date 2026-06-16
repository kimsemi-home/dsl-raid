export type RuntimeTrace = {
  trace_version: string;
  design_ir?: {
    path: string;
    hash?: string;
  };
  run: TraceRun;
  events: RuntimeEvent[];
};

export type TraceRun = {
  id: string;
  started_at?: string;
  ended_at?: string;
  environment?: string;
};

export type RuntimeEvent = {
  id: string;
  timestamp: string;
  kind: RuntimeEventKind;
  subject?: string;
  from?: string;
  to?: string;
  status?: RuntimeStatus;
  duration_ms?: number;
  visibility?: "public" | "internal" | "private" | "secret";
};

export type RuntimeEventKind =
  | "event_received"
  | "state_entered"
  | "state_exited"
  | "transition_started"
  | "transition_completed"
  | "transition_failed"
  | "action_started"
  | "action_completed"
  | "artifact_deployed"
  | "diagnostic_emitted";

export type RuntimeStatus = "ok" | "failed" | "timeout" | "cancelled" | "policy_blocked" | "degraded";
