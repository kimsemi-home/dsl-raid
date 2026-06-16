import type { Projection } from "../types";

export const sampleProjections: Projection[] = [
  {
    id: "view:runtime",
    kind: "projection",
    source: "fsm:runtime",
    show: ["states", "transitions", "events", "policies", "artifacts"]
  }
];
