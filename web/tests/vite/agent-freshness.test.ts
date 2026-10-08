import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import type {
  AgentStatus,
  AgentSession,
} from "../../src/lib/controller/agent-execution-types.js";
import { watchAgentDiscussion } from "../../src/lib/controller/agent-discussion-refresh.js";
import { ApplicationFreshness } from "../../src/lib/controller/application-freshness.js";

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
    expect(f.paths).toHaveLength(3);
    await vi.advanceTimersByTimeAsync(30000);
    expect(f.paths).toHaveLength(3);
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
    expect(f.paths).toEqual(["/v1/local/agents/settings"]);
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

test("unchanged feed renewal stays read-free and outage fallback stops on recovery", async () => {
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
    expect(f.paths.filter((p) => p.endsWith("/agents"))).toHaveLength(1);
    expect(f.paths.filter((p) => p.endsWith("/settings"))).toHaveLength(1);
    expect(f.paths.filter((p) => p.endsWith("/targets"))).toHaveLength(1);
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
