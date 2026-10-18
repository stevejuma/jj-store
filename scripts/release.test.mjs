import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import {
  orderedPackages,
  validateVersion,
  validateTag,
  metadata,
} from "./release-metadata.mjs";
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
});
