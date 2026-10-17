import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { dependencyErrors } from "./check-boundaries.mjs";

test("conversation primitives have no application runtime or storage dependency", () => {
  const tree = execFileSync(
    "cargo",
    [
      "tree",
      "--locked",
      "--offline",
      "-p",
      "jj-store-conversations",
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
        /^(?:jj-store-(?:service|rpc|wal|object|native|jj|capture)|jj-lib|jj-cli|lapis-ai-runtime|tauri|tokio) v/.test(
          line,
        ),
      ),
    [],
  );
  for (const dependency of [
    "jj-store-service",
    "jj-store-wal",
    "jj-store-capture",
    "jj-lib",
    "lapis-ai-runtime",
  ]) {
    assert.equal(
      dependencyErrors([
        {
          name: "jj-store-conversations",
          dependencies: [{ name: dependency }],
        },
      ]).length,
      1,
    );
  }
});
