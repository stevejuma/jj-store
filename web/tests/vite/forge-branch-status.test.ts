import { expect, it } from "vitest";
import {
  forgeBranchCheckStatus,
  forgeBranchPushStatus,
} from "../../src/lib/controller/forge/branch-status.js";
import type {
  ForgeCheck,
  ForgeResponse,
} from "../../src/lib/controller/forge/types.js";
const pr = { sha: "a".repeat(40), state: "open" as const };
function checks(
  states: ForgeCheck["state"][],
  complete = true,
): NonNullable<ForgeResponse["checks"]> {
  return {
    sha: pr.sha,
    complete,
    checks: states.map((state, index) => ({
      id: String(index),
      name: "Check",
      state,
      sha: pr.sha,
      url: null,
      run_id: null,
      log_id: null,
    })),
  };
}

it("compares exact published heads without claiming ancestry or an ahead count", () => {
  expect(forgeBranchPushStatus(pr.sha, pr)).toBe("Nothing to push");
  expect(forgeBranchPushStatus(pr.sha.toUpperCase(), pr)).toBe(
    "Nothing to push",
  );
  expect(forgeBranchPushStatus("b".repeat(40), pr)).toBe("Changes to push");
});
it("missing, native, malformed, closed and uncertain heads never claim the branch is pushed", () => {
  for (const head of [undefined, "short-id", "a".repeat(64), "g".repeat(40)])
    expect(forgeBranchPushStatus(head, pr)).toBe("Push status unknown");
  expect(forgeBranchPushStatus(pr.sha, undefined)).toBe("Push status unknown");
  expect(forgeBranchPushStatus(pr.sha, { ...pr, state: "closed" })).toBe(
    "Push status unknown",
  );
  expect(forgeBranchPushStatus(pr.sha, pr, true)).toBe("Push status unknown");
});
it.each([
  [["passed"], "Passed", "passed"],
  [["passed", "skipped"], "Passed", "passed"],
  [["failed", "running"], "Failed", "failed"],
  [["running", "passed"], "Running", "running"],
  [["queued", "passed"], "Running", "running"],
  [["skipped"], "Skipped", "skipped"],
  [[], "No checks", "unknown"],
  [["unknown"], "Needs attention", "unknown"],
  [["manual", "passed"], "Needs attention", "unknown"],
  [["cancelled", "passed"], "Needs attention", "unknown"],
] as [ForgeCheck["state"][], string, string][])(
  "published checks %j render %s",
  (states, label, state) => {
    expect(forgeBranchCheckStatus(pr, checks(states))).toMatchObject({
      label,
      state,
    });
  },
);
it("missing or incomplete check observations cannot produce success", () => {
  expect(forgeBranchCheckStatus(pr, undefined).label).toBe("Unknown");
  expect(forgeBranchCheckStatus(undefined, checks(["passed"])).label).toBe(
    "Unknown",
  );
  expect(forgeBranchCheckStatus(pr, checks(["passed"], false)).label).toBe(
    "Unknown",
  );
  expect(forgeBranchCheckStatus(pr, checks(["passed"]), false).label).toBe(
    "Unavailable",
  );
});
it("stale outer or per-check revisions never produce a passed badge", () => {
  expect(forgeBranchCheckStatus(pr, checks(["passed"]), true, true).label).toBe(
    "Refresh needed",
  );
  const outer = { ...checks(["passed"]), sha: "b".repeat(40) };
  expect(forgeBranchCheckStatus(pr, outer).label).toBe("Refresh needed");
  const inner = checks(["passed"]);
  inner.checks[0]!.sha = "b".repeat(40);
  expect(forgeBranchCheckStatus(pr, inner).label).toBe("Refresh needed");
});
