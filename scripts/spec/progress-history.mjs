import { readFileSync, readdirSync, writeFileSync, mkdirSync } from "node:fs";
import { join, dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { loadLedger, relationsFor } from "./progress-loader.mjs";
import { taskHistory } from "./progress-integrity.mjs";
import { updateNavigation } from "./navigation-update.mjs";
import { indexBody } from "./navigation-pages.mjs";
function files(dir) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((e) =>
    e.isDirectory() ? files(join(dir, e.name)) : [join(dir, e.name)],
  );
}
export function relationContext(ledger, relations, entry) {
  const rows = [];
  const link = (id) =>
    `[${id}](${relative(dirname(ledger.evidencePaths.get(entry.id)), ledger.evidencePaths.get(id))})`;
  for (const [id, relation] of Object.entries(relations))
    for (const verb of ["resolves", "supersedes"]) {
      for (const target of relation[verb] ?? []) {
        if (id === entry.id)
          rows.push(`- This entry ${verb} ${link(target)}. ${relation.reason}`);
        if (target === entry.id)
          rows.push(`- ${link(id)} ${verb} this entry. ${relation.reason}`);
      }
    }
  return rows.length
    ? `\n<!-- evidence-relations -->\n## Resolution and supersession\n\nThe original result remains **${entry.result}**. Milestone gates select their own evidence.\n\n${rows.join("\n")}\n<!-- /evidence-relations -->\n`
    : "";
}
export function refreshTaskHistories() {
  const read = (p) => readFileSync(p, "utf8");
  const ledger = loadLedger({
    read,
    files: [...files("spec/src"), "spec/archive/evidence-order.json"],
  });
  const relations = relationsFor(read);
  for (const entry of ledger.entries) {
    const path = ledger.evidencePaths.get(entry.id);
    const body = indexBody(read(path));
    const offset = body.indexOf(
      "\n<!-- evidence-relations -->",
      Math.max(0, body.lastIndexOf("<!-- /evidence-field -->")),
    );
    const source = offset < 0 ? body : body.slice(0, offset);
    writeFileSync(path, source + relationContext(ledger, relations, entry));
  }
  for (const task of ledger.tasks) {
    const family = task.id.split(".")[0];
    const root = `spec/src/current/progress/${family}/history/${task.id}`;
    mkdirSync(root, { recursive: true });
    const rows = taskHistory(ledger, task.id).map((e) => {
      const relation = Object.entries(relations)
        .filter(([, r]) =>
          [...(r.resolves ?? []), ...(r.supersedes ?? [])].includes(e.id),
        )
        .map(([id]) => id);
      return `| [${e.id}](${relative(root, ledger.evidencePaths.get(e.id))}) | ${e.date} | ${e.result} | ${relation.length ? `Later context: ${relation.join(", ")}` : "Original outcome"} |`;
    });
    const header = `# ${task.id} evidence history\n\nGenerated from evidence task references and historical mappings. This history is not a completion gate selection.\n\n| Evidence | Date | Original result | Later context |\n| --- | --- | --- | --- |\n`;
    let part = 0,
      current = header;
    for (const row of rows) {
      if (
        current.split("\n").length > 280 ||
        Buffer.byteLength(current + row) > 26 * 1024
      ) {
        writeFileSync(
          join(root, part ? `history-${part + 1}.md` : "index.md"),
          current,
        );
        part++;
        current = header;
      }
      current += row + "\n";
    }
    writeFileSync(
      join(root, part ? `history-${part + 1}.md` : "index.md"),
      current,
    );
  }
  updateNavigation();
  console.log(`Refreshed histories for ${ledger.tasks.length} tasks.`);
}
if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
)
  refreshTaskHistories();
