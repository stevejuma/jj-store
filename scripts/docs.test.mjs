import { test } from "node:test";
import assert from "node:assert/strict";
import {
  mkdtempSync,
  mkdirSync,
  writeFileSync,
  rmSync,
  symlinkSync,
  readFileSync,
  existsSync,
} from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import {
  sourceFingerprint,
  validateReview,
  validateCoverage,
} from "./docs/review.mjs";
import { renderCommand } from "./docs/references.mjs";
import { helpAssets } from "./docs/assets.mjs";
function fixture(t) {
  const root = mkdtempSync(join(tmpdir(), "jj-store-docs-"));
  t.after(() => rmSync(root, { recursive: true, force: true }));
  return root;
}
test("shared agent long help links target published manual pages", () => {
  const markdown = renderCommand({
    path: "jj agent",
    about: readFileSync("docs/cli/agent.md", "utf8"),
    usage: "Usage: jj agent",
    aliases: [],
    arguments: [],
    commands: [],
  });
  const links = [...markdown.matchAll(/\]\(([^)#]+\.md)(?:#[^)]*)?\)/g)];
  assert.ok(links.length > 1);
  for (const [, path] of links)
    assert.ok(existsSync(resolve("docs/site/cli", path)), path);
});
test("source changes and new files invalidate an acknowledged topic", (t) => {
  const relative = ".scratch/docs-review-fixture";
  mkdirSync(relative, { recursive: true });
  t.after(() => rmSync(relative, { recursive: true, force: true }));
  writeFileSync(`${relative}/source.rs`, "old behavior");
  writeFileSync(`${relative}/page.md`, "# Documentation");
  const topic = {
    id: "sample",
    sources: [`${relative}/source.rs`],
    pages: [`${relative}/page.md`],
    review: {
      reason: "Reviewed the exact behavior and its limits",
      fingerprint: sourceFingerprint([`${relative}/source.rs`]),
    },
  };
  validateReview([topic]);
  writeFileSync(`${relative}/source.rs`, "new behavior");
  assert.throws(() => validateReview([topic]), /review required/);
  topic.sources = [relative];
  topic.review.fingerprint = sourceFingerprint([relative]);
  writeFileSync(`${relative}/new-command.rs`, "new command");
  assert.throws(() => validateReview([topic]), /review required/);
  assert.throws(
    () => validateCoverage([topic], ["crates/new/src"]),
    /Unmapped/,
  );
  assert.throws(() => validateReview([]), /empty/);
});
test("command reference retains nested usage aliases values and inherited options", () => {
  const command = {
    path: "jj store",
    about: "Store",
    usage: "Usage: jj store [OPTIONS]",
    aliases: ["native"],
    arguments: [
      {
        id: "repository",
        long: "repository",
        short: "R",
        global: true,
        help: "Repository | directory",
        defaults: ["."],
        possible: ["one", "two"],
      },
    ],
    commands: [
      {
        path: "jj store status",
        usage: "Usage: jj store status",
        aliases: [],
        arguments: [],
        commands: [],
      },
    ],
  };
  const markdown = renderCommand(command);
  for (const expected of [
    "## jj store status",
    "Usage: jj store [OPTIONS]",
    "`native`",
    "-R, --repository",
    "Inherited by subcommands.",
    "Default: .",
    "Values: one, two",
    "Repository \\| directory",
  ])
    assert.ok(markdown.includes(expected), expected);
});
function bundle(root, files = ["index.html", "concepts/index.html"]) {
  mkdirSync(join(root, "concepts"), { recursive: true });
  writeFileSync(join(root, "index.html"), "<h1>Help</h1>");
  writeFileSync(join(root, "concepts/index.html"), "<h1>Concepts</h1>");
  writeFileSync(
    join(root, "asset-manifest.json"),
    JSON.stringify({ version: 1, files }),
  );
}
test("desktop and development delivery accept only a bounded safe documentation closure", async (t) => {
  const root = fixture(t);
  bundle(root);
  assert.equal((await helpAssets(root)).size, 2);
  for (const files of [
    ["index.html", "index.html"],
    ["index.html", "../secret"],
    ["index.html", "missing.js"],
    ["index.html", "asset-manifest.json"],
    ["index.html", "secret.toml"],
    Array(513).fill("index.html"),
  ]) {
    bundle(root, files);
    await assert.rejects(helpAssets(root));
  }
  bundle(root);
  rmSync(join(root, "concepts"), { recursive: true });
  symlinkSync(root, join(root, "concepts"));
  await assert.rejects(helpAssets(root), /symlink/);
});
test("generated closure rejects broken anchors inline scripts and remote runtime assets", (t) => {
  const root = fixture(t);
  const validate = () =>
    spawnSync(
      "uv",
      ["run", "--locked", "python", "scripts/docs/site.py", root],
      { encoding: "utf8" },
    );
  writeFileSync(
    join(root, "index.html"),
    '<h1 id="hello">Help</h1><a href="#hello">Hello</a>',
  );
  assert.equal(validate().status, 0);
  for (const html of [
    '<a href="#absent">Broken</a>',
    "<script>alert(1)</script>",
    '<script src="https://example.test/a.js"></script>',
    '<img src="missing.svg">',
  ]) {
    writeFileSync(join(root, "index.html"), html);
    assert.notEqual(validate().status, 0, html);
  }
});

test("generated closure requires both local diagram theme variants", (t) => {
  const root = fixture(t);
  writeFileSync(
    join(root, "light.svg"),
    '<svg xmlns="http://www.w3.org/2000/svg"/>',
  );
  writeFileSync(
    join(root, "dark.svg"),
    '<svg xmlns="http://www.w3.org/2000/svg"/>',
  );
  for (const dark of [
    "dark.svg",
    "missing.svg",
    "https://example.test/dark.svg",
    "",
  ]) {
    writeFileSync(
      join(root, "index.html"),
      `<figure class="architecture-diagram"><img src="light.svg" data-light-src="light.svg" data-dark-src="${dark}"></figure>`,
    );
    const result = spawnSync(
      "uv",
      ["run", "--locked", "python", "scripts/docs/site.py", root],
      { encoding: "utf8" },
    );
    assert.equal(result.status === 0, dark === "dark.svg", result.stderr);
  }
});

test("desktop staging copies the complete Help closure and rejects missing help before replacement", async (t) => {
  const { stageDesktopResources } = await import("./desktop-resources.mjs");
  const { readFileSync, existsSync } = await import("node:fs");
  const root = fixture(t),
    web = join(root, "web"),
    target = join(root, "bin"),
    destination = join(root, "resources"),
    runtime = join(root, "rustfs");
  mkdirSync(web);
  mkdirSync(target);
  mkdirSync(join(web, "help"));
  bundle(join(web, "help"));
  writeFileSync(join(web, "index.html"), "application");
  writeFileSync(
    join(web, "asset-manifest.json"),
    JSON.stringify({ version: 1, files: ["index.html"] }),
  );
  for (const name of ["jj", "jj-store"])
    writeFileSync(join(target, name), "test binary");
  writeFileSync(runtime, "test runtime");
  await stageDesktopResources({ target, runtime, web, destination });
  assert.equal(
    readFileSync(join(destination, "web/help/concepts/index.html"), "utf8"),
    "<h1>Concepts</h1>",
  );
  assert.ok(existsSync(join(destination, "web/help/asset-manifest.json")));
  rmSync(join(web, "help/index.html"));
  await assert.rejects(
    stageDesktopResources({ target, runtime, web, destination }),
  );
  assert.equal(
    readFileSync(join(destination, "web/index.html"), "utf8"),
    "application",
  );
});

test("generated reference comparison fails after manual drift", async (t) => {
  const { compareReferences } = await import("./docs/references.mjs");
  const root = fixture(t),
    path = join(root, "cli.md");
  const expected = new Map([[path, "# Exact command\n"]]);
  compareReferences(expected, true);
  compareReferences(expected);
  writeFileSync(path, "# Stale command\n");
  assert.throws(() => compareReferences(expected), /Stale reference/);
});
