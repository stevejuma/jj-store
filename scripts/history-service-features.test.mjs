import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";

const metadata = () =>
  JSON.parse(
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

test("the reusable history service owns no Tauri or publication runtime", () => {
  const pkg = metadata().packages.find(
    (item) => item.name === "jj-history-service",
  );
  assert.ok(
    pkg,
    "The shared history implementation must have an independent crate",
  );
  assert.equal(pkg.links, null);
  const tree = execFileSync(
    "cargo",
    [
      "tree",
      "--locked",
      "--offline",
      "-p",
      "jj-history-service",
      "--edges",
      "normal",
      "--prefix",
      "none",
      "--format",
      "{p}",
    ],
    { encoding: "utf8" },
  );
  assert.deepEqual(
    tree
      .split("\n")
      .filter((line) =>
        /^(?:tauri|jj-store-(?:object|wal|native|service|rpc|http|server)|jj-forge|lapis-ai-runtime) v/.test(
          line,
        ),
      ),
    [],
  );
});

test("the optional Tauri adapter consumes the shared service", () => {
  const packages = metadata().packages;
  const adapter = packages.find(
    (item) => item.name === "tauri-plugin-jj-store",
  );
  assert.ok(
    adapter.dependencies.some((item) => item.name === "jj-history-service"),
  );
  assert.ok(!adapter.dependencies.some((item) => item.name === "jj-store-jj"));
  assert.deepEqual(adapter.features.default, []);
  assert.ok(
    adapter.dependencies.find((item) => item.name === "tauri").optional,
  );
});
