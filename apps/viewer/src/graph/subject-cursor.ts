export type CursorStep = -1 | 1;

export function cursorSubject(subjects: string[], selected: string | undefined, step: CursorStep): string | undefined {
  if (subjects.length === 0) {
    return undefined;
  }
  const selectedIndex = subjects.indexOf(selected ?? "");
  const start = selectedIndex < 0 ? edgeIndex(subjects, step) : selectedIndex;
  return subjects[wrap(start + step, subjects.length)];
}

function edgeIndex(subjects: string[], step: CursorStep): number {
  return step > 0 ? -1 : subjects.length;
}

function wrap(index: number, length: number): number {
  return ((index % length) + length) % length;
}
