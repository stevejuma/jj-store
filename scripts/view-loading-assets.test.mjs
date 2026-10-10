import test from "node:test";
import assert from "node:assert/strict";
import { readFileSync, statSync } from "node:fs";
import { applicationAssets } from "./web-manifest.mjs";

test("built browser defers page views and admits their complete dynamic asset closure", () => {
  const root = new URL("../web/dist/", import.meta.url);
  const manifest = JSON.parse(
    readFileSync(new URL(".vite/manifest.json", root), "utf8"),
  );
  const admitted = new Set(
    JSON.parse(readFileSync(new URL("asset-manifest.json", root), "utf8"))
      .files,
  );
  const initial = new Set();
  function visit(key) {
    if (initial.has(key)) return;
    initial.add(key);
    for (const dependency of manifest[key].imports ?? []) visit(dependency);
  }
  visit("index.html");
  for (const name of [
    "StoreProjects",
    "StoreRepositoryProjects",
    "StoreWork",
    "StoreThreads",
    "StoreAgentSessions",
    "StoreAgentExecutionHost",
    "StoreReviewThreads",
    "CommunityProjectsRepositoryFiles",
    "CommunityProjectsRepositoryLog",
    "StoreSettings",
    "StoreStacksApplicationView",
    "StoreSearchOverlay",
    "StoreConflictEditor",
  ]) {
    const key = Object.keys(manifest).find((key) =>
      key.endsWith(`/${name}.svelte`),
    );
    assert.ok(key, `${name} has an independently emitted view`);
    assert.equal(initial.has(key), false, `${name} is outside shell imports`);
    assert.ok(admitted.has(manifest[key].file), `${name} is admitted`);
  }
  for (const file of applicationAssets(manifest))
    assert.ok(admitted.has(file), `dynamic closure includes ${file}`);
  const html = readFileSync(new URL("index.html", root), "utf8");
  for (const match of html.matchAll(/(?:src|href)="\/(assets\/[^"?]+)"/g))
    assert.ok(admitted.has(match[1]), `HTML asset is admitted: ${match[1]}`);
  for (const file of admitted) {
    if (!file.endsWith(".css")) continue;
    const css = readFileSync(new URL(file, root), "utf8");
    for (const match of css.matchAll(
      /url\(["']?(?:\.\/|\/assets\/)?([^"')]+\.(?:woff2?|ttf))["']?\)/g,
    ))
      assert.ok(
        admitted.has(`assets/${match[1]}`),
        `CSS font is admitted: ${match[1]}`,
      );
  }
  // Existing native BrowserAssets admission bounds remain the delivery gate.
  assert.ok(admitted.size <= 256, "native asset count");
  assert.ok(
    statSync(new URL("asset-manifest.json", root)).size <= 16384,
    "native manifest size",
  );
  let total = 0;
  for (const file of admitted) {
    const bytes = statSync(new URL(file, root)).size;
    assert.ok(bytes <= 8 * 1024 * 1024, `native per-file bound: ${file}`);
    total += bytes;
  }
  assert.ok(total <= 32 * 1024 * 1024, "native bundle bound");
});
