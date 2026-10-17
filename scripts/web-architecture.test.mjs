import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { architectureErrors, countedLines } from "./web-architecture.mjs";

function fixture(files) {
  const root = mkdtempSync(join(tmpdir(), "jj-store-web-architecture-"));
  for (const directory of ["catalog", "scripts", "web/src"])
    mkdirSync(join(root, directory), { recursive: true });
  for (const [path, source] of Object.entries(files)) {
    mkdirSync(join(root, path, ".."), { recursive: true });
    writeFileSync(join(root, path), source);
  }
  return root;
}

test("counts production while excluding blank and comment-only lines", () => {
  assert.equal(countedLines("// note\n\nconst value = 1;\n<!-- note -->\n"), 1);
});

test("rejects a new oversized production module", () => {
  const root = fixture({
    "web/src/large.ts": "const value = 1;\n".repeat(301),
  });
  assert.match(
    architectureErrors(root, { files: {} }).join("\n"),
    /301 counted/,
  );
});

test("rejects production imports from catalog and fixtures", () => {
  const root = fixture({
    "web/src/app.ts": 'import "../../catalog/fixtures/view.ts";\n',
  });
  const errors = architectureErrors(root, { files: {} }).join("\n");
  assert.match(errors, /imports catalog code/);
  assert.match(errors, /imports test or fixture code/);
});

test("allows stories to consume production presentation modules", () => {
  const root = fixture({
    "web/src/lib/presentation/view.ts": "export const view = 1;\n",
    "catalog/View.stories.svelte":
      '<script>import { view } from "../web/src/lib/presentation/view";</script>\n{view}\n',
  });
  assert.deepEqual(architectureErrors(root, { files: {} }), []);
});

test("headless package sources share the production size boundary", () => {
  const root = fixture({
    "packages/jj-conversations/src/large.ts": "const value = 1;\n".repeat(301),
  });
  assert.match(
    architectureErrors(root, { files: {} }).join("\n"),
    /packages\/jj-conversations\/src\/large.ts: 301 counted lines exceeds 300/,
  );
});
