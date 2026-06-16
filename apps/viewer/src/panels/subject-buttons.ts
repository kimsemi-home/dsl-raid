export type SelectSubject = (subject: string | undefined) => void;

export function bindSubjectButtons(container: Element, onSelect: SelectSubject): void {
  container.querySelectorAll<HTMLButtonElement>("[data-subject]").forEach((button) => {
    button.addEventListener("click", () => onSelect(button.dataset.subject));
  });
}
