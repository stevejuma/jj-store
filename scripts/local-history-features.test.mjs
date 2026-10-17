import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { readdirSync } from "node:fs";

const cargo = (args) => execFileSync("cargo", args, { encoding: "utf8" });

test("standalone local history excludes native publication and host runtimes", () => {
  const tree = cargo([
    "tree",
    "--locked",
    "--offline",
    "-p",
    "jj-store-jj",
    "--no-default-features",
    "--features",
    "local-history",
    "--edges",
    "normal",
    "--prefix",
    "none",
    "--format",
    "{p}",
  ]);
  assert.deepEqual(
    tree
      .split("\n")
      .filter((line) =>
        /^(?:jj-store-(?:object|wal|native|service|rpc|http|server)|jj-forge|lapis-ai-runtime|tauri) v/.test(
          line,
        ),
      ),
    [],
  );
});

test("default native retains every integration binary and headless gates native-only tests", () => {
  const metadata = JSON.parse(
    cargo([
      "metadata",
      "--locked",
      "--offline",
      "--no-deps",
      "--format-version",
      "1",
    ]),
  );
  const pkg = metadata.packages.find((pkg) => pkg.name === "jj-store-jj");
  assert.deepEqual(pkg.features.default, ["native"]);
  assert.ok(pkg.features.native.includes("local-history"));
  assert.ok(pkg.features["local-history"].includes("companion"));
  const tests = pkg.targets.filter((target) => target.kind.includes("test"));
  assert.deepEqual(
    tests.map((target) => target.name).sort(),
    readdirSync("crates/jj-store-jj/tests")
      .filter((name) => name.endsWith(".rs"))
      .map((name) => name.slice(0, -3))
      .sort(),
  );
  for (const target of pkg.targets.filter((target) =>
    target.kind.includes("example"),
  )) {
    assert.deepEqual(target["required-features"], ["native"], target.name);
  }
  for (const target of tests) {
    const expected =
      target.name === "local_history"
        ? ["local-history"]
        : ["local_evidence", "local_metadata", "local_discovery"].includes(
              target.name,
            )
          ? []
          : ["native"];
    assert.deepEqual(target["required-features"] ?? [], expected, target.name);
  }
});

test("history bridge defaults to a headless service and preserves native consumers", () => {
  const tree = cargo([
    "tree",
    "--locked",
    "--offline",
    "-p",
    "tauri-plugin-jj-store",
    "--no-default-features",
    "--edges",
    "normal",
    "--prefix",
    "none",
    "--format",
    "{p}",
  ]);
  assert.deepEqual(
    tree
      .split("\n")
      .filter((line) =>
        /^(?:jj-store-(?:object|wal|native|service|rpc|http|server)|jj-forge|lapis-ai-runtime|tauri) v/.test(
          line,
        ),
      ),
    [],
  );
  const metadata = JSON.parse(
    cargo([
      "metadata",
      "--locked",
      "--offline",
      "--no-deps",
      "--format-version",
      "1",
    ]),
  );
  const bridge = metadata.packages.find(
    (pkg) => pkg.name === "tauri-plugin-jj-store",
  );
  assert.deepEqual(bridge.features.default, []);
  for (const name of [
    "jj-store",
    "jj-store-service",
    "jj-store-rpc",
    "jj-store-server",
  ]) {
    const pkg = metadata.packages.find((pkg) => pkg.name === name);
    const dependency = pkg.dependencies.find(
      (dep) => dep.name === "jj-store-jj",
    );
    assert.ok(dependency.features.includes("native"), name);
  }
});
