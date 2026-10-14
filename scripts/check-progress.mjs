import { archivedSource } from "./spec/archive-source.mjs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { revisionReader } from "./spec/revision-reader.mjs";
import { loadLedger } from "./spec/progress-loader.mjs";
import { integrityErrors } from "./spec/progress-integrity.mjs";
import { validateLedger } from "./spec/progress-validation.mjs";
export {
  ledgerPath,
  parseLedger,
  validateLedger,
  needsEvidence,
} from "./spec/progress-validation.mjs";
export function run(
  root,
  requested = process.env.JJ_STORE_CHECK_REVISION ?? "@",
) {
  const current = revisionReader(root, requested);
  if (!current.parents.length)
    throw new Error("checked revision has no parent");
  const ledger = loadLedger(current);
  const parentReaders = current.parents.map((parent) =>
    revisionReader(root, parent),
  );
  const previousEntries = parentReaders.flatMap(
    (parent) => loadLedger(parent).entries,
  );
  const requirements = new Set(
    current.files
      .filter(
        (p) =>
          /^spec\/src\/(?:current\/(?:contracts|governance)\/|[^/]+\.md$)/.test(
            p,
          ) && !/(?:milestones|verification|SUMMARY)\.md$/.test(p),
      )
      .flatMap((p) =>
        [...current.read(p).matchAll(/^\| (JST-[A-Z]+-\d{3}) \|/gm)].map(
          (m) => m[1],
        ),
      ),
  );
  let archive = {};
  try {
    archive = JSON.parse(current.read("spec/archive/sources.json"));
  } catch {}
  const read = (path) => {
    try {
      return current.read(path);
    } catch (error) {
      if (archive[path]) return archivedSource(archive[path], current.read);
      throw error;
    }
  };
  const errors = validateLedger({
    source: ledger.source,
    parsed: ledger,
    read,
    requirements,
    changed: current.changed,
    previousEntries,
  });
  if (ledger.evidencePaths.size)
    errors.push(
      ...integrityErrors(ledger, current, previousEntries, requirements),
    );
  current.verify();
  for (const parent of parentReaders) parent.verify();
  if (errors.length) throw new Error(errors.join("\n"));
  console.log(
    `progress ok (${ledger.tasks.length} tasks; ${ledger.entries.length} evidence records at ${current.revision.slice(0, 8)}; evidence bookkeeping only)`,
  );
}
if (
  process.argv[1] &&
  resolve(process.argv[1]) === fileURLToPath(import.meta.url)
)
  try {
    run(resolve(fileURLToPath(new URL("..", import.meta.url))));
  } catch (error) {
    console.error(error.message);
    process.exitCode = 1;
  }
