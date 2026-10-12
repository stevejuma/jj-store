#!/usr/bin/env node
import { parseArgs } from "node:util";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import {
  storageContext,
  registry,
  saveRegistry,
  targets,
  snapshot,
  measure,
  canonical,
  run,
  helper,
} from "./build-storage-inventory.mjs";
import {
  freeBytes,
  limits,
  maintain,
  storageFailure,
  cleanTarget,
  active,
} from "./build-storage-policy.mjs";

const gib = (bytes) => (bytes / 1024 ** 3).toFixed(1);
export async function storageCli(
  args,
  root = resolve(import.meta.dirname, ".."),
) {
  const { values, positionals } = parseArgs({
    args: args.filter((v) => v !== "--"),
    allowPositionals: true,
    options: {
      "dry-run": { type: "boolean" },
      apply: { type: "boolean" },
      json: { type: "boolean" },
      target: { type: "string" },
      reason: { type: "string" },
    },
  });
  const mode = positionals[0] ?? "report";
  if (values.apply && values["dry-run"])
    throw Error("Choose --apply or --dry-run");
  const context = await storageContext(root);
  if (!["report", "clean", "register", "complete", "retire"].includes(mode))
    throw Error("Unknown storage command");
  if (mode === "report") {
    const entries = await snapshot(context);
    const protectedEntries = await Promise.all(
      context.workspaces.flatMap((workspace) =>
        [
          ".local/dev-demo",
          ".local/workspace-archives",
          ".local/workspace-retirement",
          "desktop/resources",
        ].map(async (relative) =>
          measure({
            path: await canonical(join(workspace, relative)),
            root: workspace,
            relative,
          }),
        ),
      ),
    );
    const free = await freeBytes(root),
      budget = limits();
    const result = {
      version: 1,
      workspaces: context.workspaces,
      entries,
      free,
      budget,
      protected: [
        ".local/dev-demo",
        ".local/workspace-archives",
        ".local/workspace-retirement",
        "desktop/resources",
        ".jj",
        ".git",
      ],
      protectedEntries,
      note: "Allocated directory sizes can share APFS blocks; only free-space changes measure recovery.",
    };
    if (values.json) console.log(JSON.stringify(result, null, 2));
    else {
      console.log(
        `Build storage: ${gib(entries.reduce((n, e) => n + e.bytes, 0))} GiB; free ${gib(free)} GiB`,
      );
      for (const entry of entries) {
        console.log(
          `${gib(entry.bytes)} GiB  ${entry.path}${entry.temporary ? ` (${entry.completed ? "completed" : "retained"}: ${entry.reason})` : ""}`,
        );
        console.log(
          `  ${Object.entries(entry.categories)
            .map(([name, bytes]) => `${name} ${gib(bytes)} GiB`)
            .join("; ")}`,
        );
      }
      console.log("Protected state and staged resources:");
      for (const entry of protectedEntries.filter((e) => e.bytes))
        console.log(`${gib(entry.bytes)} GiB  ${entry.path}`);
      console.log(
        `Limits: ${gib(budget.target)} GiB/target, ${gib(budget.total)} GiB total, ${gib(budget.free)} GiB free.\n${result.note}\nProtected application state and staged resources are never cleanup candidates.`,
      );
    }
    return result;
  }
  const registryLock = join(context.store, "registry.lock");
  if (!process.env.JJ_STORE_BUILD_REGISTRY_LOCKED) {
    const { stdout, stderr } = await run(
      "python3",
      [
        helper,
        "lock",
        registryLock,
        "exclusive-wait",
        process.execPath,
        resolve(import.meta.filename),
        ...args,
      ],
      {
        cwd: root,
        env: {
          ...process.env,
          JJ_STORE_BUILD_REGISTRY_LOCKED: "1",
          JJ_STORE_BUILD_ROOT: root,
        },
        maxBuffer: 4 * 1024 * 1024,
      },
    );
    if (stdout) console.log(stdout.trimEnd());
    if (stderr) console.error(stderr.trimEnd());
    return;
  }
  if (mode === "clean" && !values.target) {
    const result = await maintain(context, { dryRun: !values.apply });
    console.log(
      `Free space: ${gib(result.beforeFree)} -> ${gib(result.afterFree)} GiB${result.dryRun ? " (dry run; unchanged)" : ""}`,
    );
    if (result.blocked && values.apply) throw Error(storageFailure(result));
    return result;
  }
  if (!values.target) throw Error("--target is required");
  const path = await canonical(resolve(root, values.target));
  const data = await registry(context);
  if (mode === "register") {
    const relative = path.slice(context.root.length + 1);
    if (
      !path.startsWith(context.root + "/") ||
      !/^\.local\/(?:build-targets\/[a-zA-Z0-9_-]+|[a-zA-Z0-9_-]+-target)$/.test(
        relative,
      )
    )
      throw Error(
        "Temporary targets must use .local/build-targets/NAME or .local/NAME-target",
      );
    if (!values.reason?.trim())
      throw Error("--reason is required for retained targets");
    if (data.targets.some((e) => join(e.root, e.relative) === path))
      throw Error("Target already registered");
    data.targets.push({
      root: context.root,
      relative,
      reason: values.reason,
      completed: false,
    });
    await saveRegistry(context, data);
    console.log(`Registered retained target ${path}`);
    return;
  }
  const entry = (await targets(context)).find((e) => e.path === path);
  if (!entry)
    throw Error("Unknown build target; register temporary targets first");
  if (mode === "complete" || (mode === "retire" && values.apply)) {
    if (!entry.temporary)
      throw Error("Only temporary targets have a completion lifecycle");
    data.targets.find((e) => join(e.root, e.relative) === path).completed =
      true;
    await saveRegistry(context, data);
  }
  if (mode === "complete") return;
  const measured = (await snapshot(context)).find((e) => e.path === path);
  const busy = await active(context, measured);
  if (busy.length) throw Error(`Target cannot be cleaned: ${busy.join("; ")}`);
  if (!values.apply) {
    console.log(`Would clean all build artifacts in ${path}`);
    return;
  }
  const before = await freeBytes(root);
  if (measured.bytes) await cleanTarget(context, measured, "all");
  console.log(
    `Cleaned ${path}; free space ${gib(before)} -> ${gib(await freeBytes(root))} GiB`,
  );
}

if (
  process.argv[1] &&
  import.meta.url === pathToFileURL(resolve(process.argv[1])).href
) {
  try {
    await storageCli(
      process.argv.slice(2),
      process.env.JJ_STORE_BUILD_ROOT ?? resolve(import.meta.dirname, ".."),
    );
  } catch (error) {
    console.error(error.stderr || error.message);
    process.exitCode = 1;
  }
}
