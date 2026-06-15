import type {
  Artifact,
  CoreIr,
  Fsm,
  InspectorPanel,
  Projection,
  SceneEdge,
  SceneNode,
  Transition,
  ViewModel
} from "../types";

export function projectIr(ir: CoreIr, projectionId?: string): ViewModel {
  const projection = selectProjection(ir, projectionId);
  const fsm = (ir.fsms ?? []).find((candidate) => candidate.id === projection.source);
  if (!fsm) {
    throw new Error(`Projection source is not an FSM: ${projection.source}`);
  }
  return projectFsm(ir, projection, fsm);
}

function selectProjection(ir: CoreIr, projectionId?: string): Projection {
  const projections = ir.projections ?? [];
  const projection = projectionId
    ? projections.find((candidate) => candidate.id === projectionId)
    : projections[0];
  if (!projection) {
    throw new Error("No projection is available in this IR.");
  }
  return projection;
}

function projectFsm(ir: CoreIr, projection: Projection, fsm: Fsm): ViewModel {
  const nodes: SceneNode[] = [];
  const edges: SceneEdge[] = [];
  const panels: InspectorPanel[] = [fsmPanel(ir, fsm)];
  const states = fsm.states ?? [];
  const transitions = fsm.transitions ?? [];
  const width = 168;
  const height = 58;
  const colGap = 230;
  const rowGap = 150;

  states.forEach((state, index) => {
    const x = 80 + (index % 3) * colGap;
    const y = 90 + Math.floor(index / 3) * rowGap;
    const badges = [
      ...(state.initial ? ["initial"] : []),
      ...(state.terminal ? [state.terminal_semantics ?? "terminal"] : []),
      ...(state.tags ?? [])
    ];
    const subject = stateSubject(fsm.id, state.id);
    nodes.push({
      id: layoutStateId(fsm.id, state.id),
      subject,
      x,
      y,
      width,
      height,
      label: state.id,
      badges,
      style: {
        tone: state.terminal ? (state.terminal_semantics === "failed" ? "danger" : "success") : "default",
        emphasis: state.initial || state.terminal ? "strong" : "normal"
      }
    });
    panels.push(statePanel(ir, fsm, state.id, subject));
  });

  transitions.forEach((transition) => {
    const from = nodes.find((node) => node.subject === stateSubject(fsm.id, transition.from));
    const to = nodes.find((node) => node.subject === stateSubject(fsm.id, transition.to));
    if (!from || !to) {
      return;
    }
    const subject = transitionSubject(fsm.id, transition.id);
    edges.push({
      id: layoutTransitionId(fsm.id, transition.id),
      subject,
      from: from.id,
      to: to.id,
      label: transition.on ?? "epsilon",
      route: [
        { x: from.x + from.width, y: from.y + from.height / 2 },
        { x: to.x, y: to.y + to.height / 2 }
      ],
      style: {
        tone: (transition.requires ?? []).length > 0 ? "warning" : "default",
        emphasis: "normal"
      }
    });
    panels.push(transitionPanel(ir, fsm, transition, subject));
  });

  return {
    view_version: "0.1.0",
    source: {
      core_ir: "loaded",
      projection: projection.id
    },
    layout: {
      engine: "manual",
      version: "0.1.0"
    },
    nodes,
    edges,
    inspector_panels: panels
  };
}

export function subjectsForSearch(view: ViewModel): Array<{ subject: string; label: string; kind: string }> {
  const nodes = view.nodes.map((node) => ({ subject: node.subject, label: node.label, kind: "state" }));
  const edges = view.edges.map((edge) => ({ subject: edge.subject, label: edge.label ?? edge.subject, kind: "transition" }));
  return [...nodes, ...edges].sort((a, b) => a.subject.localeCompare(b.subject));
}

export function stateSubject(fsmId: string, stateId: string): string {
  return `state:${fsmLocalName(fsmId)}.${stateId}`;
}

export function transitionSubject(fsmId: string, transitionId: string): string {
  return `transition:${fsmLocalName(fsmId)}.${transitionId}`;
}

function eventSubject(fsmId: string, eventId: string): string {
  return `event:${fsmLocalName(fsmId)}.${eventId}`;
}

function layoutStateId(fsmId: string, stateId: string): string {
  return `layout:${fsmLocalName(fsmId)}.state.${stateId}`;
}

function layoutTransitionId(fsmId: string, transitionId: string): string {
  return `layout:${fsmLocalName(fsmId)}.transition.${transitionId}`;
}

function fsmLocalName(fsmId: string): string {
  return fsmId.startsWith("fsm:") ? fsmId.slice(4) : fsmId;
}

function fsmPanel(ir: CoreIr, fsm: Fsm): InspectorPanel {
  return {
    subject: fsm.id,
    title: fsm.name,
    sections: [
      {
        title: "Summary",
        rows: [
          { label: "Project", value: ir.project.name, subject: `project:${ir.project.id}` },
          { label: "States", value: String((fsm.states ?? []).length) },
          { label: "Transitions", value: String((fsm.transitions ?? []).length) },
          { label: "Source", value: fsm.defined_at?.uri ?? "not linked" }
        ]
      }
    ]
  };
}

function statePanel(ir: CoreIr, fsm: Fsm, stateId: string, subject: string): InspectorPanel {
  const transitions = fsm.transitions ?? [];
  const incoming = transitions.filter((transition) => transition.to === stateId);
  const outgoing = transitions.filter((transition) => transition.from === stateId);
  const artifacts = artifactsForSubject(ir, subject);
  return {
    subject,
    title: stateId,
    sections: [
      {
        title: "State",
        rows: [
          { label: "Parent FSM", value: fsm.id, subject: fsm.id },
          { label: "Incoming", value: String(incoming.length) },
          { label: "Outgoing", value: String(outgoing.length) }
        ]
      },
      {
        title: "Traceability",
        rows: artifacts.length > 0 ? artifacts.map(artifactRow) : [{ label: "Artifacts", value: "none linked" }]
      }
    ]
  };
}

function transitionPanel(ir: CoreIr, fsm: Fsm, transition: Transition, subject: string): InspectorPanel {
  const requires = transition.requires ?? [];
  return {
    subject,
    title: transition.id,
    sections: [
      {
        title: "Transition",
        rows: [
          { label: "From", value: transition.from, subject: stateSubject(fsm.id, transition.from) },
          { label: "To", value: transition.to, subject: stateSubject(fsm.id, transition.to) },
          {
            label: "Event",
            value: transition.on ?? "epsilon",
            subject: transition.on ? eventSubject(fsm.id, transition.on) : undefined
          }
        ]
      },
      {
        title: "Policy",
        rows:
          requires.length > 0
            ? requires.map((required) => ({ label: "Requires", value: required, subject: required }))
            : [{ label: "Requires", value: "none" }]
      },
      {
        title: "Traceability",
        rows: artifactsForSubject(ir, subject).map(artifactRow)
      }
    ]
  };
}

function artifactsForSubject(ir: CoreIr, subject: string): Artifact[] {
  const derivationIds = new Set(
    (ir.derivations ?? [])
      .filter((derivation) => derivation.source === subject || derivation.targets?.some((target) => target.artifact === subject))
      .map((derivation) => derivation.id)
  );
  return (ir.artifacts ?? []).filter((artifact) => artifact.generated_by && derivationIds.has(artifact.generated_by));
}

function artifactRow(artifact: Artifact) {
  return {
    label: artifact.kind,
    value: artifact.path,
    subject: artifact.id
  };
}
