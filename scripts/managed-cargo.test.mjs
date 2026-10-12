import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtemp,
  mkdir,
  writeFile,
  readFile,
  rm,
  realpath,
  readdir,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { cargoSelection, cargoCommand } from "./managed-cargo.mjs";
import { storageContext, run } from "./build-storage-inventory.mjs";

const wrapper = resolve(import.meta.dirname, "managed-cargo.mjs");
const roomy = {
  JJ_STORE_BUILD_TARGET_GIB: "1",
  JJ_STORE_BUILD_TOTAL_GIB: "2",
  JJ_STORE_BUILD_FREE_GIB: "0.001",
  CARGO_TARGET_DIR: "",
};
async function workspace(t) {
  const root = await realpath(
    await mkdtemp(join(tmpdir(), "jj-store-managed-cargo-")),
  );
  t.after(() => rm(root, { recursive: true, force: true }));
  await mkdir(join(root, "src"));
  await writeFile(
    join(root, "Cargo.toml"),
    '[workspace]\n[package]\nname="managed-proof"\nversion="0.1.0"\nedition="2024"\n',
  );
  await writeFile(
    join(root, "src/main.rs"),
    'fn main() { println!("managed proof"); }\n#[test] fn example(){assert_eq!(2+2,4);}\n',
  );
  const context = await storageContext(root);
  t.after(() => rm(context.store, { recursive: true, force: true }));
  await run("cargo", ["generate-lockfile", "--offline"], { cwd: root });
  return context;
}

test("managed Cargo pins targets before the test separator and rejects unknown directories", async (t) => {
  const context = await workspace(t);
  const selection = await cargoSelection(
    context,
    ["test", "--", "--exact", "example"],
    {},
  );
  assert.deepEqual(selection.args.slice(-5), [
    "--target-dir",
    join(context.root, "target"),
    "--",
    "--exact",
    "example",
  ]);
  const explicit = await cargoSelection(
    context,
    ["build", "--target-dir", "target"],
    {},
  );
  assert.equal(explicit.args.filter((v) => v === "--target-dir").length, 1);
  await assert.rejects(
    cargoSelection(context, ["build", "--target-dir", ".local/dev-demo"], {}),
    /registered/,
  );
  await assert.rejects(
    cargoSelection(
      context,
      ["build", "--config", 'build.target-dir="elsewhere"'],
      {},
    ),
    /configuration/,
  );
  assert.equal(cargoCommand(["test"])[0], process.execPath);
});

test("two managed workspaces reuse unchanged builds and keep independent executables", async (t) => {
  let first = await workspace(t),
    second = await workspace(t);
  await writeFile(join(first.root, ".gitignore"), "/target/\n");
  await run("jj", ["git", "init", first.root]);
  await run(
    "jj",
    ["--no-pager", "describe", "-m", "Disposable build fixture"],
    { cwd: first.root },
  );
  await rm(second.root, { recursive: true, force: true });
  await run(
    "jj",
    [
      "workspace",
      "add",
      second.root,
      "-r",
      "@",
      "--name",
      "build-proof",
      "-m",
      "Disposable sibling build",
    ],
    { cwd: first.root },
  );
  first = await storageContext(first.root);
  second = await storageContext(second.root);
  t.after(() => rm(first.store, { recursive: true, force: true }));
  assert.equal(first.store, second.store);
  assert.equal(first.workspaces.length, 2);
  const build = (context, args = ["build", "--locked", "--offline"]) =>
    run(process.execPath, [wrapper, ...args], {
      cwd: context.root,
      env: { ...process.env, ...roomy, JJ_STORE_BUILD_ROOT: context.root },
      maxBuffer: 4 * 1024 * 1024,
    });
  await Promise.all([build(first), build(second)]);
  const again = await build(first, [
    "build",
    "--locked",
    "--offline",
    "--verbose",
  ]);
  assert.match(again.stderr, /Fresh managed-proof/);
  assert.doesNotMatch(again.stderr, /Compiling managed-proof/);
  assert.equal(
    (
      await run(join(first.root, "target/debug/managed-proof"), [])
    ).stdout.trim(),
    "managed proof",
  );
  assert.equal(
    (
      await run(join(second.root, "target/debug/managed-proof"), [])
    ).stdout.trim(),
    "managed proof",
  );
  await build(first, [
    "test",
    "--locked",
    "--offline",
    "--",
    "--exact",
    "example",
  ]);
  assert.ok((await readdir(join(first.root, "target/debug/deps"))).length);
});

test("failed maintenance prevents compilation and preserves a live target", async (t) => {
  const context = await workspace(t);
  await mkdir(join(context.root, "target/debug"), { recursive: true });
  await writeFile(join(context.root, "target/debug/output"), "preserve");
  await mkdir(join(context.root, "target/.jj"));
  await assert.rejects(
    run(process.execPath, [wrapper, "build", "--offline", "--locked"], {
      env: {
        ...process.env,
        ...roomy,
        JJ_STORE_BUILD_TARGET_GIB: "0.000000001",
        JJ_STORE_BUILD_ROOT: context.root,
      },
    }),
    (error) => /limits exceeded/.test(error.stderr),
  );
  assert.equal(
    await readFile(join(context.root, "target/debug/output"), "utf8"),
    "preserve",
  );
  await assert.rejects(
    readFile(join(context.root, "target/debug/managed-proof")),
  );
});

test("repository Rust build entry points use the managed wrapper", async () => {
  const packageJson = JSON.parse(
    await readFile(new URL("../package.json", import.meta.url), "utf8"),
  );
  for (const [name, command] of Object.entries(packageJson.scripts))
    assert.doesNotMatch(
      command,
      /\bcargo (?:build|check|clippy|test|run)\b/,
      name,
    );
  for (const file of [
    "desktop.mjs",
    "contracts.mjs",
    "browser-fixtures.mjs",
    "install-cli.mjs",
    "profile-acceptance.mjs",
    "dev-demo-services.mjs",
    "docs/references.mjs",
  ])
    assert.match(
      await readFile(new URL(file, import.meta.url), "utf8"),
      /cargoCommand/,
    );
});
