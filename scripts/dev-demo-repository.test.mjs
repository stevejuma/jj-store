import assert from "node:assert/strict";
import { mkdtemp, readFile, realpath, rm, symlink } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";

import { seedLinearHistory } from "./dev-demo-repository.mjs";

test("demo repository seeds every supported landing document", async (t) => {
  const repository = await mkdtemp(join(tmpdir(), "jj-store-demo-seed-"));
  t.after(() => rm(repository, { recursive: true, force: true }));

  await seedLinearHistory(repository, async () => ({ stdout: "" }));

  const documents = await Promise.all(
    [
      "README.md",
      "CODE_OF_CONDUCT.md",
      "CONTRIBUTING.md",
      "LICENSE",
      "SECURITY.md",
    ].map(async (name) => [
      name,
      await readFile(join(repository, name), "utf8"),
    ]),
  );
  assert.deepEqual(
    documents.map(([name]) => name),
    [
      "README.md",
      "CODE_OF_CONDUCT.md",
      "CONTRIBUTING.md",
      "LICENSE",
      "SECURITY.md",
    ],
  );
  for (const [name, contents] of documents) {
    assert.ok(contents.trim(), `${name} should contain representative text`);
  }
});

test("local demo admission reuses canonical fixture binding and registration receipt", async (t) => {
  const { registerDemoLocalFolder } =
    await import("./dev-demo-local-folder.mjs");
  const repositoryId = "11".repeat(16);
  const folder = await mkdtemp(join(tmpdir(), "jj-store-demo-binding-"));
  t.after(() => rm(folder, { recursive: true, force: true }));
  const alias = `${folder}-alias`;
  await symlink(folder, alias);
  t.after(() => rm(alias, { force: true }));
  const calls = [];
  const binding = {
    repository: repositoryId,
    path: await realpath(folder),
    availability: "ready",
  };
  const result = await registerDemoLocalFolder({
    origin: "http://127.0.0.1:7499/",
    repositoryId,
    path: alias,
    fetcher: async (url, init) => {
      calls.push([url, init]);
      return new Response(JSON.stringify({ revision: 2, bindings: [binding] }));
    },
  });
  assert.deepEqual(result, binding);
  assert.equal(calls.length, 1);
  assert.equal(calls[0][1].method, "GET");
});
