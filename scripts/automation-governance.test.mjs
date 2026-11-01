import test from "node:test";
import assert from "node:assert/strict";
import {
  mkdtempSync,
  readFileSync,
  writeFileSync,
  mkdirSync,
  cpSync,
  rmSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createRequire } from "node:module";
import { execFileSync, spawnSync } from "node:child_process";
import { parse } from "yaml";
import { botSpecificationExemption } from "./dependency-automation/spec-policy.mjs";

const root = new URL("../", import.meta.url);
const ci = parse(
  readFileSync(new URL(".github/workflows/ci.yml", root), "utf8"),
);
const heavyCi = parse(
  readFileSync(new URL(".github/workflows/ci-heavy.yml", root), "utf8"),
);
const gates = [
  "Progress evidence gate",
  "Spec-first gate",
  "Release intent gate",
];

test("bot PRs skip only change-specific governance", () => {
  for (const name of gates) {
    const step = ci.jobs.governance.steps.find((step) => step.name === name);
    assert.ok(step, `${name} remains present`);
    const condition = new Function(
      "github",
      "startsWith",
      "contains",
      "fromJSON",
      "vars",
      `return (${step.if});`,
    );
    const evaluate = (head_ref, author, event_name = "pull_request") =>
      condition(
        {
          event_name,
          head_ref,
          event: {
            pull_request: {
              user: {
                login: author,
                type: author.endsWith("[bot]") ? "Bot" : "User",
              },
            },
          },
        },
        (text, prefix) => text.startsWith(prefix),
        (list, item) => list.includes(item),
        JSON.parse,
        { LAPIS_DEPENDENCY_BOT_LOGIN: "" },
      );
    assert.equal(
      evaluate("changeset-release/main", "github-actions[bot]"),
      false,
      name,
    );
    assert.equal(
      evaluate("dependabot/cargo/jj-lib-0.46.0", "dependabot[bot]"),
      false,
      name,
    );
    assert.equal(
      evaluate("dependabot/npm_and_yarn/yaml-3.0.0", "dependabot[bot]"),
      false,
      name,
    );
    for (const author of [
      "dependabot[bot]",
      "renovate[bot]",
      "lapismd-dependency-bot[bot]",
    ])
      assert.equal(
        evaluate("lapis-dependencies/lapis-first-party", author),
        name === "Release intent gate",
        `${name}: trusted dependency author ${author}`,
      );
    for (const [branch, author] of [
      ["fix/storage", "maintainer"],
      ["changeset-release/other", "github-actions[bot]"],
      ["dependabot/cargo/jj-lib-0.46.0", "maintainer"],
      ["dependabot/cargo/jj-lib-0.46.0", "other[bot]"],
    ])
      assert.equal(
        evaluate(branch, author),
        true,
        `${name}: ${branch} by ${author}`,
      );
    for (const event of [
      "push",
      "schedule",
      "workflow_dispatch",
      "workflow_call",
    ])
      assert.equal(
        evaluate("changeset-release/main", "github-actions[bot]", event),
        false,
        `${name}: ${event}`,
      );
    assert.equal(
      evaluate("dependabot/cargo/jj-lib-0.46.0", "maintainer", "push"),
      false,
      name,
    );
  }
});

test("bot PRs skip specification calls and retain planned functional and release lanes", () => {
  assert.equal(ci.jobs.governance.if, undefined);
  const staticStep = ci.jobs.governance.steps.find((step) =>
    step.run?.includes("pnpm spec:validate"),
  );
  assert.ok(staticStep);
  assert.equal(staticStep.if, undefined);
  assert.match(staticStep.run, /pnpm check:architecture/);
  assert.match(staticStep.run, /pnpm check:ui-ownership/);
  assert.equal(
    botSpecificationExemption({
      CI: "true",
      LAPIS_CI_EVENT: "pull_request",
      LAPIS_CI_PR_AUTHOR: "lapismd-dependency-bot[bot]",
      LAPIS_CI_PR_AUTHOR_TYPE: "Bot",
    }),
    true,
  );
  for (const command of [
    "pnpm test:governance",
    "pnpm test:spec",
    "pnpm release:check",
  ])
    assert.ok(
      ci.jobs.tooling.steps.some(
        (step) => step.if === undefined && step.run?.includes(command),
      ),
      command,
    );
  for (const name of ["build", "rust", "web", "ui"]) {
    assert.ok(ci.jobs.required.needs.includes(name), name);
    assert.doesNotMatch(
      ci.jobs[name].if ?? "",
      /dependabot|changeset-release/,
      name,
    );
  }
  assert.equal(ci.jobs.required.if, "always()");
  for (const name of [
    "build",
    "backend",
    "runtime",
    "external",
    "embedded",
    "contracts",
    "assets",
    "standard",
    "cold",
    "native",
  ]) {
    assert.ok(heavyCi.jobs.complete.needs.includes(name), name);
    assert.doesNotMatch(
      heavyCi.jobs[name].if ?? "",
      /dependabot|changeset-release/,
      name,
    );
  }
  const packageJson = JSON.parse(
    readFileSync(new URL("package.json", root), "utf8"),
  );
  const lanes = JSON.parse(
    readFileSync(new URL("scripts/validation/lanes.json", root), "utf8"),
  );
  assert.equal(
    packageJson.scripts["release:check"],
    "node scripts/validation.mjs run release:check",
  );
  for (const test of [
    "scripts/release.test.mjs",
    "scripts/ci*.test.mjs",
    "scripts/automation-governance.test.mjs",
  ])
    assert.ok(lanes["release:check"].includes(test), test);
  assert.ok(lanes["release:check"].includes("node scripts/package-check.mjs"));
});

function versionFixture() {
  const directory = mkdtempSync(join(tmpdir(), "jj-store-automation-"));
  const write = (path, source) => {
    mkdirSync(join(directory, path, ".."), { recursive: true });
    writeFileSync(join(directory, path), source);
  };
  write("package.json", JSON.stringify({ version: "0.1.1" }));
  write(
    "Cargo.toml",
    '[workspace]\nmembers = ["crates/model"]\n[workspace.package]\nversion = "0.1.0"\n[workspace.dependencies]\nmodel = { path = "crates/model", version = "0.1.0" }\n',
  );
  write(
    "crates/model/Cargo.toml",
    '[package]\nname = "model"\nversion.workspace = true\n',
  );
  write(
    "runtime/embedded-rustfs/Cargo.toml",
    '[package]\nname = "jj-store-embedded-rustfs"\nversion = "0.1.0"\n',
  );
  write("Cargo.lock", '[[package]]\nname = "model"\nversion = "0.1.0"\n');
  write(
    "runtime/embedded-rustfs/Cargo.lock",
    '[[package]]\nname = "jj-store-embedded-rustfs"\nversion = "0.1.0"\n',
  );
  write(
    "spec/src/current/contracts/architecture/23-public-distribution.md",
    "Canonical contract remains unchanged.\n",
  );
  for (const file of ["release-version.mjs", "release-metadata.mjs"])
    cpSync(new URL(`scripts/${file}`, root), join(directory, "scripts", file));
  return { directory, write };
}

test("version synchronization leaves spec and evidence unchanged", () => {
  const { directory } = versionFixture();
  try {
    execFileSync(process.execPath, ["scripts/release-version.mjs"], {
      cwd: directory,
      stdio: "pipe",
    });
    for (const file of [
      "Cargo.toml",
      "Cargo.lock",
      "runtime/embedded-rustfs/Cargo.toml",
      "runtime/embedded-rustfs/Cargo.lock",
    ])
      assert.match(
        readFileSync(join(directory, file), "utf8"),
        /version = "0\.1\.1"/,
        file,
      );
    assert.match(
      readFileSync(join(directory, "Cargo.toml"), "utf8"),
      /path = "crates\/model", version = "0\.1\.1"/,
    );
    assert.equal(
      readFileSync(
        join(
          directory,
          "spec/src/current/contracts/architecture/23-public-distribution.md",
        ),
        "utf8",
      ),
      "Canonical contract remains unchanged.\n",
    );
    const files = execFileSync(
      "python3",
      [
        "-c",
        "from pathlib import Path; print('\\n'.join(str(p.relative_to('.')) for p in Path('spec').rglob('*') if p.is_file()))",
      ],
      { cwd: directory, encoding: "utf8" },
    )
      .trim()
      .split("\n");
    assert.deepEqual(files, [
      "spec/src/current/contracts/architecture/23-public-distribution.md",
    ]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("release intent still enforces notes on ordinary PRs", () => {
  const { directory, write } = versionFixture();
  const git = (...args) =>
    execFileSync("git", ["-c", "commit.gpgsign=false", ...args], {
      cwd: directory,
      encoding: "utf8",
    }).trim();
  try {
    write("package.json", JSON.stringify({ version: "0.1.0" }));
    cpSync(
      new URL("scripts/changeset-check.mjs", root),
      join(directory, "scripts/changeset-check.mjs"),
    );
    git("init", "-q");
    git("config", "user.name", "Fixture");
    git("config", "user.email", "fixture@example.invalid");
    git("add", ".");
    git("commit", "-qm", "baseline");
    git("tag", "v0.1.0");
    const base = git("rev-parse", "HEAD");
    write("crates/model/src/lib.rs", "// Dependency update fixture\n");
    git("add", ".");
    git("commit", "-qm", "dependency update");
    const check = (branch, author) =>
      spawnSync(process.execPath, ["scripts/changeset-check.mjs"], {
        cwd: directory,
        encoding: "utf8",
        env: {
          ...process.env,
          CI_EVENT: "pull_request",
          GITHUB_HEAD_REF: branch,
          CI_PR_AUTHOR: author,
          JJ_STORE_CHECK_BASE: base,
          JJ_STORE_CHECK_REVISION: git("rev-parse", "HEAD"),
        },
      });
    for (const [branch, author] of [
      ["changeset-release/main", "github-actions[bot]"],
      ["dependabot/cargo/model-1.0.1", "dependabot[bot]"],
    ]) {
      const result = check(branch, author);
      assert.equal(result.status, 0, result.stderr);
    }
    for (const [branch, author] of [
      ["fix/model", "maintainer"],
      ["changeset-release/other", "github-actions[bot]"],
      ["dependabot/cargo/model-1.0.1", "maintainer"],
    ]) {
      const result = check(branch, author);
      assert.notEqual(result.status, 0, branch);
      assert.match(result.stderr, /require a Changesets note/);
    }
    write(
      ".changeset/fix-model.md",
      '---\n"@lapismd/jj-store": patch\n---\nUpdate the model dependency.\n',
    );
    git("add", ".");
    git("commit", "-qm", "release note");
    assert.equal(check("fix/model", "maintainer").status, 0);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("Changesets selects versions without writing spec evidence", () => {
  const { directory, write } = versionFixture();
  try {
    write(
      "package.json",
      JSON.stringify({
        name: "@lapismd/jj-store",
        version: "0.1.0",
        private: true,
      }),
    );
    const config = JSON.parse(
      readFileSync(new URL(".changeset/config.json", root), "utf8"),
    );
    // Local version selection does not need the GitHub changelog provider.
    config.changelog = false;
    write(".changeset/config.json", JSON.stringify(config));
    write(
      ".changeset/patch.md",
      '---\n"@lapismd/jj-store": patch\n---\nCoordinate a patch release.\n',
    );
    const git = (...args) =>
      execFileSync("git", ["-c", "commit.gpgsign=false", ...args], {
        cwd: directory,
        stdio: "pipe",
      });
    git("init", "-q");
    git("config", "user.name", "Fixture");
    git("config", "user.email", "fixture@example.invalid");
    git("add", ".");
    git("commit", "-qm", "release source");
    const cli = createRequire(import.meta.url).resolve(
      "@changesets/cli/bin.js",
    );
    execFileSync(process.execPath, [cli, "version"], {
      cwd: directory,
      stdio: "pipe",
    });
    execFileSync(process.execPath, ["scripts/release-version.mjs"], {
      cwd: directory,
      stdio: "pipe",
    });
    assert.equal(
      JSON.parse(readFileSync(join(directory, "package.json"), "utf8")).version,
      "0.1.1",
    );
    assert.match(
      readFileSync(join(directory, "Cargo.toml"), "utf8"),
      /version = "0\.1\.1"/,
    );
    const changed = execFileSync("git", ["diff", "--name-only"], {
      cwd: directory,
      encoding: "utf8",
    });
    assert.doesNotMatch(changed, /^spec\//m);
    const untracked = execFileSync(
      "git",
      ["ls-files", "--others", "--exclude-standard"],
      { cwd: directory, encoding: "utf8" },
    );
    assert.doesNotMatch(untracked, /^spec\//m);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
