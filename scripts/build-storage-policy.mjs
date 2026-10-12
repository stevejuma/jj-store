import { statfs } from "node:fs/promises";
import {
  helper,
  locked,
  measure,
  snapshot,
  canonical,
} from "./build-storage-inventory.mjs";

const GiB = 1024 ** 3;
export function limits(environment = process.env) {
  const value = (name, fallback) => {
    const number = Number(environment[name] ?? fallback);
    if (!Number.isFinite(number) || number <= 0) throw Error(`Invalid ${name}`);
    return number * GiB;
  };
  return {
    target: value("JJ_STORE_BUILD_TARGET_GIB", 40),
    total: value("JJ_STORE_BUILD_TOTAL_GIB", 80),
    free: value("JJ_STORE_BUILD_FREE_GIB", 30),
  };
}

export async function freeBytes(root) {
  const info = await statfs(root);
  return info.bavail * info.bsize;
}

export function pressure(entries, free, budget) {
  return (
    entries.some((e) => e.bytes > budget.target) ||
    entries.reduce((n, e) => n + e.bytes, 0) > budget.total ||
    free < budget.free
  );
}

export function cleanupOrder(entries, targetLimit = Infinity) {
  return entries
    .filter(
      (e) => (e.artifactBytes ?? e.bytes) > 0 && (!e.temporary || e.completed),
    )
    .sort(
      (a, b) =>
        Number(b.temporary) - Number(a.temporary) ||
        Number(b.bytes > targetLimit) - Number(a.bytes > targetLimit) ||
        a.modified - b.modified ||
        a.path.localeCompare(b.path),
    );
}

export async function active(context, entry) {
  if (!entry.bytes) return [];
  if (entry.unsafe.length)
    return [`unsafe entries: ${entry.unsafe.slice(0, 3).join(", ")}`];
  try {
    const { stdout } = await locked(
      context,
      entry.path,
      "exclusive",
      "python3",
      [helper, "active", entry.path],
    );
    return JSON.parse(stdout).map((pid) => `PID ${pid}`);
  } catch (error) {
    return [(error.stderr || error.message).trim()];
  }
}

export async function cleanTarget(context, entry, kind) {
  await canonical(entry.path);
  const result = await locked(
    context,
    entry.path,
    "exclusive",
    "python3",
    [helper, "clean", entry.path, entry.manifest, kind],
    { maxBuffer: 4 * 1024 * 1024 },
  );
  return JSON.parse(result.stdout);
}

export async function maintain(
  context,
  {
    dryRun = false,
    environment = process.env,
    log = console.error,
    preserve = [],
  } = {},
) {
  const budget = limits(environment);
  const entries = await snapshot(context);
  const beforeFree = await freeBytes(context.root);
  let free = beforeFree;
  const actions = [],
    skipped = [];
  const unavoidable = [];
  for (const entry of entries.filter((e) => e.bytes > budget.target)) {
    const reasons =
      entry.temporary && !entry.completed
        ? ["retained temporary target"]
        : preserve.includes(entry.path)
          ? ["just-built outputs retained for the caller"]
          : await active(context, entry);
    if (reasons.length) {
      unavoidable.push(entry.path);
      skipped.push({ path: entry.path, reasons });
      log(`Build storage: skipped ${entry.path}: ${reasons.join("; ")}`);
    }
  }
  for (const entry of cleanupOrder(entries, budget.target)) {
    if (!pressure(entries, free, budget)) break;
    if (!entry.temporary && unavoidable.length && free >= budget.free) break;
    if (unavoidable.includes(entry.path)) continue;
    const busy = preserve.includes(entry.path)
      ? ["just-built outputs retained for the caller"]
      : await active(context, entry);
    if (busy.length) {
      skipped.push({ path: entry.path, reasons: busy });
      log(`Build storage: skipped ${entry.path}: ${busy.join("; ")}`);
      continue;
    }
    for (const kind of entry.temporary ? ["all"] : ["workspace", "all"]) {
      if (!pressure(entries, free, budget)) break;
      actions.push({
        path: entry.path,
        kind,
        reason: entry.temporary
          ? "completed temporary target"
          : "build storage pressure",
      });
      log(
        `Build storage: ${dryRun ? "would clean" : "cleaning"} ${kind} artifacts in ${entry.path}`,
      );
      if (dryRun) break;
      try {
        await cleanTarget(context, entry, kind);
        Object.assign(entry, await measure(entry));
        free = await freeBytes(context.root);
      } catch (error) {
        skipped.push({
          path: entry.path,
          reasons: [(error.stderr || error.message).trim()],
        });
        break;
      }
    }
  }
  return {
    entries,
    actions,
    skipped,
    beforeFree,
    afterFree: free,
    budget,
    blocked: pressure(entries, free, budget),
    dryRun,
  };
}

export function storageFailure(result) {
  const total = result.entries.reduce((n, e) => n + e.bytes, 0);
  const oversized = result.entries
    .filter((e) => e.bytes > result.budget.target)
    .map((e) => `${e.path}: ${(e.bytes / GiB).toFixed(1)} GiB`)
    .join("; ");
  return `Build storage limits exceeded: ${(total / GiB).toFixed(1)} GiB managed (limit ${(result.budget.total / GiB).toFixed(1)}); ${oversized || "no oversized individual target"} (target limit ${(result.budget.target / GiB).toFixed(1)}); free ${(result.afterFree / GiB).toFixed(1)} GiB (minimum ${(result.budget.free / GiB).toFixed(1)}). Run pnpm dev:disk and pnpm dev:clean -- --apply after targets are idle. Stop only your owned demo with pnpm dev:stop if its build target needs cleanup.`;
}
