export function toneStroke(tone: string): string {
  switch (tone) {
    case "success":
      return "#0f766e";
    case "warning":
      return "#b45309";
    case "danger":
      return "#b91c1c";
    case "muted":
      return "#737373";
    default:
      return "#404040";
  }
}

export function badgeFill(tone: string, badge: string): string {
  if (badge === "generated") {
    return "#dbeafe";
  }
  if (badge === "covered" || badge === "deployed" || badge === "tested" || tone === "success") {
    return "#ccfbf1";
  }
  if (badge === "uncovered" || badge === "not_deployed") {
    return "#e5e7eb";
  }
  if (badge === "failed" || tone === "danger") {
    return "#fee2e2";
  }
  if (badge === "flaky") {
    return "#fef3c7";
  }
  return "#ebe6da";
}
