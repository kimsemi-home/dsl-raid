export function fixtureView() {
  return {
    nodes: [
      node("starting", 0, 0),
      node("running", 24, 16),
      node("completed", 300, 90)
    ],
    edges: [edge("running_to_completed", "running", "completed")]
  };
}

function node(id, x, y) {
  return {
    id: `node:${id}`,
    subject: `state:runtime.${id}`,
    x,
    y,
    width: 100,
    height: 56,
    label: id,
    badges: []
  };
}

function edge(id, from, to) {
  return {
    id: `edge:${id}`,
    subject: `transition:runtime.${id}`,
    from: `node:${from}`,
    to: `node:${to}`,
    route: [{ x: 124, y: 44 }, { x: 270, y: 100 }],
    label: id
  };
}
