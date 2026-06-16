import type { Projection } from "../types";

export const sampleProjections: Projection[] = [
  {
    id: "view:runtime",
    kind: "projection",
    source: "fsm:runtime",
    show: ["states", "transitions", "events", "policies", "artifacts"]
  },
  {
    id: "view:agent",
    kind: "projection",
    source: "fsm:agent",
    show: ["states", "transitions", "events", "artifacts"]
  },
  {
    id: "view:workspace",
    kind: "projection",
    source: "fsm:workspace",
    show: ["states", "transitions", "events", "artifacts"]
  }
];
