export function stateSubject(fsmId: string, stateId: string): string {
  return `state:${fsmLocalName(fsmId)}.${stateId}`;
}

export function transitionSubject(fsmId: string, transitionId: string): string {
  return `transition:${fsmLocalName(fsmId)}.${transitionId}`;
}

export function eventSubject(fsmId: string, eventId: string): string {
  return `event:${fsmLocalName(fsmId)}.${eventId}`;
}

export function layoutStateId(fsmId: string, stateId: string): string {
  return `layout:${fsmLocalName(fsmId)}.state.${stateId}`;
}

export function layoutTransitionId(fsmId: string, transitionId: string): string {
  return `layout:${fsmLocalName(fsmId)}.transition.${transitionId}`;
}

function fsmLocalName(fsmId: string): string {
  return fsmId.startsWith("fsm:") ? fsmId.slice(4) : fsmId;
}
