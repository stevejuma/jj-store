import { expect, test, vi } from "vitest";
import { AgentExecution } from "../../src/lib/controller/agent-execution.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
import { consumeAgentEvents } from "../../src/lib/controller/agent-execution-stream.js";
import { RetainedReads } from "../../src/lib/controller/retained-reads.js";
import type { AgentSession } from "../../src/lib/controller/agent-execution-types.js";

test("history captured before initial status reuses its exact committed boundary", async () => {
  const paths: string[] = [];
  const manager = new AgentExecution(
    () => true,
    async (input) => {
      const path = String(input);
      paths.push(path);
      if (path.includes("/stream?"))
        return new Response(new ReadableStream(), {
          headers: { "content-type": "text/event-stream" },
        });
      return Response.json(
        path.includes("/feedback?")
          ? { messages: [] }
          : {
              events: [
                { sequence: 1, committed: true, payload: { type: "ready" } },
                { sequence: 2, committed: false, payload: { type: "update" } },
              ],
              next: null,
            },
      );
    },
  );
  try {
    await manager.select("a");
    manager.value = {
      version: 1,
      revision: 0,
      runtime: "ready",
      runners: [],
      setups: [],
      sessions: [
        { id: "a", last_sequence: 2, published_sequence: 1 } as AgentSession,
      ],
    };
    manager.pauseView();
    await manager.select("a");
    expect(paths.filter((path) => path.includes("/events?"))).toHaveLength(1);
    expect(manager.events).toHaveLength(2);
    manager.pauseView();
    manager.value.sessions[0]!.published_sequence = 2;
    await manager.select("a");
    expect(paths.filter((path) => path.includes("/events?"))).toHaveLength(2);
  } finally {
    manager.destroy();
  }
});

test("returning to a chat reuses history and feedback, opens one stream and retains drafts", async () => {
  const paths: string[] = [];
  let streams = 0,
    cancelled = 0;
  const manager = new AgentExecution(
    () => true,
    async (url) => {
      const path = String(url);
      paths.push(path);
      if (path.includes("/stream?")) {
        streams++;
        return new Response(
          new ReadableStream({
            cancel() {
              cancelled++;
            },
          }),
          {
            headers: { "content-type": "text/event-stream" },
          },
        );
      }
      return Response.json(
        path.includes("/feedback?")
          ? { messages: [] }
          : {
              events: [
                { sequence: 1, committed: true, payload: { type: "ready" } },
              ],
              next: null,
            },
      );
    },
  );
  manager.value = {
    version: 1,
    revision: 0,
    runtime: "ready",
    runners: [],
    setups: [],
    sessions: ["a", "b"].map(
      (id) =>
        ({
          id,
          repository: "1".repeat(32),
          last_sequence: 1,
          published_sequence: 1,
        }) as AgentSession,
    ),
  };
  try {
    manager.drafts.a = "Keep draft";
    await manager.select("a");
    await manager.select("a");
    await manager.select("b");
    await manager.select("a");
    expect(paths.filter((p) => p.includes("/events?"))).toHaveLength(2);
    expect(paths.filter((p) => p.includes("/feedback?"))).toHaveLength(2);
    expect(streams).toBe(3);
    expect(cancelled).toBe(2);
    expect(manager.events).toHaveLength(1);
    expect(manager.drafts.a).toBe("Keep draft");
    manager.pauseView();
    await Promise.resolve();
    expect(cancelled).toBe(3);
    manager.value.sessions[0]!.published_sequence = 2;
    await manager.select("a");
    expect(
      paths.filter((p) => p.includes("/events?") && p.includes("after=0")),
    ).toHaveLength(3);
  } finally {
    manager.destroy();
  }
});

test("healthy activity stream survives the ordinary 35 second deadline and releases on abort", async () => {
  vi.useFakeTimers();
  const timeout = vi.spyOn(AbortSignal, "timeout").mockImplementation((ms) => {
    const abort = new AbortController();
    setTimeout(() => abort.abort(), ms);
    return abort.signal;
  });
  const abort = new AbortController();
  let body!: ReadableStreamDefaultController<Uint8Array>;
  const cancelled = vi.fn();
  const fetcher = vi.fn(
    async (_url, init) =>
      new Response(
        new ReadableStream<Uint8Array>({
          start(c) {
            body = c;
            init?.signal?.addEventListener("abort", () => {
              try {
                c.close();
              } catch {}
            });
          },
          cancel: cancelled,
        }),
        { headers: { "content-type": "text/event-stream" } },
      ),
  );
  const failed = vi.fn();
  const reading = consumeAgentEvents(
    new LocalClient("", fetcher),
    "session",
    0,
    abort.signal,
    {
      alive: () => true,
      receive: () => {},
      failed,
    },
  );
  try {
    await vi.advanceTimersByTimeAsync(1);
    for (let i = 0; i < 5; i++) {
      body.enqueue(new TextEncoder().encode(": heartbeat\n\n"));
      await vi.advanceTimersByTimeAsync(10000);
    }
    expect(fetcher).toHaveBeenCalledTimes(1);
    expect(failed).not.toHaveBeenCalled();
    expect(timeout).not.toHaveBeenCalled();
  } finally {
    abort.abort();
    await reading;
    timeout.mockRestore();
    vi.useRealTimers();
  }
});

test("event during a shared read remains dirty, errors are not cached, and clear invalidates", async () => {
  let finish!: (ok: boolean) => void;
  const read = vi.fn(
    () =>
      new Promise<boolean>((done) => {
        finish = done;
      }),
  );
  const cache = new RetainedReads(read);
  const a = cache.ensure("settings"),
    b = cache.ensure("settings");
  expect(read).toHaveBeenCalledTimes(1);
  cache.invalidate("settings");
  finish(true);
  await Promise.all([a, b]);
  const next = cache.ensure("settings");
  finish(false);
  await next;
  const retry = cache.ensure("settings");
  finish(true);
  await retry;
  await cache.ensure("settings");
  expect(read).toHaveBeenCalledTimes(3);
  cache.clear();
  const reset = cache.ensure("settings");
  finish(true);
  await reset;
  expect(read).toHaveBeenCalledTimes(4);
});
