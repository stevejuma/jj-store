import test from "node:test";
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";

test("headless capture has no application publication or upstream JJ dependency", () => {
  const tree = execFileSync(
    "cargo",
    [
      "tree",
      "--locked",
      "--offline",
      "-p",
      "jj-store-capture",
      "--no-default-features",
      "--edges",
      "normal",
      "--prefix",
      "none",
      "--format",
      "{p}",
    ],
    { encoding: "utf8", env: { ...process.env, CARGO_INCREMENTAL: "0" } },
  );
  const forbidden = tree
    .split("\n")
    .filter((line) =>
      /^(?:jj-store-(?:service|rpc|wal|object|native|jj)|jj-lib|jj-cli|lapis-ai-runtime|tauri) v/.test(
        line,
      ),
    );
  assert.deepEqual(forbidden, []);
});
