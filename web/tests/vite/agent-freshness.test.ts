import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import type {
  AgentStatus,
  AgentSession,
} from "../../src/lib/controller/agent-execution-types.js";
import { watchAgentDiscussion } from "../../src/lib/controller/agent-discussion-refresh.js";
import { ApplicationFreshness } from "../../src/lib/controller/application-freshness.js";

test("dirty failed settings reads stop while the advertised feed is offline", async () => {
  const f = await fixture();
  vi.useFakeTimers();
  try {
    f.intercept.read = (path) =>
      path.endsWith("/settings")
        ? Promise.resolve(
            Response.json({ code: "unavailable" }, { status: 503 }),
          )
        : undefined;
    f.notice("agent_settings");
    await vi.advanceTimersByTimeAsync(250);
    const count = f.paths.filter((p) => p.endsWith("/settings")).length;
    expect(count).toBe(2);
    f.disconnect();
    await vi.advanceTimersByTimeAsync(10);
    expect(f.freshness.healthy).toBe(false);
    await vi.advanceTimersByTimeAsync(4000);
    expect(f.paths.filter((p) => p.endsWith("/settings"))).toHaveLength(count);
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("pending compute configuration observes events without status polling", async () => {
  const f = await fixture();
  let computeReads = 0,
    completed = false;
  const value = { version: 1, settings: { revision: 0 }, state: "stopped" };
  f.intercept.read = (path) => {
    if (path.endsWith("/compute"))
      return Promise.resolve(
        Response.json(computeReads++ === 0 ? { state: "prepared" } : value),
      );
    if (path.includes("/outcome?"))
      return Promise.resolve(
        Response.json({ state: completed ? "completed" : "prepared" }),
      );
  };
  f.manager.compute.value = value as never;
  try {
    const command = f.manager.compute.command({
      action: "configure",
      settings: {},
    } as never);
    await vi.waitFor(() => expect(computeReads).toBe(2));
    await new Promise((resolve) => setTimeout(resolve, 250));
    expect(computeReads).toBe(2);
    f.notice("agent_settings");
    await new Promise((resolve) => setTimeout(resolve, 250));
    expect(computeReads).toBe(2);
    completed = true;
    f.notice("agent_execution");
    expect(await command).toBe(true);
    expect(computeReads).toBe(3);
  } finally {
    f.close();
  }
});

async function fixture() {
  let stream!: ReadableStreamDefaultController<Uint8Array>;
  let revision = 0;
  const paths: string[] = [];
  const status: AgentStatus = {
    version: 1,
    runtime: "ready",
    revision: 0,
    setups: [],
    sessions: [],
    runners: [],
    conversations: [],
  };
  const events: { sequence: number; committed: boolean; payload: unknown }[] =
    [];
  const intercept: { read?: (path: string) => Promise<Response> | undefined } =
    {};
  const fetcher: typeof fetch = async (input) => {
    const path = String(input);
    if (path === "/v1/events")
      return new Response(
        new ReadableStream({
          start(controller) {
            stream = controller;
          },
          cancel() {},
        }),
        { headers: { "content-type": "text/event-stream" } },
      );
    paths.push(path);
    if (path.endsWith("/chatgpt"))
      return Response.json({ supported: false, accounts: [], attempts: [] });
    const intercepted = intercept.read?.(path);
    if (intercepted) return intercepted;
    if (path.includes("/events?"))
      return Response.json({
        version: 1,
        next: null,
        events: events.filter(
          (e) =>
            e.sequence >
            Number(new URL(path, "http://localhost").searchParams.get("after")),
        ),
      });
    if (path.includes("/feedback?")) return Response.json({ messages: [] });
    return new Response(
      JSON.stringify(
        path.endsWith("/settings")
          ? { revision: 0, keep_awake: false }
          : path.endsWith("/targets")
            ? []
            : status,
      ),
      { headers: { "content-type": "application/json" } },
    );
  };
  const freshness = new ApplicationFreshness(fetcher);
  const frame = (body: unknown, type = "changed") =>
    stream.enqueue(
      new TextEncoder().encode(
        `id: jse1:${"1".repeat(32)}:${"2".repeat(32)}:${revision++}\nevent: ${type}\ndata: ${JSON.stringify(body)}\n\n`,
      ),
    );
  const ready = freshness.configure(true, null);
  frame({ version: 1, type: "ready", witnesses: [] }, "ready");
  await ready;
  const manager = new AgentExecution(() => true, fetcher, undefined, freshness);
  await manager.refresh();
  return {
    manager,
    status,
    intercept,
    events,
    disconnect: () => stream.close(),
    heartbeat: () =>
      stream.enqueue(new TextEncoder().encode(": keep-alive\n\n")),
    freshness,
    paths,
    frame,
    notice(domain: string, repository: string | null = null) {
      frame({
        version: 1,
        type: "changed",
        change: { repository, domain, witness: String(revision) },
      });
    },
    close() {
      manager.destroy();
      freshness.destroy();
    },
  };
}

test("healthy idle agent state performs no periodic settings, targets or status reads", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  try {
    expect(f.paths).toHaveLength(4);
    await vi.advanceTimersByTimeAsync(30000);
    expect(f.paths).toHaveLength(4);
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("selected inbox uses the shared feed, fetches only new activity and skips unchanged status", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  try {
    f.manager.open = true;
    f.status.sessions = [
      {
        id: "session",
        repository: "3".repeat(32),
        last_sequence: 1,
        published_sequence: 1,
      } as AgentSession,
    ];
    await f.manager.updates.status();
    f.events.push({ sequence: 1, committed: true, payload: { type: "ready" } });
    await f.manager.select("session");
    f.paths.length = 0;
    f.events.push({
      sequence: 2,
      committed: false,
      payload: { type: "update", text: "Added" },
    });
    f.status.sessions[0]!.last_sequence = 2;
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(400);
    expect(f.manager.events).toHaveLength(2);
    expect(f.paths).toEqual([
      "/v1/local/agents",
      "/v1/local/agents/events?session=session&after=1",
    ]);
    f.paths.length = 0;
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(400);
    expect(f.paths).toEqual(["/v1/local/agents"]);
    f.paths.length = 0;
    await vi.advanceTimersByTimeAsync(30000);
    expect(f.paths).toEqual([]);
    f.frame(
      { version: 1, type: "reset", reason: "restart", witnesses: [] },
      "reset",
    );
    await vi.advanceTimersByTimeAsync(1000);
    expect(f.paths.filter((p) => p.includes("/events?"))).toEqual([
      "/v1/local/agents/events?session=session&after=0",
    ]);
    expect(f.paths.some((p) => p.includes("/stream?"))).toBe(false);
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("inbox remounts reuse initial status, settings and targets", async () => {
  const f = await fixture();
  try {
    await f.manager.ensure();
    await Promise.all([f.manager.ensure(), f.manager.ensure()]);
    expect([...f.paths].sort()).toEqual([
      "/v1/local/agents",
      "/v1/local/agents/chatgpt",
      "/v1/local/agents/settings",
      "/v1/local/agents/targets",
    ]);
  } finally {
    f.close();
  }
});

test("runtime and compute settings reuse captures, stay idle and observe execution events", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  try {
    f.paths.length = 0;
    f.manager.tooling.start();
    f.manager.compute.start();
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths.sort()).toEqual([
      "/v1/local/agents/runtimes",
      "/v1/local/compute",
    ]);
    await vi.advanceTimersByTimeAsync(30000);
    expect(f.paths).toHaveLength(2);
    f.manager.tooling.stop();
    f.manager.compute.stop();
    f.manager.tooling.start();
    f.manager.compute.start();
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toHaveLength(2);
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths.filter((p) => p.endsWith("/runtimes"))).toHaveLength(2);
    expect(f.paths.filter((p) => p.endsWith("/compute"))).toHaveLength(2);
    expect(f.paths).not.toContain("/v1/local/agents/settings");
    f.manager.tooling.stop();
    f.manager.compute.stop();
    f.paths.length = 0;
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toEqual(["/v1/local/agents"]);
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("conversation metadata shares reads, ignores unrelated events and retains hidden dirtiness", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  const q = { session: "metadata-session", repository: "3".repeat(32) };
  try {
    f.intercept.read = (path) =>
      path.includes("/metadata?")
        ? Promise.resolve(
            Response.json({ repository: q.repository, job: null }),
          )
        : undefined;
    f.paths.length = 0;
    await Promise.all([
      f.manager.metadata.ensure(q),
      f.manager.metadata.ensure(q),
    ]);
    const stop = f.manager.metadata.watch(q);
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toHaveLength(1);
    await vi.advanceTimersByTimeAsync(30000);
    expect(f.paths).toHaveLength(1);
    f.notice("threads", q.repository);
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toHaveLength(2);
    stop();
    f.notice("threads", q.repository);
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toHaveLength(2);
    await f.manager.metadata.ensure(q);
    expect(f.paths).toHaveLength(3);
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("selection and discussion watchers share one feedback read and retain it on remount", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  try {
    f.paths.length = 0;
    let finish!: (response: Response) => void;
    f.intercept.read = (path) =>
      path.includes("/feedback?")
        ? new Promise((done) => {
            finish = done;
          })
        : undefined;
    const first = f.manager.discussions.ensure("session");
    const stop = f.manager.discussions.watch("session", "3".repeat(32));
    await vi.advanceTimersByTimeAsync(200);
    finish(Response.json({ messages: [] }));
    await first;
    await vi.advanceTimersByTimeAsync(6000);
    stop();
    const stopAgain = f.manager.discussions.watch("session", "3".repeat(32));
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toEqual(["/v1/local/agents/feedback?session=session"]);
    stopAgain();
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("agent notices refresh only their owning data and reset retains drafts", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  try {
    f.manager.drafts.session = "Keep my draft";
    f.paths.length = 0;
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toEqual(["/v1/local/agents"]);
    f.paths.length = 0;
    f.notice("agent_settings");
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toEqual([
      "/v1/local/agents/settings",
      "/v1/local/agents/chatgpt",
    ]);
    f.paths.length = 0;
    f.notice("code", "3".repeat(32));
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toEqual(["/v1/local/agents/targets"]);
    f.paths.length = 0;
    f.frame(
      { version: 1, type: "reset", reason: "replay_window", witnesses: [] },
      "reset",
    );
    await vi.advanceTimersByTimeAsync(200);
    expect([...f.paths].sort()).toEqual([
      "/v1/local/agents",
      "/v1/local/agents/chatgpt",
      "/v1/local/agents/settings",
      "/v1/local/agents/targets",
    ]);
    expect(f.manager.drafts.session).toBe("Keep my draft");
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("chat and recorded discussion stay idle and update on their respective events", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  const load = vi.fn(async () => {});
  const stop = watchAgentDiscussion(load, "3".repeat(32), f.freshness);
  try {
    f.manager.chat.showGeneral();
    f.manager.chat.draft!.text = "Keep chat draft";
    f.status.sessions = [
      {
        id: "session",
        conversation_id: f.manager.chat.selected,
        repository: "3".repeat(32),
        title: "Chat",
        state: "idle",
        last_sequence: 0,
      } as AgentSession,
    ];
    await f.manager.updates.status();
    await vi.advanceTimersByTimeAsync(200);
    f.paths.length = 0;
    await vi.advanceTimersByTimeAsync(30000);
    expect(f.paths).toEqual([]);
    expect(load).toHaveBeenCalledTimes(1);
    f.events.push({
      sequence: 1,
      committed: false,
      payload: { type: "update", text: "New response" },
    });
    f.status.sessions[0]!.last_sequence = 1;
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(400);
    expect(f.manager.chat.events.session).toHaveLength(1);
    expect(f.manager.chat.draft!.text).toBe("Keep chat draft");
    expect(f.paths).not.toContain("/v1/local/agents/settings");
    expect(f.paths).not.toContain("/v1/local/agents/targets");
    f.notice("threads", "3".repeat(32));
    await vi.advanceTimersByTimeAsync(200);
    expect(load).toHaveBeenCalledTimes(2);
  } finally {
    stop();
    f.close();
    vi.useRealTimers();
  }
});

test("event dirtiness survives a failed read and another event during an in-flight read", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  try {
    f.paths.length = 0;
    let fail = true;
    f.intercept.read = (path) =>
      path.endsWith("/agents") && fail
        ? Promise.reject(new Error("Temporarily offline"))
        : undefined;
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(200);
    expect(f.manager.connectionError).toBe("Temporarily offline");
    fail = false;
    await vi.advanceTimersByTimeAsync(6000);
    expect(f.manager.connectionError).toBe("");
    expect(f.paths).toHaveLength(2);
    let finish!: (response: Response) => void;
    f.intercept.read = (path) =>
      path.endsWith("/agents")
        ? new Promise((done) => {
            finish = done;
          })
        : undefined;
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(200);
    f.notice("agent_execution");
    await vi.advanceTimersByTimeAsync(1);
    f.intercept.read = undefined;
    finish(Response.json(f.status));
    await vi.advanceTimersByTimeAsync(6000);
    expect(f.paths).toHaveLength(4);
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("unchanged renewal and feed outages never poll data", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  try {
    f.paths.length = 0;
    for (let i = 0; i < 20; i++) {
      f.heartbeat();
      await vi.advanceTimersByTimeAsync(15000);
    }
    f.disconnect();
    await vi.advanceTimersByTimeAsync(1);
    f.frame({ version: 1, type: "ready", witnesses: [] }, "ready");
    await vi.advanceTimersByTimeAsync(200);
    expect(f.paths).toEqual([]);
    f.disconnect();
    await vi.advanceTimersByTimeAsync(12000);
    expect(f.paths.filter((p) => p.endsWith("/agents"))).toHaveLength(0);
    expect(f.paths.filter((p) => p.endsWith("/settings"))).toHaveLength(0);
    expect(f.paths.filter((p) => p.endsWith("/targets"))).toHaveLength(0);
    f.frame({ version: 1, type: "ready", witnesses: [] }, "ready");
    await vi.advanceTimersByTimeAsync(200);
    const captured = [...f.paths];
    await vi.advanceTimersByTimeAsync(30000);
    expect(f.paths).toEqual(captured);
  } finally {
    f.close();
    vi.useRealTimers();
  }
});

test("failed explicit status and discussion reads recover without another event", async () => {
  vi.useFakeTimers();
  const f = await fixture();
  let fail = true;
  const load = vi.fn(async () => !fail);
  const stop = watchAgentDiscussion(load, "3".repeat(32), f.freshness);
  try {
    f.intercept.read = (path) =>
      path.endsWith("/agents") && fail
        ? Promise.reject(new Error("Temporarily offline"))
        : undefined;
    await f.manager.updates.status();
    await vi.advanceTimersByTimeAsync(200);
    expect(f.manager.connectionError).toBe("Temporarily offline");
    expect(load).toHaveBeenCalledTimes(1);
    fail = false;
    await vi.advanceTimersByTimeAsync(6000);
    expect(f.manager.connectionError).toBe("");
    expect(load).toHaveBeenCalledTimes(2);
  } finally {
    stop();
    f.close();
    vi.useRealTimers();
  }
});
