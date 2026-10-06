import { archivedSource } from "./archive-source.mjs";
import { createHash } from "node:crypto";
import { historicalTaskMapping } from "../progress-mapping.mjs";
import {
  needsEvidence,
  parseLedger,
  signature,
} from "./progress-validation.mjs";
import { relationsFor } from "./progress-loader.mjs";
export function integrityErrors(
  ledger,
  current,
  previousEntries,
  requirements,
) {
  const errors = [];
  let archived;
  try {
    const sources = JSON.parse(current.read("spec/archive/sources.json"));
    for (const source of Object.values(sources))
      if (
        createHash("sha256")
          .update(archivedSource(source, current.read))
          .digest("hex") !== source.sha256
      )
        errors.push(`immutable archive changed: ${source.archive}`);
    archived = parseLedger(
      archivedSource(sources["spec/src/milestones.md"], current.read),
    ).entries;
  } catch (error) {
    errors.push(`migration archive unavailable: ${error.message}`);
    return errors;
  }
  const entries = new Map(ledger.entries.map((e) => [e.id, e]));
  for (const entry of archived)
    if (
      !entries.has(entry.id) ||
      signature(entries.get(entry.id)) !== signature(entry)
    )
      errors.push(
        `${entry.id}: historical evidence payload changed or disappeared`,
      );
  const archivedIds = new Set(archived.map((e) => e.id));
  const old = new Map(previousEntries.map((e) => [e.id, e]));
  for (const entry of ledger.entries) {
    if (
      previousEntries.some(
        (previous) =>
          previous.id === entry.id && signature(previous) !== signature(entry),
      )
    )
      errors.push(
        `${entry.id}: committed evidence is immutable; append a correction record`,
      );
    if (archivedIds.has(entry.id)) continue;
    if (!Array.isArray(entry.requirements) || !entry.requirements.length)
      errors.push(`${entry.id}: name affected requirements`);
    for (const id of entry.requirements ?? [])
      if (!requirements.has(id))
        errors.push(`${entry.id}: unknown requirement ${id}`);
    if (
      !["changed", "unchanged"].includes(entry.contract_review?.status) ||
      !entry.contract_review?.reason?.trim()
    )
      errors.push(`${entry.id}: missing justified contract review`);
    if (
      !["updated", "unchanged"].includes(entry.milestone_review?.status) ||
      !entry.milestone_review?.reason?.trim()
    )
      errors.push(`${entry.id}: missing explicit milestone review`);
  }
  const fresh = ledger.entries.filter((e) => !old.has(e.id));
  for (const path of current.changed.filter(needsEvidence))
    if (!fresh.some((e) => e.files?.includes(path)))
      errors.push(`${path}: missing fresh evidence coverage`);
  const relations = relationsFor(current.read);
  for (const [id, relation] of Object.entries(relations)) {
    if (!entries.has(id) || !relation.reason?.trim())
      errors.push(`${id}: invalid evidence relation`);
    for (const target of [
      ...(relation.resolves ?? []),
      ...(relation.supersedes ?? []),
    ])
      if (!entries.has(target) || target === id)
        errors.push(`${id}: invalid related evidence ${target}`);
  }
  const visiting = new Set(),
    done = new Set();
  function visit(id) {
    if (visiting.has(id)) {
      errors.push(`evidence relation cycle at ${id}`);
      return;
    }
    if (done.has(id)) return;
    visiting.add(id);
    for (const target of [
      ...(relations[id]?.resolves ?? []),
      ...(relations[id]?.supersedes ?? []),
    ])
      visit(target);
    visiting.delete(id);
    done.add(id);
  }
  for (const id of Object.keys(relations)) visit(id);
  return errors;
}
export function taskHistory(ledger, task) {
  const errors = [];
  const mapped = historicalTaskMapping(
    ledger.source,
    new Map(ledger.tasks.map((t) => [t.id, t])),
    errors,
  );
  if (errors.length) throw new Error(errors.join("\n"));
  return ledger.entries.filter((entry) => mapped(entry).includes(task));
}
