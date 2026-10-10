import { expect, test } from "vitest";
import {
  completedAgentChanges,
  readAgentWorkspaceChanges,
  type AgentChangeSelection,
} from "../../src/lib/controller/agent-workspace-changes.js";
import type {
  AgentSession,
  AgentEvent,
} from "../../src/lib/controller/agent-execution-types.js";
const selection: AgentChangeSelection = {
  repository: "aa".repeat(16),
  target_id: "target",
  profile: "local_jj_git",
  workspace: "agent-workspace",
  selected_operation: "11".repeat(64),
  base_commit: "22".repeat(20),
  operation: "33".repeat(64),
  commit: "44".repeat(20),
};
const snapshot = {
  version: 1,
  profile: "git_sha1_jj045",
  repository: selection.repository,
  generation: 1,
  operation: selection.operation,
  heads: [selection.operation],
};
const changed = (path: string, binary = false) => ({
  path,
  before: [],
  after: [],
  sources: [],
  content: binary
    ? { status: "unavailable", reason: "binary" }
    : {
        status: "text",
        hunks: [
          {
            kind: "matching",
            before: "same\n",
            after: "same\n",
            before_start: 0,
            after_start: 0,
          },
          {
            kind: "different",
            before: "old\n",
            after: "new\nmore\n",
            before_start: 1,
            after_start: 1,
          },
        ],
      },
});
const response = (value: unknown) =>
  new Response(JSON.stringify(value), {
    headers: { "content-type": "application/json" },
  });
function event(
  type: string,
  value: Record<string, unknown>,
  sequence = 1,
): AgentEvent {
  return {
    sequence,
    observed_at_millis: 1000,
    committed: true,
    payload: { type, requestId: "turn", ...value },
  };
}
test("completed panels admit only the exact confirmed snapshot of the latest completed turn", () => {
  const session = {
    state: "idle",
    repository_bindings: [selection],
  } as AgentSession;
  const events = [
    event("record", { record: { status: "completed" } }),
    event("repository_snapshot", { ...selection, state: "completed" }, 2),
  ];
  expect(completedAgentChanges(session, events)).toEqual([selection]);
  expect(
    completedAgentChanges({ ...session, state: "running" }, events),
  ).toEqual([]);
  expect(
    completedAgentChanges(session, [
      ...events,
      event(
        "record",
        { requestId: "next", record: { status: "completed" } },
        3,
      ),
    ]),
  ).toEqual([]);
  expect(
    completedAgentChanges(session, [
      event("record", { requestId: null, record: { status: "completed" } }),
      event(
        "repository_snapshot",
        { ...selection, requestId: null, state: "completed" },
        2,
      ),
    ]),
  ).toEqual([]);
  expect(
    completedAgentChanges(session, [
      events[0]!,
      event(
        "repository_snapshot",
        { ...selection, state: "outcome_unknown" },
        2,
      ),
    ]),
  ).toEqual([]);
});
test("local changes preserve the base and snapshot pin, count changed lines and retain captured patch bytes", async () => {
  const calls: Record<string, any>[] = [];
  const fetcher: typeof fetch = async (_input, init) => {
    const body = JSON.parse(String(init?.body));
    calls.push(body);
    return response({
      snapshot,
      data: {
        kind: body.request.kind,
        data: {
          from: selection.base_commit,
          to: selection.commit,
          path: body.request.query.path,
          changes: [changed("src/example.ts")],
        },
      },
      next_cursor: null,
    });
  };
  const result = await readAgentWorkspaceChanges(
    selection,
    fetcher,
    new AbortController().signal,
  );
  expect(result).toMatchObject({
    complete: true,
    files: [{ path: "src/example.ts", additions: 2, deletions: 1 }],
  });
  expect(result.files[0]!.patch).toContain("-old\n+new\n+more\n");
  expect(calls).toHaveLength(2);
  expect(
    calls.every(
      (c) =>
        c.operation === selection.operation &&
        c.request.query.from === selection.base_commit &&
        c.request.query.to === selection.commit,
    ),
  ).toBe(true);
  expect(calls[1]!.request.query.content).toBe(true);
});
test("binary changes keep counts unknown instead of presenting zero or inventing text", async () => {
  const fetcher: typeof fetch = async (_input, init) => {
    const body = JSON.parse(String(init?.body));
    return response({
      snapshot,
      data: {
        kind: body.request.kind,
        data: {
          from: selection.base_commit,
          to: selection.commit,
          changes: [changed("image.png", true)],
        },
      },
      next_cursor: null,
    });
  };
  const result = await readAgentWorkspaceChanges(
    selection,
    fetcher,
    new AbortController().signal,
  );
  expect(result.files[0]).toMatchObject({
    additions: null,
    deletions: null,
    unavailable: expect.stringContaining("binary"),
  });
  expect(result.files[0]!.patch).toBeUndefined();
});
test("mismatched repository, operation, generations and comparisons are refused", async () => {
  for (const wrong of [
    { ...snapshot, repository: "bb".repeat(16) },
    { ...snapshot, operation: "55".repeat(64) },
    { ...snapshot, generation: 2 },
  ]) {
    let n = 0;
    const fetcher: typeof fetch = async (_input, init) => {
      const body = JSON.parse(String(init?.body));
      return response({
        snapshot: ++n === 1 ? snapshot : wrong,
        data: {
          kind: body.request.kind,
          data: {
            from: selection.base_commit,
            to: selection.commit,
            changes: [changed("a.ts")],
          },
        },
        next_cursor: null,
      });
    };
    await expect(
      readAgentWorkspaceChanges(
        selection,
        fetcher,
        new AbortController().signal,
      ),
    ).rejects.toMatchObject({ code: "snapshot_mismatch" });
  }
  const fetcher: typeof fetch = async () =>
    response({
      snapshot,
      data: {
        kind: "diff_paths",
        data: { from: "55".repeat(20), to: selection.commit, changes: [] },
      },
      next_cursor: null,
    });
  await expect(
    readAgentWorkspaceChanges(selection, fetcher, new AbortController().signal),
  ).rejects.toMatchObject({ code: "invalid_agent_comparison" });
});
test("pagination and content reads are bounded, with partial totals explicitly marked", async () => {
  let pathPages = 0,
    active = 0,
    peak = 0;
  const fetcher: typeof fetch = async (_input, init) => {
    const body = JSON.parse(String(init?.body));
    const paths = body.request.kind === "diff_paths";
    const page = pathPages;
    if (paths) pathPages++;
    else {
      active++;
      peak = Math.max(peak, active);
      await new Promise((r) => setTimeout(r, 1));
      active--;
    }
    return response({
      snapshot,
      data: {
        kind: body.request.kind,
        data: {
          from: selection.base_commit,
          to: selection.commit,
          changes: paths
            ? Array.from({ length: 50 }, (_, n) =>
                changed(`file-${page * 50 + n}.ts`),
              )
            : [changed(body.request.query.path)],
        },
      },
      next_cursor: paths ? `page-${pathPages}` : null,
    });
  };
  const result = await readAgentWorkspaceChanges(
    selection,
    fetcher,
    new AbortController().signal,
  );
  expect(result.complete).toBe(false);
  expect(result.files).toHaveLength(100);
  expect(pathPages).toBe(2);
  expect(peak).toBeLessThanOrEqual(4);
});
test("native comparisons use the native read boundary and never invoke a mutation", async () => {
  const native = {
    ...selection,
    profile: "native",
    base_commit: "22".repeat(64),
    commit: "44".repeat(64),
  };
  const calls: URL[] = [];
  const fetcher: typeof fetch = async (input, init) => {
    const url = new URL(String(input), "http://localhost");
    calls.push(url);
    expect(init?.method ?? "GET").toBe("GET");
    return response({
      snapshot: {
        repo_id: selection.repository,
        operation_id: selection.operation,
        heads: [selection.operation],
        epoch: 1,
        published_seq: 1,
        manifest_revision: 1,
      },
      data: {
        from: native.base_commit,
        to: native.commit,
        changes: [changed("native.ts")],
      },
      next_cursor: null,
    });
  };
  expect(
    (
      await readAgentWorkspaceChanges(
        native,
        fetcher,
        new AbortController().signal,
      )
    ).files[0]!.additions,
  ).toBe(2);
  expect(calls.map((u) => u.pathname)).toEqual([
    `/v1/repos/${selection.repository}/diff/paths`,
    `/v1/repos/${selection.repository}/diff`,
  ]);
  expect(
    calls.every((u) => u.searchParams.get("at_op") === selection.operation),
  ).toBe(true);
});

test("text retention is bounded independently of exact line counts and aborted reads never start", async () => {
  let calls = 0;
  const fetcher: typeof fetch = async (_input, init) => {
    calls++;
    const body = JSON.parse(String(init?.body));
    const value = changed("large.txt");
    value.content = {
      status: "text",
      hunks: [
        {
          kind: "different",
          before: "",
          after: "x".repeat(2 * 1024 * 1024) + "\n",
          before_start: 0,
          after_start: 0,
        },
      ],
    };
    return response({
      snapshot,
      data: {
        kind: body.request.kind,
        data: {
          from: selection.base_commit,
          to: selection.commit,
          changes: [value],
        },
      },
      next_cursor: null,
    });
  };
  const result = await readAgentWorkspaceChanges(
    selection,
    fetcher,
    new AbortController().signal,
  );
  expect(result.files[0]).toMatchObject({
    additions: 1,
    deletions: 0,
    unavailable: expect.stringContaining("preview budget"),
  });
  expect(result.files[0]!.patch).toBeUndefined();
  const abort = new AbortController();
  abort.abort();
  await expect(
    readAgentWorkspaceChanges(selection, fetcher, abort.signal),
  ).rejects.toThrow();
  expect(calls).toBe(2);
});

test("historical turn cards preserve every completed request's own repository boundaries during later work", async () => {
  const { completedAgentTurns } =
    await import("../../src/lib/controller/agent-workspace-changes.js");
  const session = {
    id: "12".repeat(16),
    state: "running",
    repository_bindings: [selection],
  } as AgentSession;
  const start = event("repository_turn_start", {
    ...selection,
    operation: "10".repeat(64),
    commit: "20".repeat(20),
    state: "completed",
  });
  const first = [
    start,
    event("record", { record: { status: "completed" } }),
    event("repository_snapshot", { ...selection, state: "completed" }),
  ];
  const second = first.map((e) => ({
    ...e,
    payload: {
      ...(e.payload as object),
      requestId: "second",
      ...((e.payload as any).type === "repository_turn_start"
        ? { commit: selection.commit }
        : (e.payload as any).type === "repository_snapshot"
          ? { commit: "60".repeat(20) }
          : {}),
    },
  }));
  const turns = completedAgentTurns(session, [
    ...first,
    ...second,
    event("record", { requestId: "failed", record: { status: "failed" } }),
  ]);
  expect(turns.map((t) => [t.requestId, t.base_commit, t.commit])).toEqual([
    ["turn", "20".repeat(20), selection.commit],
    ["second", selection.commit, "60".repeat(20)],
  ]);
  expect(turns[0]!.beforeOperation).toBe("10".repeat(64));
  expect(
    completedAgentTurns(
      session,
      first.filter((e) => (e.payload as any).type !== "repository_turn_start"),
    ),
  ).toEqual([]);
  expect(
    completedAgentTurns(
      session,
      first.map((e) => ({
        ...e,
        payload: {
          ...(e.payload as object),
          ...((e.payload as any).type === "repository_turn_start"
            ? { repository: "bb".repeat(16) }
            : {}),
        },
      })),
    ),
  ).toEqual([]);
});
