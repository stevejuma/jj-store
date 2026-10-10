import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { createHooks } = require("./local-package-pnpmfile.cjs");

test("local package hook uses built Mira and keeps registry fallback and peers portable", () => {
  const root = "/workspace/jj-store";
  const archive = path.resolve(
    root,
    "../mira-mde/.local/packages/lapismd-mira-0.0.2.tgz",
  );
  const manifest = {
    devDependencies: { "@lapismd/mira": "^0.0.2" },
    peerDependencies: { "@lapismd/mira": "^0.0.2" },
  };
  const result = createHooks({
    root,
    fileExists: (file) => file === archive,
  }).hooks.readPackage(structuredClone(manifest));
  assert.equal(result.devDependencies["@lapismd/mira"], `file:${archive}`);
  assert.equal(result.peerDependencies["@lapismd/mira"], "^0.0.2");
  assert.deepEqual(
    createHooks({ root, fileExists: () => false }).hooks.readPackage(
      structuredClone(manifest),
    ),
    manifest,
  );
});

test("local package hook uses the packed Design Core artifact without rewriting peers", () => {
  const root = "/workspace/jj-store";
  const designCore = path.resolve(
    root,
    "../design-core/.local/packages/lapismd-design-core-0.1.2-built.tgz",
  );
  const manifest = {
    devDependencies: { "@lapismd/design-core": "^0.2.0" },
    peerDependencies: { "@lapismd/design-core": "^0.2.0" },
  };

  const result = createHooks({
    root,
    fileExists: (candidate) => candidate === designCore,
  }).hooks.readPackage(manifest);

  assert.equal(
    result.devDependencies["@lapismd/design-core"],
    `file:${designCore}`,
  );
  assert.equal(result.peerDependencies["@lapismd/design-core"], "^0.2.0");
});

test("local package hook retains registry range when only sibling source exists", () => {
  const manifest = {
    devDependencies: { "@lapismd/design-core": "^0.2.0" },
  };

  const result = createHooks({
    root: "/workspace/jj-store",
    fileExists: (candidate) => candidate === "/workspace/design-core",
  }).hooks.readPackage(manifest);

  assert.deepEqual(result, manifest);
});
