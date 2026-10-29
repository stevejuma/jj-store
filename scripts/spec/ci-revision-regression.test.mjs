import { classifySpecFirstChanges } from "@lapismd/spec-validator";
import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import {
  mkdtempSync,
  mkdirSync,
  rmSync,
  symlinkSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { run } from "../check-progress.mjs";
import { changeConfirmations } from "./change-confirmations.mjs";
import { writeEvidence } from "./evidence-new.mjs";
import { revisionReader } from "./revision-reader.mjs";

function fixture(t) {
  const directory = mkdtempSync(join(tmpdir(), "spec-ci-revision-"));
  const root = join(directory, "jj"),
    gitRoot = join(directory, "git");
  mkdirSync(root);
  mkdirSync(gitRoot);
  const saved = Object.fromEntries(
    ["JJ_STORE_CHECK_BASE", "JJ_STORE_CHECK_REVISION"].map((key) => [
      key,
      process.env[key],
    ]),
  );
  t.after(() => {
    for (const [key, value] of Object.entries(saved)) {
      if (value === undefined) delete process.env[key];
      else process.env[key] = value;
    }
    rmSync(directory, { recursive: true, force: true });
  });
  const jj = (...args) =>
    execFileSync("jj", ["--no-pager", ...args], {
      cwd: root,
      encoding: "utf8",
      stdio: ["ignore", "pipe", "pipe"],
    }).trim();
  const write = (path, content) => {
    mkdirSync(dirname(join(root, path)), { recursive: true });
    writeFileSync(join(root, path), content);
  };
  const commit = (message) => {
    jj("commit", "-m", message);
    return jj("log", "-r", "@-", "--no-graph", "-T", "commit_id");
  };
  const evidence = (id, files) => ({
    id,
    files,
    tasks: ["Bootstrap"],
    requirements: ["JST-GOV-001"],
    date: "2000-01-01",
    kind: "governance",
    tests: [],
    runs: [{ command: "fixture proof", result: "pass", failures: 0 }],
    result: "pass",
    remaining: [],
    versions: "fixture",
    formats: "none",
    contract_review: {
      status: "unchanged",
      reason: "Fixture contract retained.",
    },
    milestone_review: {
      status: "unchanged",
      reason: "Fixture remains planned.",
    },
  });
  jj("git", "init", "--colocate");
  symlinkSync(join(root, ".git"), join(gitRoot, ".git"));
  write(
    "spec/src/current/governance/requirements.md",
    "| JST-GOV-001 | rule |\n",
  );
  write(
    "spec/src/current/progress/Bootstrap/index.md",
    "| Bootstrap | Planned | - | JST-GOV-001 | proof | qualify | - |\n",
  );
  const archive = "```json\n[]\n```\n";
  write("spec/archive/baseline.md", archive);
  write(
    "spec/archive/sources.json",
    JSON.stringify({
      "spec/src/milestones.md": {
        archive: "spec/archive/baseline.md",
        sha256: createHash("sha256").update(archive).digest("hex"),
      },
    }),
  );
  write("spec/archive/evidence-order.json", "[]");
  const base = commit("baseline");
  process.env.JJ_STORE_CHECK_BASE = base;
  const select = (head) => {
    process.env.JJ_STORE_CHECK_REVISION = head;
  };
  return { root, gitRoot, jj, write, commit, evidence, base, select };
}

test("CI range does not replace direct-parent progress evidence; explicit ranges stay complete", (t) => {
  const f = fixture(t);
  f.write("scripts/earlier.mjs", "earlier\n");
  writeEvidence(f.evidence("E-TEST-001", ["scripts/earlier.mjs"]), f.root);
  const earlier = f.commit("earlier qualified change");
  f.write("scripts/latest.mjs", "selected\n");
  writeEvidence(f.evidence("E-TEST-002", ["scripts/latest.mjs"]), f.root);
  const head = f.commit("latest qualified change");
  f.select(head);

  const native = revisionReader(f.root, head);
  const git = revisionReader(f.gitRoot, head);
  assert.deepEqual(git.parents, [earlier]);
  assert.deepEqual(git.changed.sort(), native.changed.sort());
  assert.ok(!git.changed.includes("scripts/earlier.mjs"));
  assert.deepEqual(revisionReader(f.gitRoot).changed.sort(), git.changed);
  for (const root of [f.root, f.gitRoot]) {
    assert.doesNotThrow(() => run(root, head));
    const range = revisionReader(root, head, { base: f.base });
    assert.ok(range.changed.includes("scripts/earlier.mjs"));
    assert.ok(range.changed.includes("scripts/latest.mjs"));
    assert.equal(changeConfirmations(root, { base: f.base, head }).length, 2);
  }

  f.write("scripts/latest.mjs", "later working edit\n");
  assert.equal(git.read("scripts/latest.mjs"), "selected\n");
  git.verify();
  f.write("scripts/unqualified.mjs", "missing evidence\n");
  const unqualified = f.commit("unqualified change");
  f.select(unqualified);
  for (const root of [f.root, f.gitRoot]) {
    assert.throws(
      () => run(root, unqualified),
      /missing fresh evidence coverage/,
    );
    assert.throws(
      () => changeConfirmations(root, { base: f.base, head: unqualified }),
      /scripts\/unqualified.mjs: missing fresh evidence coverage/,
    );
  }
  f.write("scripts/final.mjs", "qualified tip after missing evidence\n");
  writeEvidence(f.evidence("E-TEST-003", ["scripts/final.mjs"]), f.root);
  const final = f.commit(
    "qualified tip does not hide an unqualified PR change",
  );
  f.select(final);
  for (const root of [f.root, f.gitRoot]) {
    assert.doesNotThrow(() => run(root, final));
    assert.throws(
      () => changeConfirmations(root, { base: f.base, head: final }),
      /scripts\/unqualified.mjs: missing fresh evidence coverage/,
    );
  }
});

test("Git CI progress retains every merge parent and rejects reused or edited evidence", (t) => {
  const f = fixture(t);
  f.write("scripts/left.mjs", "left\n");
  writeEvidence(f.evidence("E-TEST-001", ["scripts/left.mjs"]), f.root);
  const left = f.commit("left qualified change");
  f.jj("new", f.base, "-m", "right branch");
  f.write("scripts/right.mjs", "right\n");
  const rightEvidence = f.evidence("E-TEST-002", ["scripts/right.mjs"]);
  writeEvidence(rightEvidence, f.root);
  const right = f.commit("right qualified change");
  f.jj("new", left, right, "-m", "merge integration");
  const merge = f.jj("log", "-r", "@", "--no-graph", "-T", "commit_id");
  f.select(merge);
  const reader = revisionReader(f.gitRoot, merge);
  assert.deepEqual(new Set(reader.parents), new Set([left, right]));
  assert.ok(reader.changed.includes("scripts/left.mjs"));
  assert.ok(reader.changed.includes("scripts/right.mjs"));
  for (const root of [f.root, f.gitRoot])
    assert.throws(() => run(root, merge), /missing fresh evidence coverage/);
  writeEvidence(
    f.evidence("E-TEST-003", ["scripts/left.mjs", "scripts/right.mjs"]),
    f.root,
  );
  const qualified = f.commit("qualified merge");
  f.select(qualified);
  for (const root of [f.root, f.gitRoot])
    assert.doesNotThrow(() => run(root, qualified));
  writeEvidence({ ...rightEvidence, formats: "rewritten history" }, f.root);
  const edited = f.commit("invalid historical edit");
  f.select(edited);
  for (const root of [f.root, f.gitRoot])
    assert.throws(() => run(root, edited), /committed evidence is immutable/);
});

test("unchanged confirmations cover mapped canonical requirement references only", (t) => {
  const f = fixture(t);
  const owner = "spec/src/current/governance/details.md";
  const decoy = "spec/src/current/governance/unrelated.md";
  f.write(owner, "JST-GOV-001 retains its detailed contract.\n");
  f.write(decoy, "JST-GOV-0010 is a different identifier.\n");
  f.write("spec/src/current/governance/index.md", "JST-GOV-001 navigation.\n");
  f.write("docs/reference.md", "JST-GOV-001 background only.\n");
  f.commit("canonical detail chapters");
  f.write("scripts/selected.mjs", "selected\n");
  writeEvidence(f.evidence("E-TEST-001", ["scripts/selected.mjs"]), f.root);
  const head = f.commit("qualified implementation without a contract change");
  for (const root of [f.root, f.gitRoot]) {
    const confirmations = changeConfirmations(root, { head });
    assert.deepEqual(
      new Set(confirmations[0].chapters),
      new Set(["spec/src/current/governance/requirements.md", owner]),
    );
    const changed = revisionReader(root, head).changed;
    const classify = (chapter) =>
      classifySpecFirstChanges(
        changed,
        {
          rules: [{ pattern: "^scripts/", chapters: [chapter] }],
        },
        confirmations,
      );
    assert.equal(classify(owner).ok, true);
    assert.equal(classify(decoy).ok, false);
    assert.equal(classify("docs/reference.md").ok, false);
  }
});
