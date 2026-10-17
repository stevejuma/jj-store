import { createHash } from "node:crypto";
import { execFile } from "node:child_process";
import {
  lstat,
  readFile,
  realpath,
  mkdir,
  writeFile,
  rename,
} from "node:fs/promises";
import { homedir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { promisify } from "node:util";

export const run = promisify(execFile);
export const cargoRoots = [
  [".", "target"],
  ["desktop", "desktop/target"],
  ["runtime/embedded-rustfs", "runtime/embedded-rustfs/target"],
  [
    "crates/jj-store-jj/tests/fixtures/store-consumer",
    "crates/jj-store-jj/tests/fixtures/store-consumer/target",
  ],
];
export const helper = resolve(import.meta.dirname, "build-storage-lock.py");

export async function canonical(path) {
  path = resolve(path);
  try {
    const actual = await realpath(path);
    if (actual !== path) throw Error(`Refusing symlink path: ${path}`);
    return actual;
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    await canonical(dirname(path));
    return path;
  }
}

export async function storageContext(root) {
  root = await realpath(root);
  let repository = join(root, ".jj/repo");
  try {
    if ((await lstat(repository)).isFile())
      repository = resolve(
        dirname(repository),
        (await readFile(repository, "utf8")).trim(),
      );
    repository = await realpath(repository);
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    repository = root;
  }
  const identity = createHash("sha256").update(repository).digest("hex");
  const store = join(homedir(), ".cache/jj-store-build", identity);
  let workspaces = [root];
  try {
    const { stdout } = await run(
      "jj",
      [
        "--ignore-working-copy",
        "--no-pager",
        "--color=never",
        "workspace",
        "list",
        "-T",
        'json(root) ++ "\n"',
      ],
      { cwd: root },
    );
    workspaces = [
      ...new Set([
        root,
        ...stdout
          .trim()
          .split("\n")
          .filter(Boolean)
          .map(JSON.parse)
          .filter(Boolean)
          .map((p) => resolve(root, p)),
      ]),
    ];
  } catch (error) {
    if (repository !== root)
      throw Error("Cannot discover JJ workspaces", { cause: error });
  }
  return { root, repository, store, workspaces };
}

export function targetLock(context, path) {
  return join(
    context.store,
    "locks",
    createHash("sha256").update(path).digest("hex"),
  );
}

export async function registry(context) {
  try {
    const value = JSON.parse(
      await readFile(join(context.store, "targets.json"), "utf8"),
    );
    if (value.version !== 1 || !Array.isArray(value.targets))
      throw Error("Invalid build registry");
    return value;
  } catch (error) {
    if (error.code !== "ENOENT") throw error;
    return { version: 1, targets: [] };
  }
}

export async function saveRegistry(context, value) {
  await mkdir(context.store, { recursive: true, mode: 0o700 });
  const path = join(context.store, "targets.json");
  const temporary = `${path}.${process.pid}.tmp`;
  await writeFile(temporary, JSON.stringify(value, null, 2) + "\n", {
    mode: 0o600,
  });
  await rename(temporary, path);
}

export async function targets(context) {
  const result = [];
  for (const root of context.workspaces) {
    for (const [manifest, target] of cargoRoots) {
      try {
        await canonical(root);
        if (!(await lstat(join(root, manifest, "Cargo.toml"))).isFile())
          continue;
        result.push({
          path: await canonical(join(root, target)),
          manifest: join(root, manifest, "Cargo.toml"),
          root,
          temporary: false,
        });
      } catch (error) {
        if (error.code !== "ENOENT") throw error;
      }
    }
  }
  for (const entry of (await registry(context)).targets) {
    if (
      !/^\.local\/(?:build-targets\/[a-zA-Z0-9_-]+|[a-zA-Z0-9_-]+-target)$/.test(
        entry.relative,
      )
    )
      throw Error("Unsafe registered target");
    const root = await canonical(entry.root);
    result.push({
      ...entry,
      root,
      path: await canonical(join(root, entry.relative)),
      manifest: join(root, "Cargo.toml"),
      temporary: true,
    });
  }
  return [...new Map(result.map((entry) => [entry.path, entry])).values()];
}

export async function measure(entry) {
  try {
    const { stdout } = await run("python3", [helper, "inspect", entry.path], {
      maxBuffer: 1024 * 1024,
    });
    return { ...entry, ...JSON.parse(stdout) };
  } catch (error) {
    throw Error(
      `Cannot inspect ${entry.path}: ${error.stderr || error.message}`,
    );
  }
}

export async function snapshot(context) {
  return Promise.all((await targets(context)).map(measure));
}

export async function locked(context, path, mode, command, args, options = {}) {
  return run(
    "python3",
    [helper, "lock", targetLock(context, path), mode, command, ...args],
    options,
  );
}
