export type Project = {
  id: string;
  name: string;
  visibility?: string;
  tags?: string[];
  metadata?: Record<string, unknown>;
};
