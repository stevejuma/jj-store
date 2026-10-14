import { expect, test } from "vitest";
import {
  workflowFixture,
  home,
  code,
} from "../../../catalog/fixtures/workflows.js";
import {
  matchesWorkflowRun,
  runDuration,
  workflowRepositoryGroups,
  workflowRunSelection,
} from "../../src/lib/ui/store-workflows/workflow-history-presentation.js";
async function fixture() {
  const source = workflowFixture(false, home, false, false, true),
    catalog = await source.page({});
  const workflow = catalog.workflows[0]!,
    page = await source.page({ workflow: workflow.id });
  return {
    row: {
      home,
      workflow,
      run: page.runs[0]!,
      position: 18446744073709551615n,
    },
    catalog,
  };
}
test("workflow navigation groups source repositories while retaining distinct metadata homes", async () => {
  const { catalog } = await fixture();
  const entries = catalog.workflows.map((workflow) => ({ home, workflow }));
  entries.push({ home: code, workflow: catalog.workflows[0]! });
  const groups = workflowRepositoryGroups(entries, (id) =>
    id === home ? "Automation" : "Parser",
  );
  expect(groups.map((group) => [group.name, group.entries.length])).toEqual([
    ["Automation", 1],
    ["Parser", 3],
  ]);
  expect(
    groups[1]!.entries
      .filter((entry) => entry.workflow.id === catalog.workflows[0]!.id)
      .map((entry) => entry.home),
  ).toEqual([home, code]);
});
test("run filters use recorded requester, trigger and bookmark rather than the current definition", async () => {
  const { row } = await fixture();
  expect(
    matchesWorkflowRun(row, {
      author: row.run.requester,
      trigger: "manual",
      bookmark: "main",
      state: "completed",
      scope: home,
      workflow: row.workflow.id,
      source: code,
      q: "linux-builder",
    }),
  ).toBe(true);
  for (const filter of [
    { author: home },
    { trigger: "event" as const },
    { bookmark: "MAIN" },
    { state: "failed" as const },
    { source: home },
    { scope: code },
    { workflow: home },
    { q: "missing" },
  ])
    expect(matchesWorkflowRun(row, filter)).toBe(false);
  expect(row.workflow.definition.trigger.on).toBe("bookmark_changed");
});
test("duration uses the latest reported attempt and never invents missing timestamps", async () => {
  const { row } = await fixture(),
    run = row.run,
    attempt = run.attempts[0]!;
  expect(runDuration(run, 0)).toBe("0s");
  attempt.completed_at = Number(attempt.started_at) + 90500;
  expect(runDuration(run, 0)).toBe("1m 30s");
  attempt.completed_at = null;
  expect(runDuration(run, Date.now())).toBe("—");
  attempt.status = "running";
  expect(runDuration(run, Number(attempt.started_at) + 3600000)).toBe("1h 0m");
  attempt.status = "unknown";
  expect(runDuration(run, Date.now())).toBe("—");
  attempt.started_at = null;
  expect(runDuration(run, Date.now())).toBe("—");
  run.attempts = [];
  expect(runDuration(run, Date.now())).toBe("—");
});
test("terminal row links preserve full publication pins while active runs follow current evidence", async () => {
  const { row } = await fixture();
  expect(workflowRunSelection(row)).toMatchObject({
    home,
    run: row.run.id,
    at: "18446744073709551615",
    attempt: undefined,
  });
  row.run.attempts[0]!.status = "running";
  expect(workflowRunSelection(row).at).toBeUndefined();
});
