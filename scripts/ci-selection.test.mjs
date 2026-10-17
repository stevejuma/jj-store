import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  rmSync,
  renameSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import {
  packageGraph,
  workspaceGraph,
  sourceEdges,
  reverseClosure,
  selectedPackages,
  cohortPackages,
} from "./ci-packages.mjs";
import { impact, changedFiles, cacheWritable } from "./ci-plan.mjs";
const root = "/fixture";
function metadata(names, edges = []) {
  return {
    workspace_members: names,
    packages: names.map((name) => ({
      id: name,
      name,
      manifest_path: `${root}/crates/${name}/Cargo.toml`,
      dependencies: edges
        .filter((e) => e.from === name)
        .map((e) => ({ name: e.to, path: `${root}/crates/${e.to}`, ...e })),
    })),
  };
}
const graph = () =>
  packageGraph(
    metadata(
      ["core", "leaf", "consumer", "jj-store"],
      [
        { from: "leaf", to: "core" },
        { from: "consumer", to: "leaf" },
        { from: "jj-store", to: "consumer" },
      ],
    ),
    root,
  );
test("affected packages include reverse transitive normal build dev optional and target edges", () => {
  const names = [
    "core",
    "normal",
    "dev",
    "build",
    "optional",
    "target",
    "renamed",
  ];
  const edges = names.slice(1).map((from) => ({
    from,
    to: "core",
    kind: from === "dev" || from === "build" ? from : null,
    optional: from === "optional",
    target: from === "target" ? "cfg(windows)" : null,
    rename: from === "renamed" ? "alias" : null,
  }));
  const g = packageGraph(metadata(names, edges), root);
  assert.deepEqual(reverseClosure(g, ["core"]), names.sort());
  const selected = impact(["crates/leaf/src/lib.rs"], false, graph());
  assert.deepEqual(selected.packages, ["consumer", "leaf"]);
  assert.deepEqual(selected.excluded, ["jj-store"]);
});
test("shared inputs and unknown ownership conservatively select all standard consumers", () => {
  for (const file of [
    "Cargo.toml",
    "Cargo.lock",
    "crates/leaf/Cargo.toml",
    "crates/leaf/build.rs",
    ".github/workflows/ci.yml",
    "rust-toolchain.toml",
    "scripts/new.mjs",
    "proto/api.proto",
    "fixtures/input.json",
    "unknown/data",
  ]) {
    const plan = impact([file], false, graph());
    assert.deepEqual(plan.packages, ["consumer", "core", "leaf"], file);
    assert.equal(plan.web, true, file);
  }
  for (const file of [
    "crates/leaf/tests/fixtures/input.json",
    "crates/core/proto/api.proto",
  ])
    assert.equal(impact([file], false, graph()).packages.length, 3);
});
test("empty and documentation selections stay light while force runs all standard checks", () => {
  for (const files of [
    [],
    ["spec/src/current/index.md"],
    ["README.md"],
    [".changeset/note.md"],
  ]) {
    const plan = impact(files, false, graph());
    assert.equal(plan.rust, false);
    assert.equal(plan.web, false);
    assert.deepEqual(plan.packages, []);
    assert.equal(impact(files, true, graph()).packages.length, 3);
    assert.equal(impact(files, true, graph()).web, true);
  }
  assert.equal(impact(["web/src/new.ts"], false, graph()).web, true);
  for (const source of [
    "packages/jj-conversations/src/client.ts",
    "packages/jj-history/src/index.ts",
  ]) {
    const client = impact([source], false, graph());
    assert.equal(client.web, true);
    assert.equal(client.rust, false);
  }
});
test("real read-index and model impacts preserve the agreed standard cohort", () => {
  const g = workspaceGraph();
  const inputs = sourceEdges(g);
  assert.deepEqual(inputs.uncertain, []);
  assert.deepEqual(cohortPackages(g, "ordinary"), [
    "jj-agent",
    "jj-collab",
    "jj-forge",
    "jj-review",
    "jj-store-adapters",
    "jj-store-conversations",
    "jj-store-core",
    "jj-store-git",
    "jj-store-http",
    "jj-store-jj",
    "jj-store-model",
    "jj-store-native",
    "jj-store-object",
    "jj-store-read-index",
    "jj-store-replication",
    "jj-store-rpc",
    "jj-store-service",
    "jj-store-tantivy",
    "jj-store-wal",
    "jj-work",
    "jj-workflow",
    "tauri-plugin-jj-store",
  ]);
  assert.ok(cohortPackages(g, "ordinary").includes("jj-forge"));
  assert.deepEqual(
    impact(["crates/jj-store-read-index/src/lib.rs"], false, g, inputs)
      .packages,
    [
      "jj-store-http",
      "jj-store-read-index",
      "jj-store-rpc",
      "jj-store-service",
      "jj-store-tantivy",
    ],
  );
  const core = impact(["crates/jj-store-model/src/lib.rs"], false, g, inputs);
  assert.equal(core.packages.length, 19);
  assert.ok(core.packages.includes("tauri-plugin-jj-store"));
  assert.equal(core.web, true);
});
test("invalid metadata and invalid or partial cohort requests fail closed", () => {
  for (const data of [
    {},
    { workspace_members: [], packages: [] },
    { workspace_members: ["missing"], packages: [] },
    metadata(["same", "same"]),
    metadata(["leaf"], [{ from: "leaf", to: "missing" }]),
  ])
    assert.throws(() => packageGraph(data, root));
  for (const list of [[], ["leaf", "leaf"], ["missing"], ["jj-store"]])
    assert.throws(() =>
      selectedPackages("ordinary", JSON.stringify(list), graph()),
    );
  assert.deepEqual(selectedPackages("ordinary", '["leaf"]', graph()), ["leaf"]);
  const g = packageGraph(
    metadata(["leaf", "jj-store", "jj-store-server"]),
    root,
  );
  assert.throws(() => selectedPackages("heavy", '["jj-store"]', g));
});
function fixture(t) {
  const dir = mkdtempSync(join(tmpdir(), "ci-impact-"));
  t.after(() => rmSync(dir, { recursive: true, force: true }));
  return dir;
}
test("sibling source fixtures broaden reverse dependencies and dynamic includes select all", (t) => {
  const dir = fixture(t);
  const data = metadata(["core", "consumer"]);
  for (const p of data.packages) {
    p.manifest_path = p.manifest_path.replace(root, dir);
    mkdirSync(join(dir, "crates", p.name, "tests"), { recursive: true });
  }
  writeFileSync(
    join(dir, "crates/consumer/tests/test.rs"),
    '#[path = "../../core/tests/shared.rs"] mod support;\n',
  );
  let g = packageGraph(data, dir),
    inputs = sourceEdges(g, dir);
  assert.deepEqual(
    impact(["crates/core/tests/shared.rs"], false, g, inputs).packages,
    ["consumer", "core"],
  );
  writeFileSync(
    join(dir, "crates/consumer/tests/test.rs"),
    'include!(concat!(env!("OUT_DIR"), "/generated.rs"));',
  );
  g = packageGraph(data, dir);
  inputs = sourceEdges(g, dir);
  assert.equal(inputs.uncertain.length, 1);
  assert.equal(impact(["README.md"], false, g, inputs).packages.length, 2);
});
test("renames and deletions include both old and new owners", (t) => {
  const dir = fixture(t);
  const git = (...args) =>
    execFileSync("git", ["-c", "commit.gpgsign=false", ...args], {
      cwd: dir,
      encoding: "utf8",
    }).trim();
  git("init", "-q");
  git("config", "user.name", "Fixture");
  git("config", "user.email", "fixture@example.invalid");
  for (const name of ["core", "leaf"])
    mkdirSync(join(dir, "crates", name, "src"), { recursive: true });
  writeFileSync(join(dir, "crates/core/src/move.rs"), "original");
  writeFileSync(join(dir, "crates/leaf/src/deleted.rs"), "deleted");
  git("add", ".");
  git("commit", "-qm", "before");
  const base = git("rev-parse", "HEAD");
  renameSync(
    join(dir, "crates/core/src/move.rs"),
    join(dir, "crates/leaf/src/move.rs"),
  );
  rmSync(join(dir, "crates/leaf/src/deleted.rs"));
  git("add", ".");
  git("commit", "-qm", "after");
  const paths = changedFiles(base, git("rev-parse", "HEAD"), dir);
  assert.deepEqual(paths.sort(), [
    "crates/core/src/move.rs",
    "crates/leaf/src/deleted.rs",
    "crates/leaf/src/move.rs",
  ]);
  assert.equal(impact(paths, false, graph()).packages.length, 3);
});
test("cache writes stay PR scoped or require canonical main context and source", () => {
  const base = {
    repository: "stevejuma/jj-store",
    event: "pull_request",
    ref: "refs/pull/1/merge",
    sourceOnMain: false,
  };
  assert.equal(cacheWritable(base), true);
  for (const override of [
    { repository: "fork/jj-store" },
    { event: "pull_request_target" },
    { ref: "refs/heads/main" },
    { ref: "refs/pull/1/head" },
  ])
    assert.equal(cacheWritable({ ...base, ...override }), false);
  for (const event of ["push", "workflow_dispatch"]) {
    const main = { ...base, event, ref: "refs/heads/main", sourceOnMain: true };
    assert.equal(cacheWritable(main), true);
    assert.equal(cacheWritable({ ...main, sourceOnMain: false }), false);
    assert.equal(
      cacheWritable({ ...main, ref: "refs/heads/experiment" }),
      false,
    );
  }
});

test("Help and preview-tool dependencies select core UI acceptance", () => {
  const g = workspaceGraph();
  for (const path of [
    "docs/site/guides/browsing.md",
    "crates/jj-store-http/examples/browser_preview.rs",
    "crates/jj-store-cli/examples/docs_reference.rs",
    "crates/jj-store-server/src/lib.rs",
  ])
    assert.equal(impact([path], false, g).web, true, path);
});

test("only the server may consume the explicitly named sibling AI client crates", () => {
  const data = metadata(["jj-store-server"]);
  data.packages[0].dependencies = [
    { name: "lapis-ai-runtime", path: "/lapis-ai/crates/lapis-ai-runtime" },
  ];
  assert.doesNotThrow(() => packageGraph(data, root));
  data.packages[0].dependencies[0].path = "/untrusted/lapis-ai-runtime";
  assert.throws(() => packageGraph(data, root), /Unmodelled/);
  data.packages[0].dependencies[0] = {
    name: "lapis-compute",
    path: "/lapis-ai/crates/lapis-compute",
  };
  assert.throws(() => packageGraph(data, root), /Unmodelled/);
});

test("only capture may consume the exact standalone native redaction source", () => {
  const data = metadata(["jj-store-capture"]);
  data.packages[0].dependencies = [
    {
      name: "lapis-agent-redaction",
      path: "/agent-redaction/crates/lapis-agent-redaction",
    },
  ];
  assert.doesNotThrow(() => packageGraph(data, root));
  data.packages[0].dependencies[0].path = "/untrusted/lapis-agent-redaction";
  assert.throws(() => packageGraph(data, root), /Unmodelled/);
  data.packages[0].dependencies[0].path =
    "/agent-redaction/crates/lapis-agent-redaction";
  data.packages[0].name = "jj-store-model";
  assert.throws(() => packageGraph(data, root), /Unmodelled/);
});

test("only capture may consume the exact standalone acquisition source", () => {
  const data = metadata(["jj-store-capture"]);
  const dependency = {
    name: "lapis-ai-capture",
    path: "/lapis-ai/crates/lapis-ai-capture",
  };
  data.packages[0].dependencies = [dependency];
  assert.doesNotThrow(() => packageGraph(data, root));
  dependency.path = "/untrusted/lapis-ai-capture";
  assert.throws(() => packageGraph(data, root), /Unmodelled/);
  dependency.path = "/lapis-ai/crates/lapis-ai-capture";
  for (const name of ["jj-store-server", "jj-store-core", "jj-store-model"]) {
    data.packages[0].name = name;
    assert.throws(() => packageGraph(data, root), /Unmodelled/);
  }
});
