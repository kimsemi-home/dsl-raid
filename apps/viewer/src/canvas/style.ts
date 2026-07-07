const graphToneTokens = {
  default: { stroke: "#404040" },
  success: { stroke: "#0f766e", badgeFill: "#ccfbf1" },
  warning: { stroke: "#b45309", badgeFill: "#fef3c7" },
  danger: { stroke: "#b91c1c", badgeFill: "#fee2e2" },
  muted: { stroke: "#737373", badgeFill: "#e5e7eb" },
};

const graphBadgeTokens = {
  generated: { fill: "#dbeafe" },
  neutral: { fill: "#ebe6da" },
};

export function toneStroke(tone: string): string {
  switch (tone) {
    case "success":
      return graphToneTokens.success.stroke;
    case "warning":
      return graphToneTokens.warning.stroke;
    case "danger":
      return graphToneTokens.danger.stroke;
    case "muted":
      return graphToneTokens.muted.stroke;
    default:
      return graphToneTokens.default.stroke;
  }
}

export function badgeFill(tone: string, badge: string): string {
  if (badge === "generated") {
    return graphBadgeTokens.generated.fill;
  }
  if (badge === "covered" || badge === "deployed" || badge === "tested" || tone === "success") {
    return graphToneTokens.success.badgeFill;
  }
  if (badge === "uncovered" || badge === "not_deployed") {
    return graphToneTokens.muted.badgeFill;
  }
  if (badge === "failed" || tone === "danger") {
    return graphToneTokens.danger.badgeFill;
  }
  if (badge === "flaky") {
    return graphToneTokens.warning.badgeFill;
  }
  return graphBadgeTokens.neutral.fill;
}
