import { expect, test, vi } from "vitest";
import { createWorkflowHistory } from "../../src/lib/ui/store-workflows/workflow-history.svelte.js";
import { createWorkflows } from "../../src/lib/controller/workflows.svelte.js";
import { home, workflowFixture } from "../../../catalog/fixtures/workflows.js";
import type { WorkflowSelection } from "../../src/lib/controller/workflows-route-codec.js";

async function fixture() {
  const transport = workflowFixture(),
    catalog = await transport.page({});
  const workflow = catalog.workflows[0]!,
    page = vi.fn(transport.page);
  const work = createWorkflows({ ...transport, page });
  const group = {
    home,
    workflow,
    work,
    rows: [],
    page: null,
    loading: false,
    error: "",
  } as Parameters<ReturnType<typeof createWorkflowHistory>["read"]>[0];
  let selection: WorkflowSelection = { home, run: "51".repeat(16), at: "1" };
  const history = createWorkflowHistory(
    () => [],
    () => [[home, work]],
    () => selection,
  );
  return {
    history,
    work,
    group,
    page,
    catalog,
    select: (value: WorkflowSelection) => {
      selection = value;
    },
  };
}
test("run continuation stays pinned and rejects changed roots without losing loaded rows", async () => {
  const f = await fixture(),
    first = await workflowFixture().page({ workflow: f.group.workflow.id });
  f.page.mockResolvedValueOnce({ ...first, next: "next-runs" });
  await f.history.read(f.group);
  f.page.mockResolvedValueOnce({ ...first, root: "99".repeat(32) });
  await f.history.read(f.group, true);
  expect(f.page.mock.calls.at(-1)?.[0]).toMatchObject({
    at: 1,
    after: "next-runs",
    workflow: f.group.workflow.id,
  });
  expect(f.group.rows).toEqual(first.runs);
  expect(f.group.page?.next).toBe("next-runs");
  expect(f.group.error).toContain("snapshot changed");
});
test("run evidence continuation preserves publication and refuses another attempt", async () => {
  const f = await fixture();
  f.select({ home, run: "51".repeat(16), at: "1", attempt: "52".repeat(16) });
  const first = await workflowFixture().page({ run: "51".repeat(16) });
  f.page.mockResolvedValueOnce({
    ...first,
    evidence: first.evidence.slice(0, 1),
    next: "evidence-next",
  });
  await f.history.inspect();
  f.page.mockResolvedValueOnce({
    ...first,
    evidence: [{ ...first.evidence[1]!, attempt: "99".repeat(16) }],
  });
  await f.history.inspect(true);
  expect(f.page.mock.calls.at(-1)?.[0]).toMatchObject({
    at: 1,
    after: "evidence-next",
    attempt: "52".repeat(16),
  });
  expect(f.history.detail?.evidence).toEqual(first.evidence.slice(0, 1));
  expect(f.history.detailError).toContain("snapshot changed");
});
test("stale run responses and wrong publication pins never replace the requested detail", async () => {
  const f = await fixture(),
    first = await workflowFixture().page({ run: "51".repeat(16) });
  let release!: (value: typeof first) => void;
  f.page.mockImplementationOnce(
    () =>
      new Promise((resolve) => {
        release = resolve;
      }),
  );
  const pending = f.history.inspect();
  f.select({ home, run: "53".repeat(16) });
  release(first);
  await pending;
  expect(f.history.detail).toBeNull();
  const g = await fixture();
  g.page.mockResolvedValueOnce({ ...first, position: 2 });
  await g.history.inspect();
  expect(g.history.detail).toBeNull();
  expect(g.history.detailError).toContain("snapshot changed");
});
test("catalog pagination stays pinned and a read refresh preserves an unknown command identity", async () => {
  const f = await fixture();
  f.page.mockResolvedValueOnce({ ...f.catalog, next: "catalog-next" });
  await f.work.refresh();
  f.page.mockResolvedValueOnce({ ...f.catalog, position: 2 });
  await f.work.more();
  expect(f.page.mock.calls.at(-1)?.[0]).toMatchObject({
    at: 1,
    after: "catalog-next",
  });
  expect(f.work.state.page?.position).toBe(1);
  expect(f.work.state.readError).toContain("snapshot changed");
  const unknown = createWorkflows(workflowFixture(true));
  await unknown.submit({ type: "cancel", run: "51".repeat(16) });
  const bytes = JSON.stringify(unknown.state.pending);
  await unknown.refresh();
  expect(JSON.stringify(unknown.state.pending)).toBe(bytes);
  expect(await unknown.submit({ type: "cancel", run: "53".repeat(16) })).toBe(
    false,
  );
  expect(JSON.stringify(unknown.state.pending)).toBe(bytes);
});
