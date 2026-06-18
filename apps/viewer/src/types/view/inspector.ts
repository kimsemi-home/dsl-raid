export type InspectorPanel = {
  subject: string;
  title: string;
  sections: InspectorSection[];
};

export type InspectorSection = {
  title: string;
  rows: InspectorRow[];
};

export type InspectorRow = {
  label: string;
  value: string;
  subject?: string;
};
