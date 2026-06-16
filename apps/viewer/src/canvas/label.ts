import { roundedRect } from "./shape";

export function drawLabel(context: CanvasRenderingContext2D, label: string, x: number, y: number): void {
  context.font = "12px Inter, ui-sans-serif, system-ui";
  const width = context.measureText(label).width + 14;
  context.fillStyle = "rgba(255, 254, 250, 0.92)";
  roundedRect(context, x - width / 2, y - 13, width, 20, 6);
  context.fill();
  context.fillStyle = "#334155";
  context.fillText(label, x - width / 2 + 7, y + 2);
}
