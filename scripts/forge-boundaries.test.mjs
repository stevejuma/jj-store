import test from "node:test";
import assert from "node:assert/strict";
import { dependencyErrors } from "./check-boundaries.mjs";
const packageWith = (name, dependency) => ({
  name,
  dependencies: [{ name: dependency, kind: null }],
});
test("forge adapters cannot become model or WAL dependencies", () => {
  for (const owner of ["jj-store-model", "jj-store-object", "jj-store-wal"]) {
    assert.match(
      dependencyErrors([packageWith(owner, "jj-forge")]).join("\n"),
      /invalid layer/,
    );
  }
  assert.deepEqual(
    dependencyErrors([
      packageWith("jj-store-jj", "jj-forge"),
      packageWith("jj-store-service", "jj-forge"),
    ]),
    [],
  );
  assert.match(
    dependencyErrors([packageWith("jj-forge", "jj-lib")]).join("\n"),
    /outside JJ boundary/,
  );
});
