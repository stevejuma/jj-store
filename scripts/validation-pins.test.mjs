import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, mkdir, writeFile, readFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import {
  exportPinnedSources,
  restorePinnedSources,
} from "./validation/pins.mjs";
test("pinned sibling transfer excludes dirty source and validates restored identities", async () => {
  const directory = await mkdtemp(join(tmpdir(), "jj-pinned-transfer-"));
  const original = join(directory, "original"),
    restored = join(directory, "restored");
  const jj = (cwd, args) =>
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
    await mkdir(original);
    await mkdir(restored);
    const root = join(original, "jj-store"),
      destination = join(restored, "jj-store"),
      input = join(directory, "input");
    jj(original, ["git", "init", "--colocate", root]);
    await mkdir(join(root, "scripts"));
    await mkdir(join(destination, "scripts"), { recursive: true });
    await mkdir(input);
    for (const [name, version] of [
      ["lapis-ai", "0.1.1"],
      ["agent-redaction", "0.1.0"],
    ]) {
      const sibling = join(original, name);
      jj(original, ["git", "init", "--colocate", sibling]);
      await writeFile(
        join(sibling, "package.json"),
        JSON.stringify({ name: `@lapismd/${name}`, version }),
      );
      const revision = jj(sibling, [
        "log",
        "-r",
        "@",
        "--no-graph",
        "-T",
        "commit_id",
      ]);
      const pin = JSON.stringify({
        repository: `lapismd/${name}`,
        revision,
        version,
      });
      await writeFile(join(root, `scripts/${name}-source.json`), pin);
      await writeFile(join(destination, `scripts/${name}-source.json`), pin);
      await writeFile(
        join(sibling, "package.json"),
        JSON.stringify({ name: `@lapismd/${name}`, version: "9.9.9" }),
      );
    }
    const revision = jj(root, [
      "log",
      "-r",
      "@",
      "--no-graph",
      "-T",
      "commit_id",
    ]);
    const entries = await exportPinnedSources(root, input, revision);
    await restorePinnedSources(destination, input, entries);
    assert.equal(
      JSON.parse(await readFile(join(restored, "lapis-ai/package.json")))
        .version,
      "0.1.1",
    );
    assert.equal(
      JSON.parse(await readFile(join(restored, "agent-redaction/package.json")))
        .version,
      "0.1.0",
    );
    await writeFile(join(input, entries[0].path), "tampered archive");
    await assert.rejects(
      restorePinnedSources(destination, input, entries),
      /checksum/,
    );
  } finally {
    await rm(directory, { recursive: true, force: true });
  }
});
