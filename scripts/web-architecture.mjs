import { existsSync, readFileSync, readdirSync } from "node:fs";
import { extname, join, relative, resolve } from "node:path";

export const limits = {
  production: 300,
  story: 400,
  test: 500,
  script: 300,
};

const ignored = new Set([
  ".git",
  ".jj",
  ".svelte-kit",
  "dist",
  "node_modules",
  "spec",
  "storybook-static",
  "target",
]);

export function countedLines(source) {
  let blockComment = false;
  let htmlComment = false;
  let count = 0;
  for (const line of source.split("\n")) {
    const value = line.trim();
    if (blockComment) {
      if (value.includes("*/")) blockComment = false;
      continue;
    }
    if (htmlComment) {
      if (value.includes("-->")) htmlComment = false;
      continue;
    }
    if (!value || value.startsWith("//")) continue;
    if (value.startsWith("/*")) {
      blockComment = !value.includes("*/");
      continue;
    }
    if (value.startsWith("<!--")) {
      htmlComment = !value.includes("-->");
      continue;
    }
    if (
      value.startsWith("*") &&
      !/[{};,]/u.test(value) &&
      !/^\*(?:[.#:\[]|\/)/u.test(value)
    )
      continue;
    count += 1;
  }
  return count;
}

export function fileKind(path) {
  if (/\.stories\.svelte$/u.test(path)) return "story";
  if (/\.(?:test|spec)\.(?:mjs|ts)$/u.test(path)) return "test";
  if (path.startsWith("scripts/")) return "script";
  return "production";
}

export function sourceFiles(root) {
  const found = [];
  const visit = (directory) => {
    for (const entry of readdirSync(directory, { withFileTypes: true })) {
      if (ignored.has(entry.name)) continue;
      const absolute = join(directory, entry.name);
      if (entry.isDirectory()) visit(absolute);
      else if ([".css", ".mjs", ".svelte", ".ts"].includes(extname(entry.name)))
        found.push(relative(root, absolute).replaceAll("\\", "/"));
    }
  };
  for (const directory of [
    "catalog",
    "scripts",
    "web/src",
    ...(existsSync(join(root, "packages")) ? ["packages"] : []),
  ])
    visit(join(root, directory));
  return found.filter(
    (path) =>
      ![
        "web/src/lib/api/generated.ts",
        "packages/jj-history/src/generated.ts",
      ].includes(path),
  );
}

export function architectureErrors(root, baseline) {
  const errors = [];
  const files = sourceFiles(root);
  const current = new Set(files);
  for (const path of Object.keys(baseline.files))
    if (!current.has(path)) errors.push(`${path}: stale architecture baseline`);
  for (const path of files) {
    const source = readFileSync(join(root, path), "utf8");
    const count = countedLines(source);
    const kind = fileKind(path);
    const allowed = baseline.files[path] ?? limits[kind];
    if (count > allowed)
      errors.push(`${path}: ${count} counted lines exceeds ${allowed}`);
    if (
      path.startsWith("web/src/") &&
      /(?:from\s*|import\s*(?:\(|))["'][^"']*catalog\//u.test(source)
    )
      errors.push(`${path}: production web source imports catalog code`);
    if (
      path.startsWith("web/src/lib/presentation/") &&
      /(?:from\s*|import\s*(?:\(|))["'][^"']*(?:api|controller)\//u.test(source)
    )
      errors.push(`${path}: presentation source imports API/controller code`);
    if (
      path.startsWith("web/src/") &&
      !/\.(?:test|spec)\.(?:mjs|ts)$/u.test(path) &&
      /(?:from\s*|import\s*(?:\(|))["'][^"']*(?:fixtures|\.stories|\.test|\.spec)[^"']*["']/u.test(
        source,
      )
    )
      errors.push(`${path}: production source imports test or fixture code`);
  }
  return errors;
}

export function run(root = process.cwd()) {
  const resolved = resolve(root);
  const baseline = JSON.parse(
    readFileSync(
      join(resolved, "scripts/web-architecture-baseline.json"),
      "utf8",
    ),
  );
  const errors = architectureErrors(resolved, baseline);
  if (errors.length) throw new Error(errors.join("\n"));
  console.log(`web architecture ok (${sourceFiles(resolved).length} modules)`);
}

if (
  process.argv[1] &&
  resolve(process.argv[1]) === resolve(new URL(import.meta.url).pathname)
)
  run();
