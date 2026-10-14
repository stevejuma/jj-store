import test from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import { revisionReader } from "./spec/revision-reader.mjs";
import { impact } from "./ci-plan.mjs";
import { jjArtifact, verifyJj } from "./ci-setup-jj.mjs";
import { createHash } from "node:crypto";
test("CI pins JJ for each qualified platform and rejects corrupt bytes", () => {
  for (const platform of ["linux", "darwin"])
    for (const arch of ["arm64", "x64"]) {
      const artifact = jjArtifact(platform, arch);
      assert.match(artifact.name, /^jj-v0\.45\.1-.+\.tar\.gz$/);
      assert.match(artifact.digest, /^[a-f0-9]{64}$/);
    }
  assert.throws(() => jjArtifact("win32", "x64"), /Unsupported/);
  const bytes = Buffer.from("verified fixture");
  verifyJj(bytes, createHash("sha256").update(bytes).digest("hex"));
  assert.throws(
    () =>
      verifyJj(
        Buffer.from("corrupt fixture"),
        createHash("sha256").update(bytes).digest("hex"),
      ),
    /checksum/,
  );
});
test("docs-only impact stays light; shared and service changes select heavy lanes", () => {
  assert.deepEqual(impact(["spec/src/current/index.md"]), {
    rust: false,
    web: false,
    heavy: false,
    full: false,
  });
  assert.equal(impact(["crates/jj-store-service/src/lib.rs"]).heavy, true);
  assert.equal(impact(["Cargo.lock"]).full, true);
});
test("Git revision reader pins content and all merge parents", () => {
  const root = mkdtempSync(join(tmpdir(), "jj-store-ci-git-"));
  const git = (...args) =>
    execFileSync("git", args, { cwd: root, encoding: "utf8" }).trim();
  try {
    git("init", "-q");
    git("config", "user.name", "Fixture");
    git("config", "user.email", "fixture@example.invalid");
    writeFileSync(join(root, "record.md"), "original\n");
    git("add", "record.md");
    git("commit", "-qm", "baseline");
    const first = git("rev-parse", "HEAD");
    writeFileSync(join(root, "record.md"), "selected\n");
    git("commit", "-qam", "selected");
    const reader = revisionReader(root, first);
    writeFileSync(join(root, "record.md"), "uncommitted\n");
    assert.equal(reader.read("record.md"), "original\n");
    reader.verify();
    const empty = git("hash-object", "-t", "tree", "--stdin");
    assert.deepEqual(revisionReader(root, empty).files, []);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
