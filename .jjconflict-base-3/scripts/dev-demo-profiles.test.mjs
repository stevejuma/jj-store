import assert from "node:assert/strict";
import {
  mkdtemp,
  mkdir,
  writeFile,
  readFile,
  rm,
  stat,
} from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { seedRepositoryProfiles } from "./dev-demo-profiles.mjs";
test("profiles seed real native and Git backends, remove server checkout, and reuse identities", async (t) => {
  const runtimeRoot = await mkdtemp(join(tmpdir(), "jj-store-profiles-"));
  t.after(() => rm(runtimeRoot, { recursive: true, force: true }));
  const calls = [];
  const run = async (_, args) => {
    calls.push(args);
    if (args[0] === "store") {
      await mkdir(args[2], { recursive: true });
      return { stdout: JSON.stringify({ repository: "22".repeat(16) }) };
    }
    if (args[0] === "git") {
      await mkdir(join(args.at(-1), ".jj/repo/store"), { recursive: true });
      await writeFile(join(args.at(-1), ".jj/repo/store/type"), "git");
      return { stdout: "" };
    }
    if (args.includes("doctor"))
      return {
        stdout: JSON.stringify({
          profile: "local_jj_git",
          repository: "33".repeat(16),
          metadata_initialized: true,
        }),
      };
    if (args.includes("initialize"))
      return { stdout: JSON.stringify({ status: "committed" }) };
    if (args.includes("list"))
      return { stdout: args.includes("workspace") ? "seed" : "" };
    return { stdout: "" };
  };
  const options = {
    repositoryRoot: "/source",
    runtimeRoot,
    repositoryId: "11".repeat(16),
    unixSocket: "/socket",
    run,
  };
  const result = await seedRepositoryProfiles(options);
  assert.deepEqual(result.repositoryLinks, [
    "11".repeat(16),
    "22".repeat(16),
    "33".repeat(16),
  ]);
  await assert.rejects(stat(join(runtimeRoot, "server-only-seed/repository")), {
    code: "ENOENT",
  });
  assert.equal(
    (
      await readFile(
        join(runtimeRoot, "git-repository/.jj/repo/store/type"),
        "utf8",
      )
    ).trim(),
    "git",
  );
  assert.ok(calls.some((args) => args.includes("forget")));
  const count = calls.length;
  assert.deepEqual(await seedRepositoryProfiles(options), result);
  assert.equal(
    calls.length,
    count + 1,
    "completed profiles never reseed or replace user edits",
  );
});
