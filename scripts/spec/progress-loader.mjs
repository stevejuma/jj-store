import { parse } from "yaml";
import { mergeEvidencePages } from "./evidence-format.mjs";
import { parseLedger } from "./progress-validation.mjs";
export const progressRoot = "spec/src/current/progress/";
export const evidenceRoot = "spec/src/history/";
export function loadLedger({ read, files }) {
  if (!files.some((p) => p.startsWith(progressRoot)))
    return {
      ...parseLedger(read("spec/src/milestones.md")),
      source: read("spec/src/milestones.md"),
      evidencePaths: new Map(),
    };
  const source = files
    .filter(
      (p) =>
        p.startsWith(progressRoot) &&
        p.endsWith(".md") &&
        !p.includes("SUMMARY"),
    )
    .map(read)
    .join("\n");
  const tasks = source
    .split("\n")
    .filter((l) => /^\| (?:Bootstrap|M\d+)/.test(l))
    .map((line) => {
      const c = line
        .trim()
        .slice(1, -1)
        .split("|")
        .map((v) => v.trim());
      if (c.length !== 7) throw new Error(`invalid task row: ${line}`);
      const list = (s) => (s === "-" ? [] : s.split(",").map((v) => v.trim()));
      return {
        id: c[0],
        status: c[1],
        depends: list(c[2]),
        requirements: list(c[3]),
        gate: c[4],
        next: c[5],
        evidence: list(c[6]),
      };
    });
  const paths = files.filter(
    (p) => p.startsWith(evidenceRoot) && /\/E-[A-Z0-9-]+\/index\.md$/.test(p),
  );
  const evidencePaths = new Map();
  const entries = paths.map((path) => {
    const directory = path.slice(0, -8);
    const pages = files
      .filter(
        (p) =>
          p.startsWith(directory) && /\/(?:index|details-\d+)\.md$/.test(p),
      )
      .sort((a, b) =>
        a.endsWith("index.md")
          ? -1
          : b.endsWith("index.md")
            ? 1
            : a.localeCompare(b),
      );
    const entry = mergeEvidencePages(pages.map(read));
    if (!entry.id || !path.endsWith(`/${entry.id}/index.md`))
      throw new Error(`evidence path/identity mismatch: ${path}`);
    evidencePaths.set(entry.id, path);
    return entry;
  });
  const order = JSON.parse(read("spec/archive/evidence-order.json"));
  const rank = new Map(order.map((id, i) => [id, i]));
  entries.sort(
    (a, b) =>
      (rank.get(a.id) ?? Infinity) - (rank.get(b.id) ?? Infinity) ||
      a.date.localeCompare(b.date) ||
      a.id.localeCompare(b.id),
  );
  return { tasks, entries, source, evidencePaths };
}
export function relationsFor(read) {
  try {
    return parse(read("spec/history-relations.yml")) ?? {};
  } catch (error) {
    if (String(error).includes("missing")) return {};
    throw error;
  }
}
