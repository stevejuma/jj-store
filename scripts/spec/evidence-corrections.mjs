import { signature } from "./evidence-signature.mjs";

// This projection is used for metadata validation only. Gates, histories and
// immutability audits always consume the original evidence payloads.
export function correctedEvidence(entries, errors) {
  const originals = new Map(entries.map((entry) => [entry.id, entry]));
  const projected = new Map(entries.map((entry) => [entry.id, { ...entry }]));
  const seen = new Set();
  for (const record of entries) {
    if (record.corrections === undefined) continue;
    if (!Array.isArray(record.corrections) || !record.corrections.length) {
      errors.push(`${record.id}: corrections require a nonempty list`);
      continue;
    }
    for (const correction of record.corrections) {
      const target = originals.get(correction?.target);
      const fail = (message) =>
        errors.push(`${record.id}: correction ${message}`);
      if (!target || target === record || target.corrections) {
        fail("target must be an original, distinct evidence record");
        continue;
      }
      if (!correction.reason?.trim()) fail("requires a reason");
      if (record.date < target.date) fail("predates its target");
      if (
        record.result !== "pass" ||
        record.remaining?.length ||
        !record.runs?.some((run) => run.result === "pass" && run.failures === 0)
      )
        fail("requires completed passing verification");
      if (!correction.fields || !Object.keys(correction.fields).length) {
        fail("requires explicit field changes");
        continue;
      }
      for (const [field, change] of Object.entries(correction.fields)) {
        if (!["tests", "runs", "result"].includes(field)) {
          fail(`cannot replace field ${field}`);
          continue;
        }
        const key = `${target.id}:${field}`;
        if (seen.has(key)) {
          fail(`duplicate field ${key}`);
          continue;
        }
        seen.add(key);
        if (!change || signature(change.before) !== signature(target[field])) {
          fail(`prior value mismatch for ${key}`);
          continue;
        }
        if (
          field === "result" &&
          (["pass", "pending", "blocked", "fail"].includes(target.result) ||
            !["pending", "blocked", "fail"].includes(change.after))
        ) {
          fail("cannot alter a valid result or promote historical success");
          continue;
        }
        if (
          field === "tests" &&
          (!Array.isArray(change.after) ||
            target.tests.some(
              (ref) => ref.includes("#") && !change.after.includes(ref),
            ))
        ) {
          fail("must retain all named test references");
          continue;
        }
        if (field === "runs") {
          const valid =
            Array.isArray(change.after) &&
            change.after.length === target.runs.length &&
            change.after.every((run, index) => {
              const original = target.runs[index];
              const { result, failures, ...retained } = run;
              if (["pass", "fail"].includes(original.result))
                return signature(run) === signature(original);
              return (
                signature(retained) === signature(original) &&
                result === { passed: "pass", failed: "fail" }[original.outcome]
              );
            });
          if (!valid) {
            fail("must retain each recorded run, note and original outcome");
            continue;
          }
        }
        projected.get(target.id)[field] = change.after;
      }
    }
  }
  return [...projected.values()];
}
