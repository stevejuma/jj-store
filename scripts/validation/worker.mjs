import {
  readFile,
  writeFile,
  mkdir,
  copyFile,
  rm,
  readdir,
} from "node:fs/promises";
import { createReadStream } from "node:fs";
import { execFileSync, spawn } from "node:child_process";
import { join } from "node:path";
import {
  hash,
  validateArchives,
  laneInvocations,
  workerArguments,
} from "./model.mjs";
import { execute } from "./process.mjs";
const root = "/work/jj-store",
  input = "/input",
  reports = "/results";
const source = JSON.parse(await readFile(join(input, "source.json"), "utf8"));
const request = JSON.parse(await readFile(join(input, "request.json"), "utf8"));
const run = (command, args, options = {}) =>
  execFileSync(command, args, { cwd: root, stdio: "inherit", ...options });
let result = {
  schema: 1,
  ...request,
  source,
  started: new Date().toISOString(),
  status: "running",
};
await mkdir(reports, { recursive: true });
try {
  if (hash(await readFile(join(input, "source.pack"))) !== source.packSha256)
    throw Error("Source pack checksum mismatch");
  // Git plumbing imports objects only; Jujutsu owns checkout and revision selection.
  await writeFile(
    join(root, ".gitignore"),
    "target/\nnode_modules/\n.local/\nruntime/embedded-rustfs/target/\n",
  );
  run("jj", ["git", "init", "--colocate", root]);
  const pack = spawn("git", ["index-pack", "--stdin"], {
    cwd: root,
    stdio: ["pipe", "inherit", "inherit"],
  });
  const imported = new Promise((accept, reject) => {
    pack.once("error", reject);
    pack.once("exit", (code) =>
      code === 0 ? accept() : reject(Error("Object import failed")),
    );
  });
  createReadStream(join(input, "source.pack")).pipe(pack.stdin);
  await imported;
  run("git", ["update-ref", "refs/heads/validation-snapshot", source.revision]);
  run("jj", ["git", "import"]);
  run("jj", [
    "--config",
    'user.name="Validation"',
    "--config",
    'user.email="validation@localhost"',
    "edit",
    source.revision,
  ]);
  const actual = execFileSync(
    "jj",
    [
      "--ignore-working-copy",
      "log",
      "-r",
      "@",
      "--no-graph",
      "-T",
      "commit_id",
    ],
    { cwd: root, encoding: "utf8" },
  ).trim();
  if (actual !== source.revision) throw Error("Snapshot revision mismatch");
  const parents = execFileSync(
    "jj",
    [
      "--ignore-working-copy",
      "log",
      "-r",
      "@-",
      "--no-graph",
      "-T",
      'commit_id ++ "\\n"',
    ],
    { cwd: root, encoding: "utf8" },
  )
    .trim()
    .split("\n")
    .filter(Boolean);
  if (
    JSON.stringify(parents.sort()) !==
    JSON.stringify([...source.parents].sort())
  )
    throw Error("Snapshot parents mismatch");
  process.env.JJ_STORE_VALIDATION_WORKER = "1";
  process.env.JJ_STORE_VALIDATION_RUNTIME = "native";
  process.env.JJ_STORE_CHECK_REVISION = source.revision;
  if (source.base) process.env.JJ_STORE_CHECK_BASE = source.base;
  process.env.JJ_STORE_CI_REPORTS = "1";
  process.env.PNPM_HOME = "/opt/pnpm";
  const archives = validateArchives(source.archives);
  if (archives.length) {
    await execute("pnpm", ["fetch", "--frozen-lockfile", "--ignore-pnpmfile"], {
      cwd: root,
    });
    const overrides = {};
    await mkdir(join(root, ".local/validation-packages"), { recursive: true });
    for (const entry of archives) {
      const bytes = await readFile(join(input, entry.path));
      if (hash(bytes) !== entry.sha256)
        throw Error(`Archive checksum mismatch: ${entry.name}`);
      const identity = JSON.parse(
        execFileSync(
          "tar",
          ["-xOf", join(input, entry.path), "package/package.json"],
          { encoding: "utf8" },
        ),
      );
      if (identity.name !== entry.name || identity.version !== entry.version)
        throw Error("Package archive identity mismatch");
      const destination = join(root, ".local/validation-packages", entry.path);
      await copyFile(join(input, entry.path), destination);
      overrides[entry.name] = `file:${destination}`;
    }
    // An isolated install projection; restore source manifests before validation.
    const packagePath = join(root, "package.json"),
      lockPath = join(root, "pnpm-lock.yaml");
    const original = await readFile(packagePath),
      lock = await readFile(lockPath);
    const pkg = JSON.parse(original);
    pkg.pnpm = {
      ...pkg.pnpm,
      overrides: { ...pkg.pnpm?.overrides, ...overrides },
    };
    try {
      await writeFile(packagePath, JSON.stringify(pkg));
      await execute(
        "pnpm",
        ["install", "--no-frozen-lockfile", "--ignore-pnpmfile"],
        { cwd: root },
      );
    } finally {
      await writeFile(packagePath, original);
      await writeFile(lockPath, lock);
    }
  } else
    await execute(
      "pnpm",
      ["install", "--frozen-lockfile", "--ignore-pnpmfile"],
      { cwd: root },
    );
  if (request.imageTarget === "full") {
    const { prepareFull } = await import(
      join(root, "scripts/validation/prepare.mjs")
    );
    await prepareFull(root, input, source, request.lane);
  }
  const { runNative } = await import(join(root, "scripts/validation.mjs"));
  const args = workerArguments(request.lane, request.args, source);
  await mkdir(join(root, ".release/ci-results"), { recursive: true });
  if (request.lane.startsWith("test:visual")) {
    process.env.VISUAL_DELTA_CAPTURE_PROFILE = "container-linux";
    process.env.VISUAL_DELTA_NO_LOCAL_CONFIG = "1";
  }
  result.tools = Object.fromEntries(
    ["node", "pnpm", "jj", "git"].map((tool) => [
      tool,
      execFileSync(tool, ["--version"], { encoding: "utf8" }).trim(),
    ]),
  );
  result.invocations = [];
  for (const [index, invocation] of laneInvocations(
    request.lane,
    args,
  ).entries()) {
    if (
      request.lane === "test:storybook" &&
      !invocation.some((arg) => arg.startsWith("--reporter"))
    )
      invocation.push(
        "--reporter=default",
        "--reporter=junit",
        `--outputFile=.release/ci-results/storybook-${index + 1}.xml`,
      );
    const execution = { args: invocation, status: "running" };
    result.invocations.push(execution);
    try {
      await runNative(request.lane, invocation, root);
      execution.status = "passed";
    } catch (error) {
      execution.status = "failed";
      execution.error = error.message;
    }
  }
  if (result.invocations.some((item) => item.status === "failed"))
    throw Error(
      "Validation invocation failed; inspect the per-invocation reports.",
    );
  result.status = "passed";
  result.exitCode = 0;
} catch (error) {
  console.error(error);
  result.status = "failed";
  result.exitCode = 1;
  result.error = error.message;
} finally {
  result.finished = new Date().toISOString();
  await writeFile(
    join(reports, "result.json"),
    JSON.stringify(result, null, 2),
  );
  for (const path of [
    ".release/ci-results",
    "test-results",
    "web/test-results",
    "web/test-results-help",
    "playwright-report",
    "storybook-static",
    "catalog/test-results",
    ".visual-delta/cache/profiles/container-linux",
  ]) {
    try {
      if (!(await import("node:fs")).existsSync(join(root, path))) continue;
      await execute("cp", [
        "-a",
        join(root, path),
        join(reports, path.replaceAll("/", "_")),
      ]);
    } catch {
      /* Optional output is absent for some lanes. */
    }
  }
}
process.exitCode = result.exitCode;
