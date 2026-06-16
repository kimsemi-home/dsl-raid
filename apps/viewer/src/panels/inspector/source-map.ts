import type { InspectorSection, SourceMapDocument, SourceMapLocation } from "../../types";

export function sourceMapSection(subject: string, sourceMap: SourceMapDocument | undefined): InspectorSection | undefined {
  const mapping = sourceMap?.mappings.find((candidate) => candidate.ir_subject === subject);
  if (!mapping) {
    return undefined;
  }
  const rows = [];
  if (mapping.dsl_location) {
    rows.push({ label: "DSL", value: formatLocation(mapping.dsl_location) });
  }
  for (const generated of mapping.generated_locations ?? []) {
    rows.push({
      label: artifactLabel(generated.artifact),
      value: formatLocation(generated.location),
      subject: generated.artifact
    });
  }
  return rows.length > 0 ? { title: "Source Map", rows } : undefined;
}

function formatLocation(location: SourceMapLocation): string {
  const range = location.range;
  return range ? `${location.uri}:${range.start_line}-${range.end_line}` : location.uri;
}

function artifactLabel(artifact: string): string {
  const local = artifact.includes(":") ? artifact.split(":")[1] : artifact;
  return local.replaceAll("_", ".");
}
