export type HitResult =
  | { kind: "node"; subject: string; id: string }
  | { kind: "edge"; subject: string; id: string }
  | undefined;
