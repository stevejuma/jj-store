import test from "node:test";
import assert from "node:assert/strict";
import {
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
test("browser and storage lanes consume same-run assets without overwriting them with caches", () => {
  const { jobs } = parseYaml(
    readFileSync(".github/workflows/ci-heavy.yml", "utf8"),
  );
  const downloads = jobs.browser.steps.filter((step) =>
    step.uses?.startsWith("actions/download-artifact@"),
  );
  assert.deepEqual(downloads.map((step) => step.with.name).sort(), [
    "linux-development-tools",
    "linux-storage-runtime",
    "native-browser-fixtures",
    "storybook-bundle",
    "web-bundle",
  ]);
  assert.equal(
    jobs.browser.steps.some((step) =>
      step.uses?.startsWith("Swatinem/rust-cache@"),
    ),
    false,
  );
  assert.ok(jobs.browser.needs.includes("runtime"));
  assert.ok(jobs.storage.needs.includes("runtime"));
  assert.ok(jobs.complete.needs.includes("runtime"));
  const installer = jobs.browser.steps.find(
    (step) => step.run === "node scripts/ci-setup-acp.mjs",
  );
  assert.equal(installer.if, "matrix.lane == 'cold'");
  assert.ok(
    jobs.browser.steps.some((step) => step.run?.includes("chromium webkit")),
  );
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
test("required check rejects missing or failed shared runtime production", () => {
  const jobs = Object.fromEntries(
    [
      "plan",
      "fast",
      "build",
      "tests",
      "rust",
      "heavy",
      "runtime",
      "storage",
      "web",
      "assets",
      "browser",
      "native",
    ].map((name) => [name, { result: "success" }]),
  );
  const verify = (jobs) =>
    execFileSync(process.execPath, ["scripts/ci-required.mjs"], {
      env: {
        ...process.env,
        CI_STAGE: "heavy",
        CI_PLAN: JSON.stringify({
          rust: true,
          web: true,
          heavy: true,
          full: true,
        }),
        CI_JOBS: JSON.stringify(jobs),
      },
      stdio: ["ignore", "pipe", "pipe"],
    });
  assert.doesNotThrow(() => verify(jobs));
  assert.throws(
    () => verify({ ...jobs, runtime: { result: "failure" } }),
    /runtime/,
  );
  const { runtime, ...missing } = jobs;
  assert.throws(() => verify(missing), /missing planned CI lane runtime/);
});
test("failed early or ordinary lanes cannot start expensive qualification", () => {
  const { jobs } = parseYaml(readFileSync(".github/workflows/ci.yml", "utf8"));
  assert.deepEqual(jobs.tooling.needs, ["plan", "format"]);
  assert.deepEqual(jobs.governance.needs, ["plan", "format"]);
  for (const job of ["build", "web"])
    assert.ok(jobs[job].needs.includes("fast"));
  const needs = {
    plan: {
      outputs: { rust: "true", web: "true", heavy: "true", full: "true" },
    },
    fast: { result: "success" },
    build: { result: "success" },
    rust: { result: "success" },
    web: { result: "success" },
  };
  const eligible = new Function(
    "needs",
    "cancelled",
    `return (${jobs.heavy.if});`,
  );
  assert.equal(
    eligible(needs, () => false),
    true,
  );
  for (const name of ["fast", "build", "rust", "web"])
    for (const result of ["failure", "cancelled", "skipped"])
      assert.equal(
        eligible({ ...needs, [name]: { result } }, () => false),
        false,
        `${name}: ${result}`,
      );
  assert.equal(
    eligible(needs, () => true),
    false,
  );
  const webOnly = {
    ...needs,
    plan: {
      outputs: { rust: "false", web: "true", heavy: "false", full: "false" },
    },
    build: { result: "skipped" },
    rust: { result: "skipped" },
  };
  assert.equal(
    eligible(webOnly, () => false),
    true,
  );
  const docsOnly = {
    ...webOnly,
    plan: {
      outputs: { rust: "false", web: "false", heavy: "false", full: "false" },
    },
    web: { result: "skipped" },
  };
  assert.equal(
    eligible(docsOnly, () => false),
    false,
  );
  const heavy = parseYaml(
    readFileSync(".github/workflows/ci-heavy.yml", "utf8"),
  ).jobs;
  for (const job of [
    jobs.rust,
    heavy.tests,
    heavy.storage,
    heavy.browser,
    heavy.native,
  ])
    assert.equal(job.strategy["fail-fast"], true);
  assert.match(
    readFileSync(".config/nextest.toml", "utf8"),
    /fail-fast = true/,
  );
  for (const name of ["runtime", "assets", "native"])
    assert.ok(heavy[name].needs.includes("tests"));
  assert.ok(heavy.browser.needs.includes("storage"));
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
