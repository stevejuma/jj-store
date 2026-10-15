import { describe, it, expect, vi } from "vitest";
import { ForgeController } from "../../src/lib/controller/forge/controller.svelte.js";
import type {
  ForgePr,
  ForgeResponse,
} from "../../src/lib/controller/forge/types.js";
const response = (): ForgeResponse => ({
  notice: null,
  connection: null,
  capabilities: {
    edit: false,
    draft: false,
    merge: false,
    auto_merge: false,
    checks: false,
    logs: false,
    rerun: false,
    rerun_all: false,
    merge_methods: [],
    cancel: false,
    native_stacks: false,
  },
  pull_requests: [],
  plan: null,
  restack: null,
  receipt: null,
  checks: null,
  log: null,
  merge: null,
  merges: {},
  operation: null,
});
describe("forge controller", () => {
  it("retains the exact pending mutation after an unknown transport outcome", async () => {
    const calls: unknown[] = [];
    const controller = new ForgeController(async (request) => {
      calls.push(request);
      throw new Error("network outcome unknown");
    });
    const action = {
      kind: "pr",
      request_id: "original",
      number: 7,
      action: { kind: "cancel", run: "10" },
    } as const;
    await controller.run(action);
    expect(controller.pending).toEqual(action);
    await controller.run({ ...action, request_id: "replacement" });
    expect(calls).toHaveLength(1);
    controller.close();
  });
  it("read failures do not create a pending write", async () => {
    const controller = new ForgeController(async () => {
      throw new Error("access denied");
    });
    await controller.run({ kind: "status" });
    expect(controller.pending).toBeUndefined();
    expect(controller.stale).toBe(true);
    controller.close();
  });
  it("clears a completed pending operation only after observing its receipt", async () => {
    let calls = 0;
    const controller = new ForgeController(async () => {
      if (++calls === 1) throw new Error("outcome unknown");
      return {
        ...response(),
        operation: {
          id: "original",
          number: 7,
          action: { kind: "cancel", run: "10" },
          outcome: "complete",
          merge: null,
          merges: {},
        },
      };
    });
    await controller.run({
      kind: "pr",
      request_id: "original",
      number: 7,
      action: { kind: "cancel", run: "10" },
    });
    await controller.checkPending();
    expect(controller.pending).toBeUndefined();
    controller.close();
  });
});

const pr = (number: number): ForgePr => ({
  number,
  url: `https://github.com/team/repo/pull/${number}`,
  title: `PR ${number}`,
  body: "Description",
  head: `feature-${number}`,
  sha: "a".repeat(40),
  base: "main",
  draft: false,
  state: "open",
  mergeable: true,
  native_stack: null,
  updated_at: "",
});
const connection = {
  provider: "github",
  api_url: "https://api.github.com",
  repository: "team/repo",
  source_repository: "team/repo",
  remote: "origin",
  target_remote: "origin",
  target: "main",
  native_stacks: false,
} as const;
it("partial PR responses preserve the inventory and other PR merge receipts", async () => {
  const controller = new ForgeController(async () => ({
    ...response(),
    connection,
    pull_requests: [pr(1)],
    merge: { state: "enqueued", receipt: "queue-1", sha: null },
  }));
  controller.data = {
    ...response(),
    connection,
    pull_requests: [pr(1), pr(2)],
  };
  controller.selected = pr(1);
  await controller.act({ kind: "merge_status", receipt: "queue-1" });
  expect(controller.data.pull_requests.map((p) => p.number)).toEqual([1, 2]);
  controller.select("feature-2");
  expect(controller.merge).toBeUndefined();
  controller.select("feature-1");
  expect(controller.merge?.receipt).toBe("queue-1");
  controller.close();
});
it("status refresh retains and polls an enqueued merge without resubmitting", async () => {
  const calls: string[] = [];
  const controller = new ForgeController(async (request) => {
    calls.push(
      request.action.kind === "pr"
        ? request.action.action.kind
        : request.action.kind,
    );
    return request.action.kind === "pr"
      ? {
          ...response(),
          connection,
          pull_requests: [pr(1)],
          merge: { state: "merged", receipt: "queue-1", sha: "a".repeat(40) },
        }
      : { ...response(), connection, pull_requests: [pr(1), pr(2)] };
  });
  controller.data = {
    ...response(),
    connection,
    pull_requests: [pr(1), pr(2)],
  };
  controller.selected = pr(1);
  controller.merges = {
    1: { state: "enqueued", receipt: "queue-1", sha: null },
  };
  await controller.refresh();
  expect(controller.merge?.state).toBe("merged");
  expect(calls).toEqual(["status", "merge_status"]);
  controller.close();
});

it.each([
  new SyntaxError("Unexpected end of JSON input"),
  new Error("request deadline exceeded"),
])(
  "retains the original write after a malformed or timed-out reply: %s",
  async (error) => {
    const controller = new ForgeController(async () => {
      throw error;
    });
    const action = {
      kind: "pr",
      request_id: "original",
      number: 7,
      action: { kind: "cancel", run: "10" },
    } as const;
    await controller.run(action);
    expect(controller.pending).toEqual(action);
    controller.close();
  },
);

it("loads selected PR checks automatically and discards a late previous selection", async () => {
  const pending = new Map<number, (result: ForgeResponse) => void>();
  const first = pr(1),
    second = { ...pr(2), sha: "b".repeat(40) };
  const controller = new ForgeController(async ({ action }) => {
    if (action.kind === "pull_request")
      return {
        ...response(),
        pull_requests: [action.number === 1 ? first : second],
      };
    if (action.kind === "pr" && action.action.kind === "checks")
      return new Promise<ForgeResponse>((resolve) =>
        pending.set(Number(action.number), resolve),
      );
    return response();
  });
  controller.data = {
    ...response(),
    connection,
    capabilities: { ...response().capabilities, checks: true },
    pull_requests: [first, second],
  };
  controller.select(first.head);
  await vi.waitFor(() => expect(pending.has(1)).toBe(true));
  controller.select(second.head);
  await vi.waitFor(() => expect(pending.has(2)).toBe(true));
  const checks = (sha: string, name: string): ForgeResponse => ({
    ...response(),
    checks: {
      sha,
      complete: true,
      checks: [
        {
          id: name,
          name,
          sha,
          state: "passed",
          url: null,
          run_id: null,
          log_id: null,
        },
      ],
    },
  });
  pending.get(2)!(checks(second.sha, "Current revision"));
  await vi.waitFor(() => expect(controller.checks?.sha).toBe(second.sha));
  pending.get(1)!(checks(first.sha, "Old selection"));
  await Promise.resolve();
  await Promise.resolve();
  expect(controller.selected?.number).toBe(2);
  expect(controller.checks?.checks[0]?.name).toBe("Current revision");
  expect(controller.pending).toBeUndefined();
  expect(controller.busy).toBe(false);
  controller.close();
});

it("does not label stale branch checks as passed", () => {
  const controller = new ForgeController(async () => response());
  const selected = pr(1);
  controller.data = { ...response(), connection, pull_requests: [selected] };
  controller.ci = {
    [selected.head]: {
      sha: "f".repeat(40),
      complete: true,
      checks: [
        {
          id: "old",
          name: "Old check",
          sha: "f".repeat(40),
          state: "passed",
          url: null,
          run_id: null,
          log_id: null,
        },
      ],
    },
  };
  expect(controller.checkSummary(selected.head)).toBe("Checks need refresh");
  controller.ci[selected.head]!.sha = selected.sha;
  expect(controller.checkSummary(selected.head)).toBe("Checks need refresh");
  controller.close();
});
