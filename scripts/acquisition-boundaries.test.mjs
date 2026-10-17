import test from "node:test";
import assert from "node:assert/strict";
import { dependencyErrors } from "./check-boundaries.mjs";

test("source acquisition stays outside core authority and is admitted only for capture", () => {
  assert.deepEqual(
    dependencyErrors([
      {
        name: "jj-store-capture",
        dependencies: [{ name: "lapis-ai-capture" }],
      },
    ]),
    [],
  );
  for (const name of [
    "jj-agent",
    "jj-store-core",
    "jj-store-model",
    "jj-store-wal",
    "jj-store-server",
  ]) {
    assert.equal(
      dependencyErrors([
        {
          name,
          dependencies: [{ name: "lapis-ai-capture" }],
        },
      ]).length,
      1,
    );
  }
  assert.equal(
    dependencyErrors([
      {
        name: "jj-store-capture",
        dependencies: [{ name: "lapis-ai-runtime" }],
      },
    ]).length,
    1,
  );
});
