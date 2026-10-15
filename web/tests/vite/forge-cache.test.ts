import { expect, it, vi } from "vitest";
import { ForgeController } from "../../src/lib/controller/forge/controller.svelte.js";
import { ForgeReadCache } from "../../src/lib/controller/forge/read-cache.js";
import { workspaceForgeTransport } from "../../../catalog/fixtures/forge/workspace-transport.js";
import { checkLabel } from "../../src/lib/controller/forge/check-timing.js";

it("reuses revision-bound reads across request IDs, isolates copies and refreshes manually", async () => {
  const f = workspaceForgeTransport();
  const transport = vi.fn(f.transport);
  const forge = new ForgeController(transport);
  forge.data = f.initial;
  forge.selected = f.initial.pull_requests[1];
  const action = {
    kind: "pr",
    request_id: "first",
    number: 42,
    action: { kind: "checks" },
  } as const;
  const first = await forge.call(action);
  const second = await forge.call({ ...action, request_id: "second" });
  expect(second.checks).toEqual(first.checks);
  second.checks!.checks[0]!.name = "Changed locally";
  expect((await forge.call(action)).checks!.checks[0]!.name).not.toBe(
    "Changed locally",
  );
  expect(transport).toHaveBeenCalledTimes(1);
  await forge.refreshChecks();
  expect(transport).toHaveBeenCalledTimes(2);
  forge.data!.pull_requests[1]!.sha = "b".repeat(40);
  await forge.call(action);
  expect(transport).toHaveBeenCalledTimes(3);
  forge.token = "transient-new-credential";
  await forge.call(action);
  expect(transport).toHaveBeenCalledTimes(4);
  forge.close();
});
it("has no background refresh and first-load ensure is idempotent", async () => {
  const f = workspaceForgeTransport();
  const transport = vi.fn(f.transport);
  const forge = new ForgeController(transport);
  await forge.ensure();
  await forge.ensure();
  expect(transport).toHaveBeenCalledTimes(1);
  vi.useFakeTimers();
  try {
    await vi.advanceTimersByTimeAsync(180_000);
    expect(transport).toHaveBeenCalledTimes(1);
  } finally {
    vi.useRealTimers();
  }
  await forge.refresh();
  expect(transport).toHaveBeenCalledTimes(2);
  forge.close();
});
it("does not cache mutations, previews, recovery or journal proof fields", () => {
  const f = workspaceForgeTransport();
  const cache = new ForgeReadCache();
  for (const action of [
    { kind: "status" },
    { kind: "preview", bookmark: "topic", draft: true },
    { kind: "recover", request_id: "same" },
    { kind: "operation_status", request_id: "same" },
    {
      kind: "pr",
      number: 42,
      request_id: "same",
      action: { kind: "cancel", run: "42" },
    },
    {
      kind: "pr",
      number: 42,
      request_id: "same",
      action: { kind: "merge_status", receipt: "queue" },
    },
  ] as const)
    expect(cache.key(action, "sha")).toBeUndefined();
  cache.put("detail", {
    ...f.initial,
    merges: { "42": { state: "merged", receipt: "receipt", sha: null } },
  });
  expect(cache.get("detail")?.merges).toEqual({});
  for (let i = 0; i < 65; i++) cache.put(`check-${i}`, f.initial);
  expect(cache.get("check-0")).toBeUndefined();
  expect(cache.get("check-64")).toBeDefined();
});
it("keeps repository caches separate and rejects a late reply after account change", async () => {
  const f = workspaceForgeTransport();
  const transport = vi.fn(f.transport);
  const a = new ForgeController(transport),
    b = new ForgeController(transport);
  a.data = f.initial;
  b.data = f.initial;
  await a.call({ kind: "pull_request", number: 42 });
  await b.call({ kind: "pull_request", number: 42 });
  expect(transport).toHaveBeenCalledTimes(2);
  let finish!: () => void;
  const c = new ForgeController(async () => {
    await new Promise<void>((r) => {
      finish = r;
    });
    return f.initial;
  });
  c.data = f.initial;
  const pending = c.call({ kind: "pull_request", number: 42 });
  await Promise.resolve();
  c.token = "another-account";
  finish();
  await expect(pending).rejects.toThrow("Forge access changed");
  a.close();
  b.close();
  c.close();
});
it("starts checks collapsed on PR opening and preserves expansion through refresh", async () => {
  const f = workspaceForgeTransport();
  const forge = new ForgeController(f.transport);
  forge.data = f.initial;
  forge.select("feature/api");
  expect(forge.checksView.expanded).toBe(false);
  await vi.waitFor(() => expect(forge.checks).toBeDefined());
  forge.checksView.expanded = true;
  await forge.refresh();
  expect(forge.checksView.expanded).toBe(true);
  forge.select("feature/model");
  expect(forge.checksView.expanded).toBe(false);
  forge.close();
});
it("formats only valid completed timing facts", () => {
  const base = workspaceForgeTransport().ci["feature/api"]!.checks[0]!;
  const check = {
    ...base,
    state: "failed" as const,
    started_at: "2026-10-15T10:00:00Z",
    completed_at: "2026-10-15T10:00:05Z",
  };
  expect(checkLabel(check)).toBe("Failing after 5s");
  expect(checkLabel({ ...check, state: "passed" })).toBe("Successful in 5s");
  expect(checkLabel({ ...check, state: "running" })).toBe("In progress");
  for (const completed_at of [null, "invalid", "2026-10-15T09:00:00Z"])
    expect(checkLabel({ ...check, completed_at })).toBe("Failing");
});

it("successive saves advance the expected body from the mutation reply without polling", async () => {
  const f = workspaceForgeTransport();
  const transport = vi.fn(f.transport);
  const forge = new ForgeController(transport);
  forge.data = f.initial;
  forge.select("feature/api");
  await vi.waitFor(() => expect(forge.checks).toBeDefined());
  forge.body = "First saved body";
  forge.draftDirty = true;
  await forge.edit();
  expect(forge.selected?.body).toBe("First saved body");
  forge.body = "Second saved body";
  forge.draftDirty = true;
  await forge.edit();
  const edits = transport.mock.calls
    .map(([r]) => r.action)
    .filter((a) => a.kind === "pr" && a.action.kind === "edit");
  expect(edits).toHaveLength(2);
  expect(
    edits[1]?.kind === "pr" &&
      edits[1].action.kind === "edit" &&
      edits[1].action.expected_body,
  ).toBe("First saved body");
  expect(transport.mock.calls.some(([r]) => r.action.kind === "status")).toBe(
    false,
  );
  forge.close();
});

it("a stale account reply leaves checks untouched and publishes a redacted toast", async () => {
  const { NotificationService } =
    await import("../../src/lib/notifications/notification-service.svelte.js");
  const service = new NotificationService();
  const f = workspaceForgeTransport();
  let finish!: () => void;
  const forge = new ForgeController(async () => {
    await new Promise<void>((resolve) => {
      finish = resolve;
    });
    return f.initial;
  });
  forge.data = f.initial;
  forge.notifications = { service, scope: "repository" };
  forge.select("feature/api");
  await vi.waitFor(() => expect(finish).toBeTypeOf("function"));
  forge.token = "secret-during-switch";
  finish();
  await vi.waitFor(() => expect(service.records).toHaveLength(1));
  expect(service.records[0]?.title).toBe("Forge action failed");
  expect(JSON.stringify(service.records)).not.toContain("secret-during-switch");
  expect(forge.checks).toBeUndefined();
  expect(forge.pending).toBeUndefined();
  forge.close();
  service.dispose();
});
it("a failed read is not cached and an explicit next read can succeed", async () => {
  const f = workspaceForgeTransport();
  let calls = 0;
  const forge = new ForgeController(async () => {
    if (++calls === 1) throw new Error("access denied");
    return f.initial;
  });
  forge.data = f.initial;
  await expect(
    forge.call({ kind: "pull_request", number: 42 }),
  ).rejects.toThrow("access denied");
  await forge.call({ kind: "pull_request", number: 42 });
  await forge.call({ kind: "pull_request", number: 42 });
  expect(calls).toBe(2);
  forge.close();
});

it("account changes during an external effect retain its original uncertain intent", async () => {
  const f = workspaceForgeTransport();
  let finish!: () => void;
  let calls = 0;
  const forge = new ForgeController(async () => {
    calls++;
    await new Promise<void>((resolve) => {
      finish = resolve;
    });
    return f.initial;
  });
  forge.data = f.initial;
  const action = {
    kind: "pr",
    number: 42,
    request_id: "original-account-intent",
    action: { kind: "cancel", run: "42" },
  } as const;
  const pending = forge.run(action);
  await vi.waitFor(() => expect(finish).toBeTypeOf("function"));
  forge.token = "other-account";
  finish();
  await pending;
  expect(forge.pending).toEqual(action);
  await forge.run({ ...action, request_id: "replacement" });
  expect(calls).toBe(1);
  forge.close();
});
