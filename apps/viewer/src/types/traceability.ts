export type Derivation = {
  id: string;
  source: string;
  rule: {
    id: string;
    kind: string;
    generator?: string;
    version?: string;
  };
  targets?: Array<{
    artifact: string;
    role: string;
  }>;
};

export type Artifact = {
  id: string;
  kind: string;
  path: string;
  generated_by?: string;
  visibility?: string;
  tags?: string[];
};

export type Diagnostic = {
  id: string;
  code: string;
  severity: "hint" | "info" | "warning" | "error";
  message: string;
  subjects?: string[];
  suggestion?: string;
};
