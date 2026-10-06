import test from "node:test";
import assert from "node:assert/strict";
import {
  existsSync,
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  rmSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { execFileSync } from "node:child_process";
import {
  changesFromVcs,
  classifySpecFirstChanges,
} from "@lapismd/spec-validator";
function fixture() {
  const root = mkdtempSync(join(tmpdir(), "spec-native-jj-"));
  const jj = (...args) =>
    execFileSync("jj", ["--no-pager", ...args], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    }).trim();
  jj("git", "init", "--no-colocate");
  assert.equal(existsSync(join(root, ".git")), false);
  const write = (path, content) => {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  };
  const revision = (selection) =>
    jj("log", "-r", selection, "--no-graph", "-T", "commit_id");
  return {
    root,
    jj,
    write,
    revision,
    cleanup: () => rmSync(root, { recursive: true, force: true }),
  };
}
test("registry_validator_checks_explicit_native_jj_revision_without_reading_later_edits", () => {
  const f = fixture();
  try {
    f.write("web/view.ts", "baseline\n");
    f.jj("commit", "-m", "baseline");
    const base = f.revision("@-");
    f.write("web/view.ts", "selected revision\n");
    f.jj("commit", "-m", "selected");
    const head = f.revision("@-");
    f.write("web/view.ts", "later working edit\n");
    const changes = changesFromVcs({ base, head }, f.root);
    assert.deepEqual(
      changes.map((change) => change.path),
      ["web/view.ts"],
    );
    assert.ok(changes[0].changedLines.includes("selected revision"));
    assert.ok(!changes[0].changedLines.includes("later working edit"));
  } finally {
    f.cleanup();
  }
});
test("registry_validator_audits_both_native_jj_merge_parents", () => {
  const f = fixture();
  try {
    f.write("web/left.ts", "baseline\n");
    f.write("web/right.ts", "baseline\n");
    f.jj("commit", "-m", "baseline");
    const base = f.revision("@-");
    f.write("web/left.ts", "left\n");
    f.write("spec/current/contracts/ui/left.md", "# Left contract\n");
    f.jj("commit", "-m", "left");
    const left = f.revision("@-");
    f.jj("new", base, "-m", "right branch");
    f.write("web/right.ts", "right\n");
    f.write("spec/current/contracts/ui/right.md", "# Right contract\n");
    f.jj("commit", "-m", "right");
    const right = f.revision("@-");
    f.jj("new", left, right, "-m", "merge acceptance");
    const changes = changesFromVcs({}, f.root);
    const paths = new Set(changes.map((change) => change.path));
    assert.ok(paths.has("web/left.ts"));
    assert.ok(paths.has("web/right.ts"));
    const options = {
      canonicalPattern: "^spec/current/contracts/.+\\.md$",
      protected: ["^web/"],
      rules: [
        {
          pattern: "^web/",
          chapterPatterns: ["^spec/current/contracts/ui/.+\\.md$"],
        },
      ],
    };
    assert.equal(classifySpecFirstChanges(changes, options).ok, true);
  } finally {
    f.cleanup();
  }
});
