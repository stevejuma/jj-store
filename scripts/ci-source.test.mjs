import test from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { ensurePinnedSource } from "./ci-pinned-source.mjs";
import { validateRedactionSourcePin } from "./ci-redaction-source.mjs";

const pin = {
  repository: "lapismd/agent-redaction",
  version: "0.1.0",
  revision: "a".repeat(40),
};
test("native privacy source pins reject mutable revisions and other owners", async () => {
  validateRedactionSourcePin(
    JSON.parse(
      await readFile(
        new URL("./agent-redaction-source.json", import.meta.url),
        "utf8",
      ),
    ),
  );
  assert.equal(validateRedactionSourcePin(pin), pin);
  for (const change of [
    { revision: "main" },
    { repository: "other/source" },
    { version: "next" },
  ])
    assert.throws(
      () => validateRedactionSourcePin({ ...pin, ...change }),
      /Invalid/,
    );
});
test("source materialization preserves local checkouts and fences CI replacements", async (t) => {
  const directory = await mkdtemp(join(tmpdir(), "jj-store-source-"));
  t.after(() => rm(directory, { recursive: true, force: true }));
  const root = join(directory, "consumer");
  const source = join(directory, "agent-redaction");
  await mkdir(root);
  const options = {
    root,
    name: "agent-redaction",
    packageName: "@lapismd/agent-redaction",
    environment: {},
  };
  await assert.rejects(ensurePinnedSource(pin, options), /Create the sibling/);
  await mkdir(source);
  const retained = join(source, "retained.txt");
  await writeFile(retained, "source stays intact");
  assert.equal(await ensurePinnedSource(pin, options), source);
  options.environment = { CI: "true" };
  await assert.rejects(ensurePinnedSource(pin, options), /Refusing to replace/);
  const marker = join(source, ".jj-store-source.json");
  await writeFile(marker, JSON.stringify(pin));
  assert.equal(await ensurePinnedSource(pin, options), source);
  await assert.rejects(
    ensurePinnedSource({ ...pin, revision: "b".repeat(40) }, options),
    /Refusing to replace/,
  );
  assert.equal(await readFile(retained, "utf8"), "source stays intact");
});
