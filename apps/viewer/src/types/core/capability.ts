export type Capability = {
  id: string;
  name: string;
  kind: string;
  owner?: string;
  provides?: string[];
  requires?: string[];
  visibility?: string;
  tags?: string[];
};
