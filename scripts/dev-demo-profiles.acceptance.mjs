#!/usr/bin/env node
import assert from "node:assert/strict";
import { readFile, realpath, stat } from "node:fs/promises";
import { join, resolve } from "node:path";
import { execute } from "./dev-demo-services.mjs";
import { sampleWork } from "./dev-demo-work-samples.mjs";
const root = resolve(import.meta.dirname, "..");
let runtimeRoot = process.env.JJ_STORE_DEV_ROOT;
assert.ok(
  runtimeRoot,
  "acceptance requires an explicit isolated development root",
);
runtimeRoot = await realpath(runtimeRoot);
const runtime = JSON.parse(
  await readFile(join(runtimeRoot, "runtime.json"), "utf8"),
);
const profiles = runtime.repositoryProfiles;
assert.equal(new Set(profiles.repositoryLinks).size, 3);
const headers = {
  Origin: new URL(runtime.origin).origin,
  "X-JJ-Store-Protocol": "1",
  "Sec-Fetch-Dest": "empty",
  "Sec-Fetch-Mode": "cors",
  "Sec-Fetch-Site": "same-origin",
};
const get = async (path) => {
  const response = await fetch(`${runtime.origin}v1/${path}`, {
    headers,
    signal: AbortSignal.timeout(30000),
  });
  assert.equal(response.status, 200, await response.clone().text());
  return response.json();
};
const directory = await get("repositories");
assert.ok(
  directory.repositories.some((repo) => repo.id === profiles.nativeLocal),
);
assert.ok(
  directory.repositories.some((repo) => repo.id === profiles.nativeServer),
);
const gitEntry = directory.repositories.find(
  (repo) => repo.id === profiles.localGit,
);
assert.equal(
  gitEntry?.profile,
  "git_sha1_jj045",
  "Git fixture retains its own authority",
);
assert.equal(gitEntry?.availability, "ready");
const gitSnapshot = await get(`local/repos/${profiles.localGit}/snapshot`);
assert.equal(gitSnapshot.profile, "git_sha1_jj045");
assert.equal(gitSnapshot.heads.length, 1);
const work = await get(`repos/${profiles.nativeLocal}/work?limit=256`);
const samples = sampleWork(profiles.nativeLocal, profiles.repositoryLinks);
// Restart qualification preserves the seeded records alongside later user/test work.
const sampleIds = new Set(samples.map((sample) => sample.id));
assert.equal(
  work.projects.filter((project) => sampleIds.has(project.id)).length,
  3,
);
for (const sample of samples) {
  const project = work.projects.find((project) => project.id === sample.id);
  assert.deepEqual(
    Object.entries(project.repositories)
      .filter(([, active]) => active)
      .map(([id]) => id),
    [sample.repository],
  );
  const page = await get(
    `repos/${profiles.nativeLocal}/work?project=${sample.id}&limit=256`,
  );
  assert.equal(page.tasks.length, sample.tasks.length);
  for (const task of page.tasks)
    assert.ok(
      task.repository === null || task.repository === sample.repository,
    );
}
const local = await get(`repos/${profiles.nativeLocal}/local/workspaces`);
assert.ok(
  local.links.some(
    (link) =>
      resolve(link.path) === resolve(runtimeRoot, "repository") &&
      link.profile === "native",
  ),
);
assert.deepEqual(
  (await get(`repos/${profiles.nativeServer}/local/workspaces`)).links,
  [],
);
await assert.rejects(stat(join(runtimeRoot, "server-only-seed")), {
  code: "ENOENT",
});
const server = await get(`repos/${profiles.nativeServer}/snapshot`);
assert.equal(server.heads.length, 1);
await get(`repos/${profiles.nativeServer}/references?at_op=${server.heads[0]}`);
assert.equal(
  (
    await readFile(join(profiles.localFolder, ".jj/repo/store/type"), "utf8")
  ).trim(),
  "git",
);
const diagnostic = JSON.parse(
  (
    await execute(
      join(root, "target/debug/jj-store-jj"),
      ["-R", profiles.localFolder, "store", "doctor", "--format=json"],
      { env: { JJ_CONFIG: join(runtimeRoot, "jj.toml") } },
    )
  ).stdout,
);
assert.equal(diagnostic.repository, profiles.localGit);
assert.equal(diagnostic.server_required, false);
console.log(
  JSON.stringify(
    {
      profiles,
      homeSnapshot: work.snapshot,
      serverSnapshot: server,
      projects: 3,
      tasks: 9,
      checks:
        "mixed directory and snapshots, native references, distinct links/targets, local folder binding, server-only cleanup and Git companion identity",
    },
    null,
    2,
  ),
);
