import { archivedSource } from "./archive-source.mjs";
import { readFileSync, writeFileSync, mkdirSync, readdirSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { createHash } from "node:crypto";
import { evidencePages, mergeEvidencePages } from "./evidence-format.mjs";
import {
  compactTables,
  splitContent,
  slug,
  rewriteLinks,
} from "./content-split.mjs";
const root = "spec/src";
const read = (p) => readFileSync(p, "utf8");
const write = (p, s) => {
  mkdirSync(dirname(p), { recursive: true });
  writeFileSync(p, s);
};
const original = JSON.parse(read("spec/archive/sources.json"));
const sources = Object.fromEntries(
  Object.keys(original).map((p) => [p, read(p)]),
);
const ledger = sources[`${root}/milestones.md`];
if (!ledger.includes("```json"))
  throw new Error(
    "the one-time migration is already complete; use the authoring/history commands",
  );
const entries = JSON.parse(/```json\s*\n([\s\S]*?)\n```/.exec(ledger)[1]);
const baselineEntries = JSON.parse(
  /```json\s*\n([\s\S]*?)\n```/.exec(
    archivedSource(original[`${root}/milestones.md`], read),
  )[1],
);
write(
  "spec/archive/evidence-order.json",
  JSON.stringify(
    baselineEntries.map((e) => e.id),
    null,
    2,
  ) + "\n",
);
const relocation = {};
const pending = [];
for (const [path, source] of Object.entries(sources)) {
  const stem = path.slice(path.lastIndexOf("/") + 1, -3);
  if (["SUMMARY", "index", "milestones", "verification"].includes(stem))
    continue;
  const base =
    stem === "spec-governance"
      ? `${root}/current/governance`
      : `${root}/current/contracts/${stem}`;
  const chapters = splitContent(compactTables(source), stem);
  const anchors = {};
  const children = [];
  chapters.forEach((chapter, i) => {
    const dest = `${base}/${String(i + 1).padStart(2, "0")}-${slug(chapter.title)}.md`;
    for (const match of chapter.source.matchAll(/^#{1,6} (.+)$/gm))
      anchors[slug(match[1])] = dest;
    children.push([chapter.title, dest]);
    pending.push([dest, chapter.source, path]);
  });
  relocation[path] = { target: `${base}/index.md`, anchors };
  write(
    `${base}/index.md`,
    `# ${source.match(/^# (.+)$/m)?.[1] ?? stem}\n\nRead the owning topic and requirement before implementation.\n\n${children.map(([title, p]) => `- [${title}](${relative(base, p)})`).join("\n")}\n`,
  );
}
// Preserve each historical task row; evidence histories will be generated from records.
const taskLines = ledger
  .split("\n")
  .filter((l) => /^\| (?:Bootstrap|M\d+)/.test(l));
const families = new Map();
for (const line of taskLines) {
  const family = line.split("|")[1].trim().split(".")[0];
  if (!families.has(family)) families.set(family, []);
  families.get(family).push(compactTables(line));
}
const header =
  "| ID | Status | Depends | Requirements | Gate | Next | Evidence |\n| --- | --- | --- | --- | --- | --- | --- |\n";
for (const [family, rows] of families) {
  const base = `${root}/current/progress/${family}`;
  let pages = [];
  let current = `# ${family} tasks and gates\n\n${header}`;
  for (const row of rows) {
    if (Buffer.byteLength(current + row) > 26 * 1024) {
      pages.push(current);
      current = `# ${family} tasks and gates (continued)\n\n${header}`;
    }
    current += row + "\n";
  }
  pages.push(current);
  pages.forEach((source, i) =>
    write(`${base}/${i === 0 ? "index" : `tasks-${i + 1}`}.md`, source),
  );
}
write(
  `${root}/current/progress/index.md`,
  "# Implementation progress\n\nTask pages are the sole milestone-status authority. Done requires original-gate evidence; generated histories do not promote tasks.\n\nRead the selected milestone and its generated history. Every protected slice adds a Markdown record and an explicit milestone review.\n",
);
const mappingRows = ledger
  .split("\n")
  .filter((l) => /^\|\s*E-[A-Z0-9-]+\*/.test(l));
write(
  `${root}/current/progress/task-mapping.md`,
  "# Historical task mappings\n\nOriginal evidence IDs and task fields remain unchanged. These mappings apply only to task lookup.\n\n| Historical evidence family | Original task | Current task |\n| --- | --- | --- |\n" +
    mappingRows.map(compactTables).join("\n") +
    "\n",
);
relocation[`${root}/milestones.md`] = {
  target: `${root}/current/progress/index.md`,
  anchors: {},
};
// Global verification coverage lives in small matrices; old qualification prose becomes history.
const verification = compactTables(sources[`${root}/verification.md`]);
const matrices = new Map();
for (const line of verification.split("\n")) {
  const match = /^\| (JST-([A-Z]+)-\d{3}) \|/.exec(line);
  if (match) {
    if (!matrices.has(match[2])) matrices.set(match[2], []);
    matrices.get(match[2]).push(line);
  }
}
matrices
  .get("GOV")
  .push(
    "| JST-GOV-010 | Implemented | [Migration evidence](../../history/2026/10/06/E-GOV-001/index.md) |",
    "| JST-GOV-011 | Implemented | [Migration evidence](../../history/2026/10/06/E-GOV-001/index.md) |",
  );
for (const [area, rows] of matrices)
  write(
    `${root}/current/verification/${area.toLowerCase()}.md`,
    `# ${area} verification\n\n| ID | Status | Evidence |\n| --- | --- | --- |\n` +
      rows.join("\n") +
      "\n",
  );
write(
  `${root}/current/verification/index.md`,
  "# Requirement verification\n\nEach requirement appears in exactly one matrix row across this directory. These rows record requirement coverage; milestone task status remains in Progress.\n",
);
relocation[`${root}/verification.md`] = {
  target: `${root}/current/verification/index.md`,
  anchors: {},
};
const historicalNotes = verification
  .split("\n")
  .filter((l) => !/^\| (?:JST-|ID |[- ]+\|)/.test(l))
  .join("\n");
for (const [i, chapter] of splitContent(
  historicalNotes,
  "Verification delivery notes",
).entries())
  pending.push([
    `${root}/history/notes/verification/${String(i + 1).padStart(2, "0")}-${slug(chapter.title)}.md`,
    chapter.source,
    `${root}/verification.md`,
  ]);
// Explicit resolution annotations are separate from immutable evidence payloads.
write(
  "spec/history-relations.yml",
  "E-READ-096:\n  resolves: [E-READ-094, E-READ-095]\n  reason: Main integration completed the previously recorded landing blockers.\nE-READ-094:\n  supersedes: [E-READ-091]\n  reason: Accepted cooperative cancellation replaces the proposed hard comparison deadline; the original failed probe remains a failure.\n",
);
for (const entry of entries) {
  const directory = `${root}/history/${entry.date.replaceAll("-", "/")}/${entry.id}`;
  const pages = evidencePages(entry);
  if (JSON.stringify(mergeEvidencePages(pages)) !== JSON.stringify(entry)) {
    // Property order is not evidence data; canonical comparison below is authoritative.
    const stable = (v) =>
      Array.isArray(v)
        ? v.map(stable)
        : v && typeof v === "object"
          ? Object.fromEntries(
              Object.keys(v)
                .sort()
                .map((k) => [k, stable(v[k])]),
            )
          : v;
    if (
      JSON.stringify(stable(mergeEvidencePages(pages))) !==
      JSON.stringify(stable(entry))
    )
      throw new Error(`lossy conversion ${entry.id}`);
  }
  pages.forEach((page, i) =>
    write(
      `${directory}/${i === 0 ? "index" : `details-${String(i + 1).padStart(2, "0")}`}.md`,
      page,
    ),
  );
}
// Locate the one shared topology contract and replace duplicate prose with references.
const topology =
  "JST-LOCAL-005 and JST-UI-020 separate bounded stack inventory from saved";
const topologyOwner = pending.find(
  ([path, source, original]) =>
    original.endsWith("/local-jj.md") && source.includes(topology),
)?.[0];
for (const item of pending) {
  if (
    topologyOwner &&
    /\/(?:api|ui)\.md$/.test(item[2]) &&
    item[1].includes(topology)
  )
    item[1] = item[1].replace(
      /JST-LOCAL-005 and JST-UI-020 separate bounded stack inventory from saved[\s\S]*?No\nmembership migration is implied\./,
      `The [native stack discovery contract](${relative(dirname(item[0]), topologyOwner)}) owns inventory, boundaries and membership semantics.`,
    );
  item[1] = rewriteLinks(item[1], item[0], item[2], relocation);
  write(item[0], item[1]);
}
// Resolve old matrix links at their new location.
for (const area of matrices.keys()) {
  const p = `${root}/current/verification/${area.toLowerCase()}.md`;
  write(p, rewriteLinks(read(p), p, `${root}/verification.md`, relocation));
}
for (const [old, mapping] of Object.entries(relocation))
  write(
    old,
    `# Previous chapter location\n\nThis chapter moved to [${mapping.target.split("/").slice(-3, -1).join(" / ")}](${relative(dirname(old), mapping.target)}). Follow that index for current contracts and historical evidence.\n` +
      Object.entries(mapping.anchors)
        .map(
          ([anchor, target]) =>
            `\n<a id="${anchor}"></a>\n\n[${anchor}](${relative(dirname(old), target)}#${anchor})\n`,
        )
        .join(""),
  );
write(
  `${root}/index.md`,
  "# Native JJ Store specification\n\nRead [current contracts and guidance](current/index.md), the owning topic, [progress](current/progress/index.md) and [verification](current/verification/index.md) before implementing a slice.\n\n[Historical evidence](history/index.md) preserves original outcomes. Resolution annotations explain subsequent changes; original failures and incomplete gates remain recorded.\n",
);
write(
  `${root}/current/index.md`,
  "# Current specification\n\nStart with the owning contract and requirement, then its verification and milestone task. Governance explains the evidence format, limits and protected-change workflow.\n",
);
write(
  `${root}/current/contracts/index.md`,
  "# Contract areas\n\nEach area owns its requirements and related behavior. Topic indexes link to bounded pages.\n",
);
write(
  `${root}/history/index.md`,
  "# Historical evidence\n\nRecords preserve exact commands, results and original task identities. Task mappings and resolution annotations provide current context without changing historical outcomes. Original source bytes are archived outside the active spec and search tree.\n",
);
write("spec/relocations.json", JSON.stringify(relocation, null, 2) + "\n");
// Generate bounded fragments and readable indexes, never one enormous authored SUMMARY.
function navigation(dir) {
  const items = readdirSync(dir, { withFileTypes: true }).filter(
    (e) => !e.name.startsWith("SUMMARY"),
  );
  const links = [];
  for (const e of items.filter(
    (e) => e.isFile() && e.name.endsWith(".md") && e.name !== "index.md",
  )) {
    const p = join(dir, e.name);
    const title = read(p).match(/^# (.+)$/m)?.[1] ?? e.name;
    links.push(`- [${title}](${e.name})`);
  }
  for (const e of items
    .filter((e) => e.isDirectory())
    .sort((a, b) =>
      a.name.localeCompare(b.name, undefined, { numeric: true }),
    )) {
    const child = join(dir, e.name);
    if (!readdirSync(child).includes("index.md"))
      write(
        join(child, "index.md"),
        `# ${e.name}\n\nBrowse the child pages below.\n`,
      );
    navigation(child);
    const title =
      read(join(child, "index.md")).match(/^# (.+)$/m)?.[1] ?? e.name;
    links.push(
      `- [${title}](${e.name}/index.md)`,
      `  {{#include ${e.name}/SUMMARY.nav.md}}`,
    );
  }
  write(join(dir, "SUMMARY.nav.md"), links.join("\n") + "\n");
  const index = join(dir, "index.md");
  if (dir !== root)
    write(
      index,
      read(index) +
        "\n## Pages\n\n" +
        links.filter((l) => !l.includes("{{#include")).join("\n") +
        "\n",
    );
}
navigation(root);
write(
  `${root}/SUMMARY.md`,
  "# Summary\n\n- [Specification](index.md)\n{{#include SUMMARY.nav.md}}\n",
);
console.log(
  `Migrated ${entries.length} evidence records, ${taskLines.length} task rows and ${matrices.size} verification matrices.`,
);
