export function mustQuery<T extends Element>(selector: string): T {
  const element = document.querySelector<T>(selector);
  if (!element) {
    throw new Error(`Missing required DOM node: ${selector}`);
  }
  return element;
}
