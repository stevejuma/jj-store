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
        /^(?:jj-(?:agent|collab|review|work)|jj-store-(?:core|model|service|rpc|wal|object|native|jj|capture)|jj-lib|jj-cli|lapis-ai-runtime|lapis-agent-redaction|kingfisher-scanner|tauri|tokio) v/.test(
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

test("only optional conversation privacy may consume the native redaction library", () => {
  const dependency = { name: "lapis-agent-redaction", optional: true };
  const owner = { name: "jj-store-conversations", dependencies: [dependency] };
  assert.deepEqual(dependencyErrors([owner]), []);
  dependency.optional = false;
  assert.equal(dependencyErrors([owner]).length, 1);
  owner.name = "jj-store-core";
  dependency.optional = true;
  assert.equal(dependencyErrors([owner]).length, 1);
});

test("conversation store capture admits only optional domain interfaces", () => {
  for (const name of [
    "jj-store-core",
    "jj-store-model",
    "jj-agent",
    "jj-collab",
  ]) {
    const dependency = { name, optional: true };
    const owner = {
      name: "jj-store-conversations",
      dependencies: [dependency],
    };
    assert.deepEqual(dependencyErrors([owner]), []);
    dependency.optional = false;
    assert.equal(dependencyErrors([owner]).length, 1);
  }
});

test("store capture links owned interfaces without a concrete provider or runtime", () => {
  const tree = execFileSync(
    "cargo",
    [
      "tree",
      "--locked",
      "--offline",
      "-p",
      "jj-store-conversations",
      "--features",
      "store-capture",
      "--edges",
      "normal",
      "--prefix",
      "none",
      "--format",
      "{p}",
    ],
    { encoding: "utf8" },
  );
  assert.match(tree, /^jj-store-core v/m);
  assert.match(tree, /^jj-agent v/m);
  assert.match(tree, /^jj-collab v/m);
  assert.deepEqual(
    tree
      .split("\n")
      .filter((line) =>
        /^(?:jj-store-(?:service|rpc|wal|object|native|git|jj|capture)|jj-lib|jj-cli|lapis-ai-runtime|lapis-agent-redaction|kingfisher-scanner|tauri|tokio) v/.test(
          line,
        ),
      ),
    [],
  );
});

test("opt-in native privacy retains the scanner without application authority", () => {
  const tree = execFileSync(
    "cargo",
    [
      "tree",
      "--locked",
      "--offline",
      "-p",
      "jj-store-conversations",
      "--features",
      "native-privacy",
      "--edges",
      "normal",
      "--prefix",
      "none",
      "--format",
      "{p}",
    ],
    { encoding: "utf8" },
  );
  assert.match(tree, /^lapis-agent-redaction v/m);
  assert.match(tree, /^kingfisher-scanner v/m);
  assert.deepEqual(
    tree
      .split("\n")
      .filter((line) =>
        /^(?:jj-store-(?:service|rpc|wal|object|native|jj|capture)|jj-lib|jj-cli|lapis-ai-runtime|tauri) v/.test(
          line,
        ),
      ),
    [],
  );
});
