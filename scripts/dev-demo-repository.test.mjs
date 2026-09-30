import assert from "node:assert/strict";
import { mkdtemp, readFile, rm } from "node:fs/promises";
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
