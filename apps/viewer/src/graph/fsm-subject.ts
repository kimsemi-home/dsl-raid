import type { CoreIr } from "../types";

const FSM_SUBJECT_KINDS = new Set(["state", "transition", "event", "guard", "action"]);

export function fsmIdForSubject(ir: CoreIr, subject: string | undefined): string | undefined {
  if (!subject) {
    return undefined;
  }
  if (subject.startsWith("fsm:")) {
    return hasFsm(ir, subject) ? subject : undefined;
  }
  const [kind, rest] = subject.split(":", 2);
  if (!FSM_SUBJECT_KINDS.has(kind) || !rest) {
    return undefined;
  }
  const local = rest.split(".", 1)[0];
  return findFsm(ir, local)?.id;
}

export function projectionForSubject(ir: CoreIr, subject: string | undefined): string | undefined {
  const fsmId = fsmIdForSubject(ir, subject);
  return ir.projections?.find((projection) => projection.source === fsmId)?.id;
}

function hasFsm(ir: CoreIr, fsmId: string): boolean {
  return (ir.fsms ?? []).some((fsm) => fsm.id === fsmId);
}

function findFsm(ir: CoreIr, local: string) {
  return (ir.fsms ?? []).find((fsm) => fsm.id === `fsm:${local}` || fsm.id === local);
}
