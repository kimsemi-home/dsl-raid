export type DefinedAt = {
  uri: string;
  range?: {
    start_line?: number;
    start_column?: number;
    end_line?: number;
    end_column?: number;
  };
};

export type Expression = {
  language: string;
  source: string;
};
