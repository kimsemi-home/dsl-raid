import type { Point } from "../types";

export function relativePoint(canvas: HTMLCanvasElement, event: MouseEvent | PointerEvent | WheelEvent): Point {
  const rect = canvas.getBoundingClientRect();
  return {
    x: event.clientX - rect.left,
    y: event.clientY - rect.top
  };
}
