import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import {
  orderedPackages,
  publishablePackages,
  validateVersion,
  validateTag,
  metadata,
  nativeReleaseAssetNames,
  nativeReleaseTargets,
} from "./release-metadata.mjs";
import {
  releaseArchitectureTag,
  shortSource,
  sourceArchitectureTag,
  sourceImageTag,
} from "./image-tags.mjs";
test("publication order resolves internal edges and rejects cycles", () => {
  const packages = [
    { name: "server", dependencies: ["model"] },
    { name: "model", dependencies: [] },
  ];
  assert.deepEqual(
    orderedPackages({ packages }).map((p) => p.name),
    ["model", "server"],
  );
  assert.throws(
    () =>
      orderedPackages({
        packages: [
          { name: "a", dependencies: ["b"] },
          { name: "b", dependencies: ["a"] },
        ],
      }),
    /cycle/,
  );
});
test("release rejects divergent helper, Changesets and tag identities", () => {
  const data = { version: "0.1.0", packages: [{ version: "0.1.1" }] };
  assert.throws(() => validateVersion(data, "0.1.0"), /agree/);
  assert.throws(() => validateTag("v0.2.0", "0.1.0"), /match/);
  validateTag("v0.1.0", "0.1.0");
});
test("native releases expose a bundle and standalone server for every target", () => {
  assert.deepEqual(nativeReleaseTargets, [
    "x86_64-unknown-linux-gnu",
    "aarch64-unknown-linux-gnu",
    "aarch64-apple-darwin",
    "x86_64-apple-darwin",
  ]);
  assert.deepEqual(nativeReleaseAssetNames("0.1.0"), [
    "jj-store-0.1.0-x86_64-unknown-linux-gnu",
    "jj-store-0.1.0-x86_64-unknown-linux-gnu.tar.gz",
    "jj-store-0.1.0-aarch64-unknown-linux-gnu",
    "jj-store-0.1.0-aarch64-unknown-linux-gnu.tar.gz",
    "jj-store-0.1.0-aarch64-apple-darwin",
    "jj-store-0.1.0-aarch64-apple-darwin.tar.gz",
    "jj-store-0.1.0-x86_64-apple-darwin",
    "jj-store-0.1.0-x86_64-apple-darwin.tar.gz",
  ]);
  assert.match(
    readFileSync("scripts/release-archive.mjs", "utf8"),
    /cpSync\(`target\/\$\{target\}\/release\/jj-store`, join\("dist", name\)\)/,
  );
  assert.match(
    readFileSync("scripts/release-finalize.mjs", "utf8"),
    /expected four native bundles and four server executables/,
  );
});
test("main images use bounded source and architecture tags", () => {
  const source = "0123456789abcdef0123456789abcdef01234567";
  assert.equal(shortSource(source), "0123456789ab");
  assert.equal(sourceImageTag(source), "sha-0123456789ab");
  assert.equal(
    sourceArchitectureTag(source, "arm64"),
    "sha-0123456789ab-arm64",
  );
  assert.equal(
    releaseArchitectureTag(source, "amd64"),
    "release-sha-0123456789ab-amd64",
  );
  assert.throws(() => sourceImageTag("0123456"), /invalid/);
  assert.throws(() => sourceArchitectureTag(source, "ppc64"), /unsupported/);
});
test("all public package versions have an acyclic coordinated graph", () => {
  const data = metadata();
  const graph = JSON.parse(
    execFileSync(
      "cargo",
      [
        "metadata",
        "--locked",
        "--offline",
        "--no-deps",
        "--format-version",
        "1",
      ],
      { encoding: "utf8" },
    ),
  );
  const members = new Set(graph.workspace_members);
  const expected = graph.packages
    .filter((pkg) => members.has(pkg.id))
    .map((pkg) => pkg.name);
  expected.push("jj-store-embedded-rustfs");
  assert.deepEqual(
    data.packages.map((pkg) => pkg.name).sort(),
    expected.sort(),
  );
  assert.ok(data.packages.some((p) => p.name === "jj-forge"));
  const order = new Map(
    orderedPackages(data).map((pkg, index) => [pkg.name, index]),
  );
  const external = {
    "jj-store-capture": ["lapis-agent-redaction", "lapis-ai-capture"],
    "jj-store-conversations": ["lapis-agent-redaction"],
    "jj-store-server": ["lapis-ai-runtime", "lapis-ai-config"],
  };
  for (const pkg of data.packages)
    for (const dependency of pkg.dependencies) {
      if (!order.has(dependency))
        assert.ok(
          external[pkg.name]?.includes(dependency),
          `unmodelled external dependency ${dependency}`,
        );
      else
        assert.ok(
          order.get(dependency) < order.get(pkg.name),
          `${dependency} before ${pkg.name}`,
        );
    }
  validateVersion(data, data.version);
  const published = publishablePackages(data).map((pkg) => pkg.name);
  for (const name of [
    "jj-history-service",
    "jj-vault",
    "tauri-plugin-jj-store",
    "jj-store-embedded-rustfs",
  ])
    assert.equal(published.includes(name), false, name);
  assert.ok(published.includes("jj-store"));
  assert.ok(published.includes("jj-store-model"));
});
