import { revisionReader } from "./revision-reader.mjs";
import { archivedSource } from "./archive-source.mjs";
import { readFileSync, readdirSync } from "node:fs";
import { createHash } from "node:crypto";
import { join, resolve } from "node:path";
import { loadLedger } from "./progress-loader.mjs";
import { parseLedger, signature } from "./progress-validation.mjs";
export function sourceFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory()
      ? sourceFiles(join(directory, e.name))
      : [join(directory, e.name)],
  );
}
export function migrationAudit(root = ".", revision) {
  const selected = revision
    ? revisionReader(resolve(root), revision)
    : undefined;
  const read = selected?.read ?? ((p) => readFileSync(join(root, p), "utf8"));
  const files = selected
    ? selected.files.filter((path) => path.startsWith("spec/src/"))
    : sourceFiles(join(root, "spec/src")).map((p) =>
        root === "." ? p : p.slice(root.length + 1),
      );
  const sources = JSON.parse(read("spec/archive/sources.json"));
  const baseline = parseLedger(
    archivedSource(sources["spec/src/milestones.md"], read),
  );
  const ledger = loadLedger({
    read,
    files: [...files, "spec/archive/evidence-order.json"],
  });
  const errors = [];
  const entries = new Map(ledger.entries.map((e) => [e.id, e]));
  for (const original of baseline.entries)
    if (signature(entries.get(original.id)) !== signature(original))
      errors.push(`lost/changed evidence ${original.id}`);
  if (
    ledger.entries
      .filter((e) => baseline.entries.some((b) => b.id === e.id))
      .map((e) => e.id)
      .join("\n") !== baseline.entries.map((e) => e.id).join("\n")
  )
    errors.push("historical evidence order changed");
  const tasks = new Map(ledger.tasks.map((t) => [t.id, t]));
  for (const task of baseline.tasks)
    if (signature(tasks.get(task.id)) !== signature(task))
      errors.push(`original task state/gate changed: ${task.id}`);
  const ids = (source) =>
    [...source.matchAll(/^\| (JST-[A-Z]+-\d{3}) \|/gm)].map((m) => m[1]);
  const originalIds = new Set(
    Object.entries(sources)
      .filter(([p]) => !/(milestones|verification|SUMMARY)\.md$/.test(p))
      .flatMap(([, s]) => ids(archivedSource(s, read))),
  );
  const currentIds = files
    .filter((p) =>
      /^spec\/src\/current\/(?:contracts|governance)\/.+\.md$/.test(p),
    )
    .flatMap((p) => ids(read(p)));
  for (const id of originalIds)
    if (currentIds.filter((v) => v === id).length !== 1)
      errors.push(`requirement preservation/uniqueness: ${id}`);
  const statements = (source) =>
    new Map(
      source
        .split("\n")
        .filter((line) => /^\| JST-/.test(line))
        .map((line) => {
          const cells = line
            .trim()
            .slice(1, -1)
            .split(/(?<!\\)\|/)
            .map((v) => v.trim());
          return [cells[0], cells[1]];
        }),
    );
  const priorStatements = new Map(
    Object.entries(sources)
      .filter(([path]) => !/(milestones|verification|SUMMARY)\.md$/.test(path))
      .flatMap(([, source]) => [...statements(archivedSource(source, read))]),
  );
  const currentStatements = new Map(
    files
      .filter((path) =>
        /^spec\/src\/current\/(?:contracts|governance)\/.+\.md$/.test(path),
      )
      .flatMap((path) => [...statements(read(path))]),
  );
  for (const [id, statement] of priorStatements)
    if (
      !["JST-GOV-002", "JST-GOV-003"].includes(id) &&
      currentStatements.get(id) !== statement
    )
      errors.push(`original requirement statement changed: ${id}`);
  for (const source of Object.values(sources))
    if (
      createHash("sha256")
        .update(archivedSource(source, read))
        .digest("hex") !== source.sha256
    )
      errors.push(`archive bytes changed: ${source.archive}`);
  for (const path of files.filter((p) => p.endsWith(".md"))) {
    const source = read(path);
    if (
      source.split("\n").length - (source.endsWith("\n") ? 1 : 0) > 400 ||
      Buffer.byteLength(source) > 32768
    )
      errors.push(`oversized authored page: ${path}`);
  }
  selected?.verify();
  if (errors.length) throw new Error(errors.join("\n"));
  return {
    evidence: baseline.entries.length,
    tasks: baseline.tasks.length,
    requirements: originalIds.size,
    pages: files.filter((p) => p.endsWith(".md")).length,
  };
}
if (process.argv[1]?.endsWith("/migration-audit.mjs"))
  console.log("migration audit ok", migrationAudit());
