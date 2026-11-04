import assert from "node:assert/strict";
import test from "node:test";
import { uiDevelopmentPresentation } from "./dev-ui-config.mjs";

test("desktop browser mode is explicit and backend-only restarts retain presentation", () => {
  assert.equal(uiDevelopmentPresentation("start"), "browser");
  assert.equal(uiDevelopmentPresentation("desktop"), "macos");
  assert.equal(
    uiDevelopmentPresentation("backend", { presentation: "macos" }),
    "macos",
  );
  assert.equal(uiDevelopmentPresentation("backend", {}), "browser");
  assert.equal(
    uiDevelopmentPresentation("start", { presentation: "macos" }),
    "browser",
  );
  assert.throws(
    () => uiDevelopmentPresentation("unknown"),
    /unknown UI development action/,
  );
});
