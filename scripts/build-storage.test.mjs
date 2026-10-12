import test from "node:test";
import assert from "node:assert/strict";
import { spawn } from "node:child_process";
import { once } from "node:events";
import {
  mkdtemp,
  mkdir,
  writeFile,
  readFile,
  rm,
  realpath,
  symlink,
  access,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import {
  storageContext,
  snapshot,
  registry,
  saveRegistry,
  helper,
  locked,
  run,
  canonical,
} from "./build-storage-inventory.mjs";
import {
  limits,
  pressure,
  cleanupOrder,
  maintain,
  active,
  cleanTarget,
} from "./build-storage-policy.mjs";

async function fixture(t) {
  const root = await realpath(
    await mkdtemp(join(tmpdir(), "jj-store-build-storage-")),
  );
  t.after(() => rm(root, { recursive: true, force: true }));
  await mkdir(join(root, "src"));
  await writeFile(
    join(root, "Cargo.toml"),
    '[workspace]\n[package]\nname="cache-proof"\nversion="0.1.0"\nedition="2024"\n',
  );
  await writeFile(
    join(root, "src/main.rs"),
    'fn main() { println!("proof"); }',
  );
  const context = await storageContext(root);
  context.store = join(root, ".leases");
  await mkdir(join(root, "target/debug/deps"), { recursive: true });
  await writeFile(
    join(root, "target/CACHEDIR.TAG"),
    "Signature: 8a477f597d28d172789f06886806bc55\n",
  );
  await writeFile(join(root, "target/debug/deps/old"), "build output");
  return context;
}

async function waitLine(child) {
  const [data] = await once(child.stdout, "data");
  assert.match(String(data), /ready/);
}

test("storage thresholds and cleanup order retain active temporary targets", () => {
  const budget = limits({});
  assert.equal(budget.target, 40 * 1024 ** 3);
  assert.equal(budget.total, 80 * 1024 ** 3);
  assert.equal(budget.free, 30 * 1024 ** 3);
  assert.throws(() => limits({ JJ_STORE_BUILD_FREE_GIB: "0" }));
  assert.ok(pressure([{ bytes: budget.target + 1 }], budget.free, budget));
  assert.ok(pressure([], budget.free - 1, budget));
  const entries = [
    {
      path: "active",
      bytes: 1,
      temporary: true,
      completed: false,
      modified: 0,
    },
    { path: "normal", bytes: 1, temporary: false, modified: 1 },
    { path: "old", bytes: 1, temporary: true, completed: true, modified: 2 },
  ];
  assert.deepEqual(
    cleanupOrder(entries).map((e) => e.path),
    ["old", "normal"],
  );
  assert.equal(
    cleanupOrder(
      [
        { path: "small", bytes: 1, modified: 0 },
        { path: "oversized", bytes: 20, modified: 100 },
      ],
      10,
    )[0].path,
    "oversized",
  );
});

test("dry run reports cleanup and preserves cache and application state", async (t) => {
  const context = await fixture(t);
  const sentinel = join(context.root, ".local/dev-demo/repository");
  await mkdir(join(context.root, ".local/dev-demo"), { recursive: true });
  await writeFile(sentinel, "preserved state");
  const result = await maintain(context, {
    dryRun: true,
    environment: { JJ_STORE_BUILD_TARGET_GIB: "0.000000001" },
    log: () => {},
  });
  assert.equal(result.actions.length, 1);
  assert.equal(result.actions[0].kind, "workspace");
  assert.equal(result.beforeFree, result.afterFree);
  assert.equal(await readFile(sentinel, "utf8"), "preserved state");
  assert.equal(
    await readFile(join(context.root, "target/debug/deps/old"), "utf8"),
    "build output",
  );
});

test("symlink roots and protected cache entries refuse deletion", async (t) => {
  const context = await fixture(t);
  const target = join(context.root, "target");
  await symlink(target, join(context.root, "alias"));
  await assert.rejects(canonical(join(context.root, "alias")), /symlink/);
  await mkdir(join(target, ".jj"));
  let entry = (await snapshot(context))[0];
  assert.match((await active(context, entry))[0], /unsafe/);
  await assert.rejects(cleanTarget(context, entry, "all"), /Command failed/);
  await rm(join(target, ".jj"), { recursive: true });
  await symlink(join(context.root, "Cargo.toml"), join(target, "source"));
  entry = (await snapshot(context))[0];
  await assert.rejects(cleanTarget(context, entry, "all"));
  assert.match(
    await readFile(join(context.root, "Cargo.toml"), "utf8"),
    /cache-proof/,
  );
});

test("managed build leases block cleanup while independent targets can build", async (t) => {
  const context = await fixture(t);
  const entry = (await snapshot(context))[0];
  const lease = spawn("python3", [
    helper,
    "lock",
    join(context.store, "locks", "test"),
    "shared",
    "python3",
    "-c",
    'import time; print("ready", flush=True); time.sleep(60)',
  ]);
  t.after(() => lease.kill("SIGTERM"));
  await waitLine(lease);
  await assert.rejects(
    run("python3", [
      helper,
      "lock",
      join(context.store, "locks", "test"),
      "exclusive",
      "true",
    ]),
  );
  await run("python3", [
    helper,
    "lock",
    join(context.store, "locks", "other"),
    "shared",
    "true",
  ]);
  const { targetLock } = await import("./build-storage-inventory.mjs");
  const build = spawn("python3", [
    helper,
    "lock",
    targetLock(context, entry.path),
    "shared",
    "python3",
    "-c",
    'import time; print("ready", flush=True); time.sleep(60)',
  ]);
  t.after(() => build.kill("SIGTERM"));
  await waitLine(build);
  await assert.rejects(cleanTarget(context, entry, "all"));
  const exited = once(build, "exit");
  build.kill("SIGTERM");
  await exited;
  await cleanTarget(context, entry, "all");
  await assert.rejects(access(join(entry.path, "debug/deps/old")));
});

test("native Cargo locks and running target executables refuse cleanup", async (t) => {
  const context = await fixture(t);
  const target = join(context.root, "target");
  const lock = spawn("python3", [
    "-c",
    'import fcntl,sys,time; f=open(sys.argv[1],"w"); fcntl.flock(f,fcntl.LOCK_EX); print("ready",flush=True); time.sleep(60)',
    join(target, "debug/.cargo-build-lock"),
  ]);
  t.after(() => lock.kill("SIGTERM"));
  await waitLine(lock);
  const entry = (await snapshot(context))[0];
  await assert.rejects(cleanTarget(context, entry, "all"));
  const stopped = once(lock, "exit");
  lock.kill("SIGTERM");
  await stopped;
  const consumer = spawn("python3", [
    "-c",
    'import sys,time; f=open(sys.argv[1]); print("ready",flush=True); time.sleep(60)',
    join(target, "debug/deps/old"),
  ]);
  t.after(() => consumer.kill("SIGTERM"));
  await waitLine(consumer);
  assert.ok((await active(context, entry)).length);
  await assert.rejects(cleanTarget(context, entry, "all"));
});

test("Cargo-selected first-party cleanup retains dependency artifacts and can be repeated", async (t) => {
  const context = await fixture(t);
  await mkdir(join(context.root, "dependency/src"), { recursive: true });
  await writeFile(
    join(context.root, "dependency/Cargo.toml"),
    '[package]\nname="retained-dependency"\nversion="0.1.0"\nedition="2024"\n',
  );
  await writeFile(
    join(context.root, "dependency/src/lib.rs"),
    "pub fn value()->u8 { 42 }",
  );
  await writeFile(
    join(context.root, "Cargo.toml"),
    '[workspace]\nexclude=["dependency"]\n[package]\nname="cache-proof"\nversion="0.1.0"\nedition="2024"\n[dependencies]\nretained-dependency={path="dependency"}\n',
  );
  await run("cargo", ["build", "--offline"], { cwd: context.root });
  const entry = (await snapshot(context))[0];
  const { readdir } = await import("node:fs/promises");
  const dependency = (await readdir(join(entry.path, "debug/deps"))).find((n) =>
    /^libretained_dependency.*rlib$/.test(n),
  );
  assert.ok(dependency);
  const binary = (await readdir(join(entry.path, "debug/deps"))).find((n) =>
    /^cache_proof-[a-f0-9]+$/.test(n),
  );
  assert.ok(binary);
  await cleanTarget(context, entry, "workspace");
  await access(join(entry.path, "debug/deps", dependency));
  await assert.rejects(access(join(entry.path, "debug/deps", binary)));
  await assert.rejects(access(join(entry.path, "debug/cache-proof")));
  await cleanTarget(context, entry, "workspace");
  await cleanTarget(context, entry, "all");
  await access(join(entry.path, "debug/.cargo-build-lock"));
  await cleanTarget(context, entry, "all");
});

test("temporary target completion is explicit and automatic cleanup prioritizes it", async (t) => {
  const context = await fixture(t);
  const data = await registry(context);
  data.targets.push({
    root: context.root,
    relative: ".local/build-targets/qualification",
    completed: false,
    reason: "qualification in progress",
  });
  await saveRegistry(context, data);
  await mkdir(join(context.root, ".local/build-targets/qualification"), {
    recursive: true,
  });
  await writeFile(
    join(context.root, ".local/build-targets/qualification/CACHEDIR.TAG"),
    "Signature: 8a477f597d28d172789f06886806bc55\n",
  );
  await writeFile(
    join(context.root, ".local/build-targets/qualification/artifact"),
    "temporary",
  );
  assert.equal(
    cleanupOrder(await snapshot(context)).some((e) => e.temporary),
    false,
  );
  data.targets[0].completed = true;
  await saveRegistry(context, data);
  assert.equal(cleanupOrder(await snapshot(context))[0].temporary, true);
  const result = await maintain(context, {
    environment: { JJ_STORE_BUILD_TARGET_GIB: "0.000000001" },
    log: () => {},
  });
  assert.equal(result.actions[0].kind, "all");
  assert.equal(result.actions[0].reason, "completed temporary target");
});

test("cleanup rejects missing Cargo markers and preserves running executables", async (t) => {
  const context = await fixture(t);
  const entry = (await snapshot(context))[0];
  await rm(join(entry.path, "CACHEDIR.TAG"));
  await assert.rejects(cleanTarget(context, entry, "all"));
  await writeFile(
    join(entry.path, "CACHEDIR.TAG"),
    "Signature: 8a477f597d28d172789f06886806bc55\n",
  );
  await writeFile(
    join(context.root, "src/main.rs"),
    "fn main(){std::thread::sleep(std::time::Duration::from_secs(60));}",
  );
  await run("cargo", ["build", "--offline"], { cwd: context.root });
  const binary = join(entry.path, "debug/cache-proof");
  const child = spawn(binary, []);
  t.after(() => child.kill("SIGTERM"));
  await once(child, "spawn");
  assert.ok(
    (await active(context, entry)).some((reason) =>
      reason.includes(String(child.pid)),
    ),
  );
  await assert.rejects(cleanTarget(context, entry, "all"));
});

test("temporary retirement dry run preserves lifecycle and unknown roots are rejected", async (t) => {
  const context = await fixture(t);
  const actual = await storageContext(context.root);
  t.after(() => rm(actual.store, { recursive: true, force: true }));
  const cli = async (...args) =>
    run(
      process.execPath,
      [join(import.meta.dirname, "build-storage.mjs"), ...args],
      {
        env: { ...process.env, JJ_STORE_BUILD_ROOT: context.root },
        maxBuffer: 4 * 1024 * 1024,
      },
    );
  await assert.rejects(
    cli(
      "register",
      "--target",
      ".local/dev-demo",
      "--reason",
      "protected state",
    ),
  );
  await cli(
    "register",
    "--target",
    ".local/build-targets/finished",
    "--reason",
    "disposable qualification",
  );
  const path = join(context.root, ".local/build-targets/finished");
  await mkdir(path, { recursive: true });
  await writeFile(
    join(path, "CACHEDIR.TAG"),
    "Signature: 8a477f597d28d172789f06886806bc55\n",
  );
  await writeFile(join(path, "output"), "disposable");
  await cli("retire", "--target", path, "--dry-run");
  assert.equal((await registry(actual)).targets[0].completed, false);
  assert.equal(await readFile(join(path, "output"), "utf8"), "disposable");
  await cli("retire", "--target", path, "--apply");
  assert.equal((await registry(actual)).targets[0].completed, true);
  await assert.rejects(access(join(path, "output")));
});

test("interrupted cache removal releases leases and a retry completes", async (t) => {
  const context = await fixture(t);
  const entry = (await snapshot(context))[0];
  for (let i = 0; i < 10; i++)
    await writeFile(join(entry.path, `output-${i}`), "disposable");
  const { targetLock } = await import("./build-storage-inventory.mjs");
  const code =
    'import importlib.util,sys,time\ns=importlib.util.spec_from_file_location("storage",sys.argv[1]);m=importlib.util.module_from_spec(s);s.loader.exec_module(m)\nu=m.os.unlink\ndef slow(p,*a,**k):\n u(p,*a,**k);print("removed",flush=True);time.sleep(0.3)\nm.os.unlink=slow\nm.clean(sys.argv[2],sys.argv[3],"all")';
  const child = spawn("python3", [
    helper,
    "lock",
    targetLock(context, entry.path),
    "exclusive",
    "python3",
    "-c",
    code,
    helper,
    entry.path,
    entry.manifest,
  ]);
  t.after(() => child.kill("SIGTERM"));
  const [data] = await once(child.stdout, "data");
  assert.match(String(data), /removed/);
  const exited = once(child, "exit");
  child.kill("SIGTERM");
  await exited;
  await cleanTarget(context, entry, "all");
  await assert.rejects(access(join(entry.path, "output-9")));
  await access(join(entry.path, "CACHEDIR.TAG"));
});

test("an oversized protected target does not evict useful idle caches when free space is healthy", async (t) => {
  const context = await fixture(t);
  await writeFile(
    join(context.root, "target/large"),
    Buffer.alloc(2 * 1024 ** 2),
  );
  await mkdir(join(context.root, "desktop/target"), { recursive: true });
  await writeFile(
    join(context.root, "desktop/Cargo.toml"),
    '[workspace]\n[package]\nname="idle-cache"\nversion="0.1.0"\n',
  );
  const output = join(context.root, "desktop/target/useful");
  await writeFile(output, "useful idle cache");
  const result = await maintain(context, {
    preserve: [join(context.root, "target")],
    environment: {
      JJ_STORE_BUILD_TARGET_GIB: "0.001",
      JJ_STORE_BUILD_FREE_GIB: "0.001",
    },
    log: () => {},
  });
  assert.equal(result.blocked, true);
  assert.equal(result.actions.length, 0);
  assert.equal(await readFile(output, "utf8"), "useful idle cache");
});
