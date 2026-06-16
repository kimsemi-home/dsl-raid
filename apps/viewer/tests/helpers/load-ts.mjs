import { mkdir, readFile, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import path from "node:path";
import { pathToFileURL } from "node:url";
import ts from "typescript";

const outRoot = path.join(tmpdir(), "dslraid-viewer-ts");
const compilerOptions = {
  module: ts.ModuleKind.ESNext,
  target: ts.ScriptTarget.ES2022,
  verbatimModuleSyntax: false
};

export async function loadTs(sourcePath) {
  const outPath = await transpileTs(sourcePath, new Set());
  return import(`${pathToFileURL(outPath).href}?v=${Date.now()}`);
}

async function transpileTs(sourcePath, seen) {
  const normalized = path.posix.normalize(sourcePath);
  if (seen.has(normalized)) {
    return outputPath(normalized);
  }
  seen.add(normalized);

  const source = await readFile(new URL(`../../${normalized}`, import.meta.url), "utf8");
  const js = ts.transpileModule(source, {
    compilerOptions
  }).outputText;
  await Promise.all(relativeImports(js).map((item) => transpileTs(resolvePath(normalized, item), seen)));

  const outPath = outputPath(normalized);
  await mkdir(path.dirname(outPath), { recursive: true });
  await writeFile(outPath, rewriteImports(js), "utf8");
  return outPath;
}

function relativeImports(js) {
  const matches = js.matchAll(/from "(\.{1,2}\/[^"]+)"/g);
  return [...matches].map((match) => match[1]).filter((item) => !path.extname(item));
}

function resolvePath(sourcePath, specifier) {
  return path.posix.normalize(`${path.posix.dirname(sourcePath)}/${specifier}.ts`);
}

function rewriteImports(js) {
  return js.replace(/from "(\.{1,2}\/[^"]+)"/g, (_match, specifier) => {
    return `from "${path.extname(specifier) ? specifier : `${specifier}.js`}"`;
  });
}

function outputPath(sourcePath) {
  return path.join(outRoot, sourcePath.replace(/\.ts$/, ".js"));
}
