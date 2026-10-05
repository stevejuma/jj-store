import { test } from "node:test";
import assert from "node:assert/strict";
import { applicationAssets } from "./web-manifest.mjs";
test("browser manifest retains dynamic chunks and fonts while excluding unreferenced workers", () => {
  const manifest = {
    "index.html": {
      file: "assets/index.js",
      imports: ["shared"],
      dynamicImports: ["preview"],
      css: ["assets/index.css"],
      assets: ["assets/body.woff2"],
    },
    shared: { file: "assets/shared.js", imports: ["shared"] },
    preview: { file: "assets/preview.js", assets: ["assets/math.ttf"] },
    unused: { file: "assets/worker.js", dynamicImports: ["oversized"] },
    oversized: { file: "assets/oversized.js" },
  };
  assert.deepEqual(applicationAssets(manifest), [
    "assets/body.woff2",
    "assets/index.css",
    "assets/index.js",
    "assets/math.ttf",
    "assets/preview.js",
    "assets/shared.js",
    "index.html",
  ]);
});
test("browser manifest rejects missing dependencies and unsafe asset paths", () => {
  assert.throws(
    () =>
      applicationAssets({
        "index.html": { file: "assets/index.js", imports: ["missing"] },
      }),
    /Missing browser build entry/,
  );
  assert.throws(
    () => applicationAssets({ "index.html": { file: "assets/..bad.js" } }),
    /Unexpected browser asset path/,
  );
});
