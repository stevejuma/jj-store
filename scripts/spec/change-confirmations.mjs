import { revisionReader } from "./revision-reader.mjs";
import { loadLedger } from "./progress-loader.mjs";
import { integrityErrors } from "./progress-integrity.mjs";
import { signature, validateLedger } from "./progress-validation.mjs";
import { archivedSource } from "./archive-source.mjs";
export function confirmationIsQualified(entry) {
  return (
    entry.result === "pass" &&
    Array.isArray(entry.remaining) &&
    entry.remaining.length === 0 &&
    entry.runs?.some((run) => run.result === "pass" && run.failures === 0)
  );
}
export function changeConfirmations(root, selection) {
  if (selection.files?.length) return [];
  const current = revisionReader(root, selection.head ?? "@", {
    base: selection.base,
  });
  const ledger = loadLedger(current);
  const parentReaders = (
    selection.base ? [selection.base] : current.parents
  ).map((rev) => revisionReader(root, rev));
  const old = parentReaders.flatMap((reader) => loadLedger(reader).entries);
  const definitions = new Map();
  for (const file of current.files.filter((p) =>
    /^spec\/src\/current\/(?:contracts|governance)\/.+\.md$/.test(p),
  ))
    for (const match of current
      .read(file)
      .matchAll(/^\| (JST-[A-Z]+-\d{3}) \|/gm))
      definitions.set(match[1], file);
  const errors = integrityErrors(
    ledger,
    current,
    old,
    new Set(definitions.keys()),
  );
  const archive = JSON.parse(current.read("spec/archive/sources.json"));
  const read = (path) => {
    try {
      return current.read(path);
    } catch (error) {
      if (archive[path]) return archivedSource(archive[path], current.read);
      throw error;
    }
  };
  errors.push(
    ...validateLedger({
      source: ledger.source,
      parsed: ledger,
      read,
      requirements: new Set(definitions.keys()),
      changed: current.changed,
      previousEntries: old,
    }),
  );
  if (errors.length) throw new Error(errors.join("\n"));
  const inherited = new Set(old.map(signature));
  for (const entry of ledger.entries)
    if (
      !inherited.has(signature(entry)) &&
      entry.contract_review?.status === "unchanged" &&
      !confirmationIsQualified(entry)
    )
      throw new Error(
        `${entry.id}: unchanged-contract confirmation requires passing completed validation`,
      );
  const result = ledger.entries
    .filter(
      (e) =>
        !inherited.has(signature(e)) &&
        e.contract_review?.status === "unchanged",
    )
    .map((e) => ({
      paths: e.files,
      chapters: (e.requirements ?? []).map((id) => definitions.get(id)),
      evidence: ledger.evidencePaths.get(e.id),
      reason: e.contract_review.reason,
    }));
  current.verify();
  for (const reader of parentReaders) reader.verify();
  return result;
}
