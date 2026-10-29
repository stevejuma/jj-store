import test from "node:test";
import assert from "node:assert/strict";
import { mkdtemp, writeFile, rm } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import { runBrowserTests, previewCommand } from "./browser-tests.mjs";

test("browser startup waits for preparation and retains exact target and caller filters", async () => {
  let prepared;
  const gate = new Promise((resolve) => {
    prepared = resolve;
  });
  const calls = [];
  const run = runBrowserTests("app", ["reads.spec.ts", "-g", "one case"], {
    root: "/workspace",
    environment: { CI: "true" },
    prepare: () => gate,
    execute: async (...args) => {
      calls.push(args);
    },
  });
  await Promise.resolve();
  assert.equal(calls.length, 0);
  prepared("/registered target/debug/examples/browser_preview");
  await run;
  assert.deepEqual(calls[0].slice(0, 2), [
    "playwright",
    [
      "test",
      "--config",
      "web/playwright.config.ts",
      "reads.spec.ts",
      "-g",
      "one case",
    ],
  ]);
  assert.equal(
    calls[0][2].env.JJ_STORE_BROWSER_PREVIEW_EXECUTABLE,
    "/registered target/debug/examples/browser_preview",
  );
  assert.equal(calls[0][2].env.CI, "true");
});
test("failed preview preparation never starts app or Help browser acceptance", async () => {
  for (const kind of ["app", "docs"]) {
    let started = false;
    await assert.rejects(
      runBrowserTests(kind, [], {
        prepare: async () => {
          throw Error("build failed");
        },
        execute: async () => {
          started = true;
        },
      }),
      /build failed/,
    );
    assert.equal(started, false);
  }
});
test("preview commands preserve spaces and apostrophes without evaluating path text", async (t) => {
  const dir = await mkdtemp(join(tmpdir(), "jj-browser-preview-"));
  t.after(() => rm(dir, { recursive: true, force: true }));
  const executable = join(dir, "preview ' with spaces");
  await writeFile(executable, "#!/bin/sh\nprintf '%s' 'exact preview'\n", {
    mode: 0o755,
  });
  assert.equal(
    execFileSync("/bin/sh", ["-c", previewCommand(executable)], {
      encoding: "utf8",
    }),
    "exact preview",
  );
});
