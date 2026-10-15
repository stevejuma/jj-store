import type { ForgePr, ForgeResponse } from "./types.js";
type PrHead = Pick<ForgePr, "sha" | "state">;
type Checks = NonNullable<ForgeResponse["checks"]>;
export type BranchCheckStatus = {
  state: "passed" | "failed" | "running" | "unknown" | "skipped";
  label: string;
  description: string;
};

/** Observed head equality is display-only; it supplies neither ancestry nor push admission. */
export function forgeBranchPushStatus(
  localHead: string | undefined,
  pr: PrHead | undefined,
  uncertain = false,
) {
  const sha1 = /^[0-9a-f]{40}$/iu;
  if (
    uncertain ||
    !localHead ||
    !sha1.test(localHead) ||
    pr?.state !== "open" ||
    !sha1.test(pr.sha)
  )
    return "Push status unknown";
  return localHead.toLowerCase() === pr.sha.toLowerCase()
    ? "Nothing to push"
    : "Changes to push";
}

/** A badge describes checks for the published PR head, including uncertain observations. */
export function forgeBranchCheckStatus(
  pr: PrHead | undefined,
  checks: Checks | undefined,
  available = true,
  stale = false,
): BranchCheckStatus {
  const unknown = (label: string, description: string): BranchCheckStatus => ({
    state: "unknown",
    label,
    description,
  });
  if (!available) return unknown("Unavailable", "Checks unavailable");
  if (stale) return unknown("Refresh needed", "Checks need refresh");
  if (!pr || !checks?.complete) return unknown("Unknown", "Checks unknown");
  if (
    checks.sha !== pr.sha ||
    checks.checks.some((check) => check.sha !== pr.sha)
  )
    return unknown("Refresh needed", "Checks need refresh");
  if (!checks.checks.length) return unknown("No checks", "No checks reported");
  if (checks.checks.some((check) => check.state === "failed"))
    return { state: "failed", label: "Failed", description: "Checks failed" };
  if (
    checks.checks.some((check) => ["queued", "running"].includes(check.state))
  )
    return {
      state: "running",
      label: "Running",
      description: "Checks running",
    };
  if (
    checks.checks.some((check) =>
      ["unknown", "manual", "cancelled"].includes(check.state),
    )
  )
    return unknown("Needs attention", "Checks need attention");
  return checks.checks.some((check) => check.state === "passed")
    ? { state: "passed", label: "Passed", description: "Checks passed" }
    : { state: "skipped", label: "Skipped", description: "Checks skipped" };
}
