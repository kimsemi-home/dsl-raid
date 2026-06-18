export type Requirement = {
  id: string;
  name: string;
  description?: string;
  satisfied_by?: string[];
  visibility?: string;
  tags?: string[];
};
