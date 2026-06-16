export type SelectSubject = (subject: string | undefined, related?: string[]) => void;

export function bindSubjectButtons(container: Element, onSelect: SelectSubject): void {
  container.querySelectorAll<HTMLButtonElement>("[data-subject]").forEach((button) => {
    button.addEventListener("click", () => onSelect(button.dataset.subject, relatedSubjects(button)));
  });
}

export function relatedSubjects(button: HTMLElement): string[] | undefined {
  const value = button.dataset.relatedSubjects?.trim();
  return value ? value.split(" ").filter(Boolean) : undefined;
}
