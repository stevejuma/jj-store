import {
  existsSync,
  mkdirSync,
  writeFileSync,
  readFileSync,
  readdirSync,
  rmSync,
} from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { evidencePages } from "./evidence-format.mjs";
import { updateNavigation } from "./navigation-update.mjs";
export function writeEvidence(entry, root = ".") {
  if (
    !/^E-[A-Z0-9-]+$/.test(entry.id) ||
    !/^\d{4}-\d{2}-\d{2}$/.test(entry.date)
  )
    throw new Error("invalid evidence ID or date");
  const directory = join(
    root,
    "spec/src/history",
    entry.date.replaceAll("-", "/"),
    entry.id,
  );
  mkdirSync(directory, { recursive: true });
  const pages = evidencePages(entry);
  for (const name of readdirSync(directory))
    if (
      /^details-\d+\.md$/.test(name) &&
      Number(name.match(/\d+/)[0]) > pages.length
    )
      rmSync(join(directory, name));
  pages.forEach((page, i) =>
    writeFileSync(
      join(
        directory,
        i ? `details-${String(i + 1).padStart(2, "0")}.md` : "index.md",
      ),
      page,
    ),
  );
  return directory;
}
if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
) {
  const args = process.argv.slice(2).filter((v) => v !== "--");
  const values = {};
  for (let i = 0; i < args.length; i += 2) {
    if (!["--id", "--task", "--requirement"].includes(args[i]) || !args[i + 1])
      throw new Error("use --id E-... --task M... --requirement JST-...");
    (values[args[i]] ??= []).push(args[i + 1]);
  }
  if (
    values["--id"]?.length !== 1 ||
    !values["--task"]?.length ||
    !values["--requirement"]?.length
  )
    throw new Error("ID, task and requirement are required");
  const entry = {
    id: values["--id"][0],
    tasks: values["--task"],
    requirements: values["--requirement"],
    date: new Date().toISOString().slice(0, 10),
    kind: "verification",
    files: [],
    tests: [],
    runs: [],
    result: "pending",
    remaining: ["Complete actual validation and the milestone review."],
    versions: "Record actual versions.",
    formats: "Record affected formats.",
    scope: "Describe the concrete behavior and impact surface.",
    next: "Complete the named acceptance target.",
    contract_review: {
      status: "unchanged",
      reason:
        "Explain the unchanged contract, or select changed and update the owning topics.",
    },
    milestone_review: {
      status: "unchanged",
      reason: "Review the selected tasks and next actions.",
    },
  };
  const containsId = (directory) =>
    readdirSync(directory, { withFileTypes: true }).some(
      (item) =>
        item.isDirectory() &&
        (item.name === entry.id || containsId(join(directory, item.name))),
    );
  if (containsId("spec/src/history"))
    throw new Error("evidence ID already exists");
  const output = writeEvidence(entry);
  updateNavigation();
  console.log(
    `Created ${output}/index.md; fill the actual evidence before validation.`,
  );
}
