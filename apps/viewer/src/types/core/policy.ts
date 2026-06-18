export type Policy = {
  id: string;
  name: string;
  kind: string;
  applies_to?: string[];
  visibility?: string;
  tags?: string[];
};
