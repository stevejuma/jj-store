import { test } from "node:test";
import assert from "node:assert/strict";
import {
  validationConfig,
  imageKey,
  validateArchives,
  laneImage,
} from "./validation/model.mjs";
test("Docker is default and native must be explicit", () => {
  assert.equal(validationConfig({}, {}).runtime, "docker");
  assert.equal(
    validationConfig({}, { JJ_STORE_VALIDATION_RUNTIME: "native" }).runtime,
    "native",
  );
  assert.throws(() => validationConfig({ runtime: "magic" }, {}), /runtime/);
});
test("remote resource limits are bounded positive numbers", () => {
  assert.equal(validationConfig({}, {}).cpus, 8);
  assert.throws(() => validationConfig({ memoryGiB: 0 }, {}), /memoryGiB/);
});
test("image identity changes with recipe, dependency and architecture inputs", () => {
  const a = imageKey({ recipe: "a", lock: "one", arch: "amd64" });
  assert.notEqual(a, imageKey({ recipe: "a", lock: "two", arch: "amd64" }));
  assert.notEqual(a, imageKey({ recipe: "a", lock: "one", arch: "arm64" }));
  assert.equal(a, imageKey({ arch: "amd64", lock: "one", recipe: "a" }));
});
test("archive inputs require identity, version and exact checksum", () => {
  assert.throws(
    () => validateArchives([{ name: "a", path: "a.tgz" }]),
    /archive/,
  );
  assert.throws(
    () =>
      validateArchives([
        { name: "a", version: "1.0.0", path: "a.tgz", sha256: "a".repeat(64) },
        { name: "a", version: "1.0.0", path: "b.tgz", sha256: "a".repeat(64) },
      ]),
    /Duplicate/,
  );
  assert.equal(
    validateArchives([
      {
        name: "@lapismd/a",
        version: "1.0.0",
        path: "a.tgz",
        sha256: "a".repeat(64),
      },
    ]).length,
    1,
  );
});
test("UI and Rust lanes select the complete toolchain they need", () => {
  assert.equal(laneImage("test:storybook"), "ui");
  assert.equal(laneImage("catalog:check"), "ui");
  assert.equal(laneImage("test:embedded-rustfs"), "full");
  assert.equal(laneImage("spec:check"), "full");
});

test("snapshot preserves a dirty JJ commit and its direct parent", async () => {
  const { mkdtemp, writeFile, mkdir, rm, readFile } =
    await import("node:fs/promises");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const { execFileSync } = await import("node:child_process");
  const { snapshot } = await import("./validation/source.mjs");
  const directory = await mkdtemp(join(tmpdir(), "jj-validation-"));
  const repo = join(directory, "source"),
    destination = join(directory, "input"),
    restored = join(directory, "restored");
  const jj = (args, cwd = repo) =>
    execFileSync(
      "jj",
      [
        "--no-pager",
        "--config",
        'user.name="Validation"',
        "--config",
        'user.email="validation@localhost"',
        ...args,
      ],
      { cwd, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] },
    ).trim();
  try {
    jj(["git", "init", repo], directory);
    await writeFile(join(repo, "kept.txt"), "parent");
    await writeFile(join(repo, "removed.txt"), "delete me");
    jj(["commit", "-m", "Parent"]);
    await writeFile(join(repo, "kept.txt"), "dirty");
    await rm(join(repo, "removed.txt"));
    await writeFile(join(repo, "new.txt"), "new");
    const manifest = await snapshot(repo, destination);
    assert.equal(manifest.parents.length, 1);
    jj(["git", "init", restored], directory);
    execFileSync("git", ["index-pack", "--stdin"], {
      cwd: restored,
      input: await readFile(join(destination, "source.pack")),
      stdio: ["pipe", "pipe", "pipe"],
    });
    execFileSync(
      "git",
      ["update-ref", "refs/heads/validation-snapshot", manifest.revision],
      { cwd: restored },
    );
    jj(["git", "import"], restored);
    jj(["edit", manifest.revision], restored);
    assert.equal(
      jj(["log", "-r", "@", "--no-graph", "-T", "commit_id"], restored),
      manifest.revision,
    );
    assert.equal(
      jj(["log", "-r", "@-", "--no-graph", "-T", "commit_id"], restored),
      manifest.parents[0],
    );
    assert.equal(await readFile(join(restored, "kept.txt"), "utf8"), "dirty");
    assert.equal(await readFile(join(restored, "new.txt"), "utf8"), "new");
    await assert.rejects(readFile(join(restored, "removed.txt")), /ENOENT/);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});
test("SSH context transport quotes arguments and rejects other endpoint types", async () => {
  const { sshDockerCommand } = await import("./validation/docker.mjs");
  const [cmd, args] = sshDockerCommand("ssh://steve@example.test:2222", [
    "run",
    "image",
    "a; echo unexpected",
  ]);
  assert.equal(cmd, "ssh");
  assert.ok(args.includes("2222"));
  assert.match(args.at(-1), /'a; echo unexpected'/);
  assert.throws(
    () => sshDockerCommand("unix:///var/run/docker.sock", []),
    /SSH Docker context/,
  );
});

test("native lane dispatch resolves project binaries and preserves literal arguments", async () => {
  const { mkdtemp, mkdir, writeFile, readFile, rm } =
    await import("node:fs/promises");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const { runNative } = await import("./validation.mjs");
  const directory = await mkdtemp(join(tmpdir(), "jj-validation-dispatch-"));
  try {
    await mkdir(join(directory, "scripts/validation"), { recursive: true });
    await mkdir(join(directory, "node_modules/.bin"), { recursive: true });
    await writeFile(
      join(directory, "scripts/validation/lanes.json"),
      JSON.stringify({ probe: "validation-probe" }),
    );
    await writeFile(
      join(directory, "node_modules/.bin/validation-probe"),
      `#!/usr/bin/env node\nrequire('node:fs').writeFileSync('actual.json',JSON.stringify({args:process.argv.slice(2),runtime:process.env.JJ_STORE_VALIDATION_RUNTIME}));\n`,
      { mode: 0o755 },
    );
    await runNative(
      "probe",
      ["space word", "; echo should-stay-literal"],
      directory,
    );
    assert.deepEqual(
      JSON.parse(await readFile(join(directory, "actual.json"), "utf8")),
      { args: ["space word", "; echo should-stay-literal"], runtime: "native" },
    );
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("default browser suites run complete bounded shards without changing explicit targets", async () => {
  const { laneInvocations } = await import("./validation/model.mjs");
  assert.deepEqual(
    laneInvocations("test:storybook", []),
    Array.from({ length: 4 }, (_, i) => [`--shard=${i + 1}/4`]),
  );
  assert.deepEqual(
    laneInvocations("test:storybook", [
      "Example.stories.svelte",
      "--shard=1/2",
    ]),
    [["Example.stories.svelte", "--shard=1/2"]],
  );
  assert.deepEqual(laneInvocations("web:test", []), [[]]);
});

test("native dispatch keeps bounded full-suite shards and exact explicit targets", async () => {
  const { mkdtemp, mkdir, writeFile, readFile, rm } =
    await import("node:fs/promises");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const { runNativeInvocations } = await import("./validation.mjs");
  const directory = await mkdtemp(join(tmpdir(), "jj-validation-shards-"));
  const recorded = async () =>
    (await readFile(join(directory, "invocations.jsonl"), "utf8"))
      .trim()
      .split("\n")
      .map((line) => JSON.parse(line));
  try {
    await mkdir(join(directory, "scripts/validation"), { recursive: true });
    await mkdir(join(directory, "node_modules/.bin"), { recursive: true });
    await writeFile(
      join(directory, "scripts/validation/lanes.json"),
      JSON.stringify({ "test:storybook": "shard-probe" }),
    );
    await writeFile(
      join(directory, "node_modules/.bin/shard-probe"),
      `#!/usr/bin/env node
const log = require('node:fs');
log.appendFileSync('invocations.jsonl', JSON.stringify(process.argv.slice(2)) + '\\n');
if (process.argv.includes('--shard=2/4')) process.exit(1);
`,
      { mode: 0o755 },
    );
    // Every shard runs even when an earlier one fails, then the first
    // failure surfaces; explicit targets stay a single exact invocation.
    await assert.rejects(runNativeInvocations("test:storybook", [], directory));
    assert.deepEqual(await recorded(), [
      ["--shard=1/4"],
      ["--shard=2/4"],
      ["--shard=3/4"],
      ["--shard=4/4"],
    ]);
    await rm(join(directory, "invocations.jsonl"));
    await runNativeInvocations(
      "test:storybook",
      ["Example.stories.svelte"],
      directory,
    );
    assert.deepEqual(await recorded(), [["Example.stories.svelte"]]);
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});

test("source snapshots preserve a separate Jujutsu workspace without a Git worktree", async () => {
  const { mkdtemp, writeFile, rm } = await import("node:fs/promises");
  const { tmpdir } = await import("node:os");
  const { join } = await import("node:path");
  const { execFileSync } = await import("node:child_process");
  const { snapshot } = await import("./validation/source.mjs");
  const temp = await mkdtemp(join(tmpdir(), "jj-separate-validation-"));
  const main = join(temp, "main"),
    secondary = join(temp, "secondary");
  const jj = (cwd, args) =>
    execFileSync(
      "jj",
      [
        "--config",
        "user.name=Validation",
        "--config",
        "user.email=validation@localhost",
        ...args,
      ],
      { cwd, encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] },
    ).trim();
  try {
    jj(temp, ["git", "init", "--colocate", main]);
    await writeFile(join(main, "file.txt"), "parent");
    jj(main, ["commit", "-m", "parent"]);
    jj(main, ["workspace", "add", secondary, "--name", "validation"]);
    await writeFile(join(secondary, "file.txt"), "dirty source");
    const source = await snapshot(secondary, join(temp, "input"));
    assert.equal(
      source.revision,
      jj(secondary, ["log", "-r", "@", "--no-graph", "-T", "commit_id"]),
    );
    assert.deepEqual(
      source.parents,
      jj(secondary, ["log", "-r", "@-", "--no-graph", "-T", "commit_id"]).split(
        "\n",
      ),
    );
  } finally {
    await rm(temp, { recursive: true, force: true });
  }
});
