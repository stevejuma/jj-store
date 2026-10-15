import test from "node:test";
import assert from "node:assert/strict";
import {
  existsSync,
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  rmSync,
  symlinkSync,
  readFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import { revisionReader } from "./spec/revision-reader.mjs";
import { impact } from "./ci-plan.mjs";
import { jjArtifact, verifyJj, jjArchiveMember } from "./ci-setup-jj.mjs";
import { createHash } from "node:crypto";
import { prebuiltBrowserResources } from "./ci-prebuilt.mjs";
import { acpArtifact, verifyAcp } from "./ci-setup-acp.mjs";
import { parse as parseYaml } from "yaml";
function coldHarness(t) {
  const root = mkdtempSync(join(tmpdir(), "jj-store-ci-selection-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  mkdirSync(join(root, "scripts"));
  mkdirSync(join(root, "bin"));
  const script = join(root, "scripts/real-browser-acceptance.mjs");
  writeFileSync(script, readFileSync("scripts/real-browser-acceptance.mjs"));
  writeFileSync(
    join(root, "scripts/profile-acceptance.mjs"),
    `
import {mkdtempSync,rmSync} from 'node:fs';import {tmpdir} from 'node:os';import {join} from 'node:path';import {execFileSync} from 'node:child_process';
const runtime=mkdtempSync(join(tmpdir(),'jj-store-ci-profile-owner-'));
try {execFileSync('pnpm',['exec','playwright','test','profiles-emoji.spec.ts'],{env:{...process.env,JJ_STORE_DEV_ROOT:runtime,JJ_STORE_REAL_RUNTIME:runtime,JJ_STORE_REAL_SKIP_PROFILE:'0'}});} finally {rmSync(runtime,{recursive:true,force:true});}
`,
  );
  const log = join(root, "calls.jsonl");
  writeFileSync(
    join(root, "bin/pnpm"),
    `#!/usr/bin/env node
const fs = require('node:fs');
const args = process.argv.slice(2);
fs.appendFileSync(process.env.SELECTION_LOG, JSON.stringify({args, root:process.env.JJ_STORE_DEV_ROOT, runtime:process.env.JJ_STORE_REAL_RUNTIME, skipProfile:process.env.JJ_STORE_REAL_SKIP_PROFILE})+'\\n');
if(args[0]==='dev:reset') {
 fs.writeFileSync(process.env.JJ_STORE_DEV_ROOT+'/runtime.json', JSON.stringify({format:2,accessMode:'local-trusted',storageMode:'embedded-rustfs',resources:{firstStartupMillis:1,restartStartupMillis:1,totalRssKiB:1,fixturePreparationMillis:1}}));
 console.log('URL: http://127.0.0.1:7530/\\nRepository ID: '+ 'ab'.repeat(16)+'\\nRuntime: '+process.env.JJ_STORE_DEV_ROOT);
} else if(args[0]==='exec' && process.env.SELECTION_FAIL==='1') process.exit(1);
`,
    { mode: 0o755 },
  );
  const env = {
    ...process.env,
    PATH: `${join(root, "bin")}:${process.env.PATH}`,
    SELECTION_LOG: log,
  };
  for (const key of [
    "JJ_STORE_REAL_TEST",
    "JJ_STORE_REAL_SHARD",
    "JJ_STORE_REAL_KEEP",
    "JJ_STORE_LOCAL_RELEASE_QUALIFY",
  ])
    delete env[key];
  return {
    run: (extra = {}) =>
      execFileSync(process.execPath, [script], {
        env: { ...env, ...extra },
        stdio: "pipe",
      }),
    calls: () => readFileSync(log, "utf8").trim().split("\n").map(JSON.parse),
  };
}
test("parallel cold shards use disjoint result directories and CI stops on failure", () => {
  const directories = ["1/4", "2/4", "3/4", "4/4"].map((shard) => {
    const output = execFileSync(
      process.execPath,
      [
        "--import",
        "tsx",
        "--input-type=module",
        "-e",
        'import config from "./web/playwright.real.config.ts"; console.log(JSON.stringify({output:config.outputDir,failures:config.maxFailures}));',
      ],
      {
        encoding: "utf8",
        env: {
          ...process.env,
          CI: "true",
          JJ_STORE_REAL_BASE_URL: "http://127.0.0.1:7530",
          JJ_STORE_REAL_SHARD: shard,
        },
      },
    );
    const config = JSON.parse(output);
    assert.equal(config.failures, 1);
    return config.output;
  });
  assert.equal(new Set(directories).size, 4);
});
test("heavy browser reporting keeps all tests and emits JUnit", () => {
  for (const file of [
    "playwright.config.ts",
    "playwright.real.config.ts",
    "playwright.docs.config.ts",
  ]) {
    const output = execFileSync(
      process.execPath,
      [
        "--import",
        "tsx",
        "--input-type=module",
        "-e",
        `import config from "./web/${file}"; console.log(JSON.stringify({failures:config.maxFailures,reporter:config.reporter}));`,
      ],
      {
        encoding: "utf8",
        env: {
          ...process.env,
          CI: "true",
          JJ_STORE_CI_REPORTS: "1",
          JJ_STORE_REAL_BASE_URL: "http://127.0.0.1:7530",
        },
      },
    );
    const config = JSON.parse(output);
    assert.equal(config.failures, 0);
    assert.deepEqual(config.reporter, [["line"], ["junit"]]);
  }
});
test("the complete cold suite isolates profile shutdown and cleans both fixtures", (t) => {
  const harness = coldHarness(t);
  harness.run();
  const calls = harness.calls(),
    suites = calls.filter((call) => call.args[0] === "exec");
  assert.equal(suites.length, 2);
  assert.ok(suites[0].args.includes("--shard=1/1"));
  assert.equal(suites[0].skipProfile, "1");
  assert.ok(suites[1].args.includes("profiles-emoji.spec.ts"));
  assert.equal(suites[1].skipProfile, "0");
  assert.notEqual(suites[0].runtime, suites[1].runtime);
  assert.equal(calls.filter((call) => call.args[0] === "dev:stop").length, 1);
  for (const suite of suites) assert.equal(existsSync(suite.runtime), false);
});
test("cold shard failures clean their fixture and do not launch the later profile suite", (t) => {
  const harness = coldHarness(t);
  assert.throws(() => harness.run({ SELECTION_FAIL: "1" }));
  const calls = harness.calls();
  assert.equal(calls.filter((call) => call.args[0] === "dev:reset").length, 1);
  assert.equal(calls.filter((call) => call.args[0] === "dev:stop").length, 1);
  assert.equal(existsSync(calls[0].root), false);
});
test("browser and storage suites preserve coverage and consume only their inputs", () => {
  const load = (file) =>
    parseYaml(readFileSync(`.github/workflows/${file}.yml`, "utf8")).jobs;
  const heavy = load("ci-heavy"),
    cold = load("ci-heavy-cold").tests,
    ui = load("ci-heavy-ui").tests;
  assert.ok(heavy.cold.needs.includes("runtime"));
  assert.ok(heavy.embedded.needs.includes("runtime"));
  assert.ok(heavy.complete.needs.includes("runtime"));
  assert.equal(heavy.ui, undefined); // UI is mandatory in standard checks.
  const lanes = [
    ...cold.strategy.matrix.include,
    ...ui.strategy.matrix.include,
  ];
  for (const [lane, shards] of [
    ["cold", ["1/4", "2/4", "3/4", "4/4"]],
    ["storybook", ["1/3", "2/3", "3/3"]],
    ["browser", ["1/2", "2/2"]],
  ])
    assert.deepEqual(
      lanes.filter((item) => item.lane === lane).map((item) => item.shard),
      shards,
    );
  for (const lane of ["cold-profile", "git-cycle", "catalog", "help"])
    assert.equal(lanes.filter((item) => item.lane === lane).length, 1);
  for (const job of [cold, ui]) {
    assert.equal(
      job.steps.some((step) => step.uses?.includes("rust-cache")),
      false,
    );
    assert.ok(
      job.steps.some(
        (step) =>
          step.run?.includes("ci-artifacts.mjs") && /consume/.test(step.run),
      ),
    );
    assert.ok(
      job.steps.some((step) => step.run?.endsWith("--with-deps chromium")),
    );
    const setup = job.steps.find(
      (step) => step.uses === "./.github/actions/setup",
    );
    assert.equal(setup.with["native-dependencies"], "false");
    assert.match(setup.with["build-tools"], /matrix\.lane == 'git-cycle'/);
  }
  assert.equal(
    ui.steps.find((step) => step.run?.endsWith("--with-deps webkit")).if,
    "matrix.lane == 'help'",
  );
  assert.equal(
    cold.steps.find((step) => step.run === "node scripts/ci-setup-acp.mjs").if,
    "matrix.lane == 'cold' || matrix.lane == 'cold-profile'",
  );
});
test("browser orchestration selects one suite per lane and forwards real shard arguments", (t) => {
  const root = mkdtempSync(join(tmpdir(), "jj-store-ci-browser-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  const log = join(root, "calls.jsonl");
  writeFileSync(
    join(root, "pnpm"),
    `#!/usr/bin/env node
require('node:fs').appendFileSync(process.env.BROWSER_CALL_LOG, JSON.stringify(process.argv.slice(2))+'\\n');
`,
    { mode: 0o755 },
  );
  const invoke = (lane, shard = "") => {
    writeFileSync(log, "");
    execFileSync(process.execPath, ["scripts/ci-browser.mjs", lane], {
      env: {
        ...process.env,
        CI: "true",
        JJ_STORE_CI_PREBUILT: "1",
        JJ_STORE_BROWSER_SHARD: shard,
        BROWSER_CALL_LOG: log,
        PATH: `${root}:${process.env.PATH}`,
      },
    });
    return readFileSync(log, "utf8").trim().split("\n").map(JSON.parse);
  };
  assert.deepEqual(invoke("storybook", "2/3"), [
    ["test:storybook", "--shard=2/3"],
  ]);
  assert.deepEqual(invoke("browser", "1/2"), [
    [
      "exec",
      "playwright",
      "test",
      "--config",
      "web/playwright.config.ts",
      "--shard=1/2",
    ],
  ]);
  assert.deepEqual(invoke("catalog"), [
    ["exec", "node", "--test", "scripts/catalog-browser.test.mjs"],
  ]);
  assert.deepEqual(invoke("help"), [
    ["exec", "playwright", "test", "--config", "web/playwright.docs.config.ts"],
  ]);
});
test("CI pins published ACP and rejects unsupported or corrupt artifacts", () => {
  for (const platform of ["linux", "darwin"])
    for (const arch of ["arm64", "x64"]) {
      const artifact = acpArtifact(platform, arch);
      assert.equal(
        artifact.name,
        artifact.member.replace("/jj-acp", ".tar.gz"),
      );
      assert.match(artifact.digest, /^[a-f0-9]{64}$/);
    }
  assert.throws(() => acpArtifact("win32", "x64"), /Unsupported/);
  const bytes = Buffer.from("verified ACP fixture");
  const digest = createHash("sha256").update(bytes).digest("hex");
  verifyAcp(bytes, digest);
  assert.throws(() => verifyAcp(Buffer.from("corrupt"), digest), /checksum/);
});
test("prebuilt CI validates complete resources before accepting the cold fixture", async () => {
  const root = mkdtempSync(join(tmpdir(), "jj-store-prebuilt-"));
  const env = {
    CI: "true",
    JJ_STORE_CI_PREBUILT: "1",
    JJ_STORE_DEV_RUSTFS_RUNTIME: "runtime/jj-store-rustfs",
  };
  const assetManifest = join(root, "web/dist/asset-manifest.json");
  const manifest = (files) =>
    writeFileSync(assetManifest, JSON.stringify({ version: 1, files }));
  try {
    assert.equal(await prebuiltBrowserResources(root, {}), false);
    await assert.rejects(
      prebuiltBrowserResources(root, { ...env, CI: "false" }),
      /explicit CI/,
    );
    await assert.rejects(
      prebuiltBrowserResources(root, {
        ...env,
        JJ_STORE_DEV_RUSTFS_RUNTIME: "",
      }),
      /explicit RustFS/,
    );
    await assert.rejects(prebuiltBrowserResources(root, env), /ENOENT/);
    for (const file of [
      "target/debug/jj-store",
      "target/debug/jj-store-jj",
      "runtime/jj-store-rustfs",
      "web/dist/help/index.html",
      "web/dist/index.html",
      "web/dist/assets/app.js",
    ]) {
      mkdirSync(join(root, file, ".."), { recursive: true });
      writeFileSync(join(root, file), "fixture", { mode: 0o755 });
    }
    writeFileSync(
      join(root, "web/dist/help/asset-manifest.json"),
      JSON.stringify({ version: 1, files: ["index.html"] }),
    );
    manifest(["index.html", "assets/app.js"]);
    assert.equal(await prebuiltBrowserResources(root, env), true);
    manifest(["index.html", "../outside.js"]);
    await assert.rejects(prebuiltBrowserResources(root, env), /Unsafe/);
    manifest(["index.html", "assets/missing.js"]);
    await assert.rejects(prebuiltBrowserResources(root, env), /ENOENT/);
    manifest(["index.html", "index.html"]);
    await assert.rejects(prebuiltBrowserResources(root, env), /Invalid/);
    manifest(["index.html", "assets/app.js"]);
    rmSync(join(root, "web/dist/assets/app.js"));
    symlinkSync(
      join(root, "web/dist/index.html"),
      join(root, "web/dist/assets/app.js"),
    );
    await assert.rejects(prebuiltBrowserResources(root, env), /symlink/);
    rmSync(join(root, "web/dist/assets/app.js"));
    writeFileSync(join(root, "web/dist/assets/app.js"), "fixture");
    rmSync(join(root, "web/dist/help/index.html"));
    await assert.rejects(prebuiltBrowserResources(root, env), /ENOENT/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
test("JJ extraction selects the exact safe archive member", () => {
  assert.equal(jjArchiveMember("./\n./jj\n./README.md\n"), "./jj");
  assert.equal(jjArchiveMember("jj\nREADME.md\n"), "jj");
  for (const members of ["../jj\n", "/jj\n", "bin/jj\n", "jj\n./jj\n", ""])
    assert.throws(() => jjArchiveMember(members), /one root executable/);
});
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
  const docs = impact(["spec/src/current/index.md"]);
  assert.equal(docs.rust, false);
  assert.equal(docs.web, false);
  assert.deepEqual(docs.packages, []);
  assert.equal(impact(["crates/jj-store-service/src/lib.rs"]).rust, true);
  assert.equal(impact(["Cargo.lock"]).web, true);
});
test("Git revision reader pins content and all merge parents", () => {
  const root = mkdtempSync(join(tmpdir(), "jj-store-ci-git-"));
  const git = (...args) =>
    execFileSync("git", ["-c", "commit.gpgsign=false", ...args], {
      cwd: root,
      encoding: "utf8",
    }).trim();
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

// Retain this historical evidence anchor: qualification is now manual-only.
test("failed early or ordinary lanes cannot start expensive qualification", () => {
  const standard = parseYaml(
    readFileSync(".github/workflows/ci.yml", "utf8"),
  ).jobs;
  const heavy = parseYaml(
    readFileSync(".github/workflows/ci-heavy.yml", "utf8"),
  ).jobs;
  for (const name of ["build", "web"])
    assert.ok(standard[name].needs.includes("fast"));
  assert.ok(heavy.build.needs.includes("standard"));
  assert.equal(heavy.build.if, undefined);
  assert.equal(standard.heavy, undefined);
});
