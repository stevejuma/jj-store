import assert from "node:assert/strict";
import path from "node:path";
import test from "node:test";
import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { createHooks } = require("./local-package-pnpmfile.cjs");

test("local package hook uses available Design Core source", () => {
  const root = "/workspace/jj-store";
  const designCore = path.resolve(root, "../design-core");
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

test("local package hook uses available agent redaction source", () => {
  const root = "/workspace/jj-store";
  const agentRedaction = path.resolve(root, "../agent-redaction");
  const manifest = {
    dependencies: { "@lapismd/agent-redaction": "^0.1.0" },
  };

  const result = createHooks({
    root,
    fileExists: (candidate) => candidate === agentRedaction,
  }).hooks.readPackage(manifest);

  assert.equal(
    result.dependencies["@lapismd/agent-redaction"],
    `file:${agentRedaction}`,
  );
});

test("local package hook retains registry range when source is absent", () => {
  const manifest = {
    devDependencies: { "@lapismd/design-core": "^0.2.0" },
  };

  const result = createHooks({
    root: "/workspace/jj-store",
    fileExists: () => false,
  }).hooks.readPackage(manifest);

  assert.deepEqual(result, manifest);
});
