import type { ForgeCheck } from "./types.js";
export class ChecksPresentation {
  grouped = $state(true);
  expanded = $state(true);
  collapsed = $state<Record<string, boolean>>({});
  scrollTop = 0;
}
export function checkGroups(checks: readonly ForgeCheck[]) {
  return [
    { id: "failed", label: "failing", states: ["failed"] },
    { id: "running", label: "in progress", states: ["running", "queued"] },
    {
      id: "attention",
      label: "needing attention",
      states: ["manual", "unknown"],
    },
    { id: "passed", label: "successful", states: ["passed"] },
    {
      id: "other",
      label: "skipped or cancelled",
      states: ["skipped", "cancelled"],
    },
  ]
    .map((group) => ({
      ...group,
      checks: checks.filter((check) => group.states.includes(check.state)),
    }))
    .filter((group) => group.checks.length);
}

export function summarizeChecks(
  owner: import("./controller.svelte.js").ForgeController,
  branch: string,
) {
  const checks = owner.ci[branch];
  if (!checks || !checks.complete) return "Checks unknown";
  const pr = owner.pr(branch);
  if (
    !pr ||
    checks.sha !== pr.sha ||
    checks.checks.some((c) => c.sha !== pr.sha)
  )
    return "Checks need refresh";
  if (!checks.checks.length) return "No checks";
  if (checks.checks.some((c) => c.state === "failed")) return "Checks failed";
  if (checks.checks.some((c) => ["queued", "running"].includes(c.state)))
    return "Checks running";
  if (
    checks.checks.some((c) =>
      ["unknown", "manual", "cancelled"].includes(c.state),
    )
  )
    return "Checks need attention";
  return "Checks passed";
}
