export type SourceMapDocument = {
  source_map_version: string;
  design_ir: {
    path: string;
    hash?: string;
  };
  mappings: SourceMapMapping[];
};

export type SourceMapMapping = {
  id: string;
  ir_subject: string;
  dsl_location?: SourceMapLocation;
  generated_locations?: GeneratedLocation[];
};

export type GeneratedLocation = {
  artifact: string;
  location: SourceMapLocation;
};

export type SourceMapLocation = {
  uri: string;
  range?: SourceRange;
};

export type SourceRange = {
  start_line: number;
  start_column?: number;
  end_line: number;
  end_column?: number;
};
