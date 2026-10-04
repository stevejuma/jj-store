import { readFileSync, realpathSync } from "node:fs";
import { resolve, relative, isAbsolute } from "node:path";
import ts from "typescript";

export const name = "ui-ownership";
const forbidden =
  /(?:@lapismd\/lapis-community(?:\b|\/)|(?:^|\/)lapis-community(?:\/|$))/u;
const codeFile = /\.(?:[cm]?[jt]sx?|svelte|css|scss)$/u;
const configuration =
  /(?:^|\/)(?:package\.json|pnpm-lock\.yaml|pnpm-workspace\.yaml|\.pnpmfile\.cjs|local-package-pnpmfile\.cjs|(?:vite|tsconfig|spec-validator)[^/]*\.(?:json|[cm]?js|ts))$/u;

export function moduleReferences(source, file) {
  if (/\.(?:css|scss)$/u.test(file))
    return [
      ...source.matchAll(
        /@(?:import|source)\s+(?:url\()?\s*["']([^"']+)["']/gu,
      ),
    ].map((m) => ({ value: m[1], offset: m.index }));
  const text = file.endsWith(".svelte")
    ? source.replace(
        /<script\b[^>]*>([\s\S]*?)<\/script>|[\s\S]+?(?=<script\b|$)/gu,
        (whole, script) =>
          script === undefined
            ? whole.replace(/[^\n]/gu, " ")
            : whole
                .replace(/^[\s\S]*?>/u, (tag) => " ".repeat(tag.length))
                .replace(/<\/script>$/u, "         "),
      )
    : source;
  const tree = ts.createSourceFile(file, text, ts.ScriptTarget.Latest, true);
  const refs = file.endsWith(".svelte")
    ? [...source.matchAll(/<style\b[^>]*>([\s\S]*?)<\/style>/gu)].flatMap(
        (style) =>
          moduleReferences(style[1], "inline.css").map((ref) => ({
            ...ref,
            offset: style.index + ref.offset,
          })),
      )
    : [];
  const visit = (node) => {
    let value;
    if (
      (ts.isImportDeclaration(node) || ts.isExportDeclaration(node)) &&
      node.moduleSpecifier &&
      ts.isStringLiteralLike(node.moduleSpecifier)
    )
      value = node.moduleSpecifier;
    if (
      ts.isCallExpression(node) &&
      (node.expression.kind === ts.SyntaxKind.ImportKeyword ||
        (ts.isIdentifier(node.expression) &&
          node.expression.text === "require")) &&
      node.arguments.length === 1 &&
      ts.isStringLiteralLike(node.arguments[0])
    )
      value = node.arguments[0];
    if (
      ts.isImportTypeNode(node) &&
      ts.isLiteralTypeNode(node.argument) &&
      ts.isStringLiteralLike(node.argument.literal)
    )
      value = node.argument.literal;
    if (value) refs.push({ value: value.text, offset: value.getStart(tree) });
    ts.forEachChild(node, visit);
  };
  visit(tree);
  return refs;
}

export function ownershipFindings({
  root,
  files,
  read = (file) => readFileSync(resolve(root, file), "utf8"),
  realpath = (file) => realpathSync(resolve(root, file)),
}) {
  const findings = [];
  const add = (file, message, source = "", offset = 0) =>
    findings.push({
      code: "UI-OWNERSHIP",
      rule: "JST-GOV-008",
      file,
      line: source.slice(0, offset).split("\n").length,
      message,
    });
  for (const file of files) {
    if (/^(?:node_modules|target|dist|spec\/book|\.git|\.jj)\//u.test(file))
      continue;
    if (!codeFile.test(file) && !configuration.test(file)) continue;
    const source = read(file);
    for (const ref of moduleReferences(source, file))
      if (forbidden.test(ref.value.replaceAll("\\", "/")))
        add(
          file,
          `Community module reference is prohibited: ${ref.value}`,
          source,
          ref.offset,
        );
    if (configuration.test(file)) {
      const match = forbidden.exec(source.replaceAll("\\", "/"));
      if (match)
        add(
          file,
          "Community package or checkout resolution is prohibited",
          source,
          match.index,
        );
    }
    if (file.startsWith("web/src/")) {
      const target = relative(root, realpath(file));
      if (target.startsWith("..") || isAbsolute(target))
        add(file, "Production source must resolve inside this repository");
    }
  }
  const manifestFile = "docs/ui-provenance.json";
  let manifest;
  try {
    manifest = JSON.parse(read(manifestFile));
  } catch {
    add(manifestFile, "A valid local UI provenance manifest is required");
    return findings;
  }
  if (
    manifest?.version !== 1 ||
    !Array.isArray(manifest.copies) ||
    !manifest.copies.length
  ) {
    add(manifestFile, "UI provenance requires version 1 and nonempty copies");
    return findings;
  }
  const seen = new Set();
  for (const copy of manifest.copies) {
    if (
      typeof copy.local !== "string" ||
      !copy.local.startsWith("web/src/lib/") ||
      copy.local.split("/").some((part) => part === "..") ||
      seen.has(copy.local) ||
      !files.includes(copy.local)
    )
      add(manifestFile, `Invalid or missing local copy: ${copy.local}`);
    seen.add(copy.local);
    if (
      !Array.isArray(copy.sources) ||
      !copy.sources.length ||
      copy.sources.some(
        (s) =>
          typeof s.path !== "string" ||
          !s.path.startsWith("src/") ||
          !/^[0-9a-f]{40,64}$/u.test(s.revision ?? "") ||
          s.license !== "Apache-2.0",
      )
    )
      add(manifestFile, `Incomplete source/license provenance: ${copy.local}`);
  }
  const owned = new Set();
  for (const entry of manifest.owned ?? []) {
    if (
      typeof entry.local !== "string" ||
      !files.includes(entry.local) ||
      !entry.local.startsWith("web/src/lib/") ||
      seen.has(entry.local) ||
      owned.has(entry.local) ||
      typeof entry.reason !== "string" ||
      entry.reason.trim().length < 20
    )
      add(manifestFile, `Invalid native ownership declaration: ${entry.local}`);
    owned.add(entry.local);
  }
  for (const file of files.filter(
    (file) => file.startsWith("web/src/lib/") && codeFile.test(file),
  )) {
    const source = read(file);
    if (
      (/web\/src\/lib\/(?:community\/|ui\/community-)/u.test(file) ||
        /@community-copy\b/u.test(source)) &&
      !seen.has(file) &&
      !owned.has(file)
    )
      add(
        file,
        "Community-derived presentation requires a local provenance entry",
      );
  }
  return findings;
}

export function validate(context) {
  return ownershipFindings({
    root: context.model.repoRoot,
    files: context.trackedFiles,
  });
}

export default { name, validate };
