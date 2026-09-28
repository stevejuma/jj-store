import { randomBytes } from "node:crypto";
import { mkdir, writeFile } from "node:fs/promises";
import { join, resolve } from "node:path";

export async function seedRepository(options) {
  const { repositoryRoot, runtimeRoot, run, unixSocket } = options;
  const repository = join(runtimeRoot, "repository");
  const config = join(runtimeRoot, "jj.toml");
  await writeFile(
    config,
    '[user]\nname = "Maya Chen"\nemail = "maya@example.invalid"\n',
  );
  const nativeEnv = {
    JJ_CONFIG: config,
  };
  const created = await run(
    resolve(repositoryRoot, "target/debug/jj"),
    [
      "store",
      "init",
      repository,
      "--unix-socket",
      unixSocket,
      "--cache",
      join(runtimeRoot, "cache"),
      "--state-dir",
      join(runtimeRoot, "init-state"),
      "--workspace",
      "demo",
      "--format=json",
    ],
    { env: nativeEnv },
  );
  const repositoryId = JSON.parse(created.stdout).repository;
  const jj = (args) =>
    run(
      resolve(repositoryRoot, "target/debug/jj"),
      ["-R", repository, "--no-pager", "--color=never", ...args],
      { env: nativeEnv },
    );
  await seedLinearHistory(repository, jj);
  await seedConflict(repository, jj);
  await seedDivergence(jj);
  return repositoryId;
}

async function seedLinearHistory(repository, jj) {
  await mkdir(join(repository, "src"), { recursive: true });
  await mkdir(join(repository, "assets"), { recursive: true });
  await writeFile(
    join(repository, "README.md"),
    "# jj-store demo\n\nThis native repository is created by `pnpm dev:restart`.\n",
  );
  await writeFile(join(repository, "LICENSE"), "Apache-2.0\n");
  await writeFile(
    join(repository, "Cargo.toml"),
    '[package]\nname = "jj-store-demo"\nversion = "0.1.0"\nedition = "2024"\n',
  );
  await writeFile(
    join(repository, "src/lib.rs"),
    'pub fn repository_name() -> &\'static str {\n    "jj-store demo"\n}\n',
  );
  await writeFile(join(repository, "assets/logo.bin"), randomBytes(96));
  await jj(["describe", "-m", "Bootstrap native repository"]);
  await jj(["bookmark", "create", "release/0.1", "-r", "@"]);
  await jj(["new", "-m", "Add repository browser"]);
  await writeFile(
    join(repository, "src/lib.rs"),
    'pub fn repository_name() -> &\'static str {\n    "jj-store browser demo"\n}\n\npub fn supports_native_history() -> bool { true }\n',
  );
  await jj(["describe", "-m", "Add repository browser"]);
  await jj(["bookmark", "create", "main", "-r", "@"]);
  await jj(["new", "-m", "Experiment with persistent layouts"]);
  await writeFile(
    join(repository, "LAYOUT.md"),
    "# Layout experiment\n\nColumn widths persist between sessions.\n",
  );
  await jj(["describe", "-m", "Experiment with persistent layouts"]);
  await jj(["bookmark", "create", "experiment/layout", "-r", "@"]);
  await jj(["new", "-m", "Working copy"]);
}

async function seedConflict(repository, jj) {
  await writeFile(
    join(repository, "conflict.txt"),
    "# Browser conflict policy\nmode = conservative\ntimeout = 30\n",
  );
  await jj(["describe", "-m", "Conflict base"]);
  const base = (await jj(["log", "-r", "@", "-T", "commit_id", "--no-graph"]))
    .stdout;
  await jj(["new", base, "-m", "Conflict left"]);
  await writeFile(
    join(repository, "conflict.txt"),
    "# Browser conflict policy\nmode = prefer-local\nreviewer = Maya\n",
  );
  await jj(["describe", "-m", "Conflict left"]);
  const left = (await jj(["log", "-r", "@", "-T", "commit_id", "--no-graph"]))
    .stdout;
  await jj(["new", base, "-m", "Conflict right"]);
  await writeFile(
    join(repository, "conflict.txt"),
    "# Browser conflict policy\nmode = prefer-remote\nretry = 3\n",
  );
  await jj(["describe", "-m", "Conflict right"]);
  const right = (await jj(["log", "-r", "@", "-T", "commit_id", "--no-graph"]))
    .stdout;
  await jj(["new", left, right, "-m", "Unresolved browser conflict"]);
  await jj(["bookmark", "create", "conflict/demo", "-r", "@"]);
  await jj(["new", "experiment/layout", "-m", "Working copy"]);
}

async function seedDivergence(jj) {
  await jj(["new", "experiment/layout", "-m", "Divergent layout baseline"]);
  const change = (await jj(["log", "-r", "@", "-T", "change_id", "--no-graph"]))
    .stdout;
  const operation = (
    await jj([
      "op",
      "log",
      "--at-op=@",
      "--ignore-working-copy",
      "-n",
      "1",
      "--no-graph",
      "-T",
      "self.id()",
    ])
  ).stdout;
  await jj([
    `--at-op=${operation}`,
    "describe",
    "-r",
    change,
    "-m",
    "Divergent layout alpha",
  ]);
  await jj([
    `--at-op=${operation}`,
    "describe",
    "-r",
    change,
    "-m",
    "Divergent layout beta",
  ]);
  await jj(["new", "experiment/layout", "-m", "Working copy"]);
}
