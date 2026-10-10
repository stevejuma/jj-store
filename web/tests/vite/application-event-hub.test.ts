import { afterEach, expect, test, vi } from "vitest";
import { ApplicationEventHub } from "../../src/lib/api/application-event-hub.js";
import type {
  ApplicationEvent,
  FreshnessChange,
} from "../../src/lib/api/application-events.js";
const prefix = `jse1:${"aa".repeat(16)}:${"bb".repeat(16)}:`;
const change: FreshnessChange = {
  domain: "threads",
  repository: "11".repeat(16),
  witness: "one",
};
const ready: ApplicationEvent = {
  version: 1,
  type: "ready",
  witnesses: [change],
};
const flush = () => vi.advanceTimersByTimeAsync(0);
afterEach(() => vi.useRealTimers());

function fixture() {
  vi.useFakeTimers();
  const streams: ReadableStreamDefaultController<Uint8Array>[] = [];
  const cancelled: number[] = [];
  const fetcher = vi.fn(async (_url: unknown, init?: RequestInit) => {
    const index = streams.length;
    const body = new ReadableStream<Uint8Array>({
      start(controller) {
        streams.push(controller);
      },
      cancel() {
        cancelled.push(index);
      },
    });
    return new Response(body, {
      headers: { "content-type": "text/event-stream" },
    });
  });
  const hub = new ApplicationEventHub(fetcher);
  const cleanups: (() => void)[] = [];
  function subscribe(token: string | null = null, after?: string) {
    const subscriber = { deliver: vi.fn(), end: vi.fn() };
    const stop = hub.subscribe(token, after, subscriber);
    cleanups.push(stop);
    return { ...subscriber, stop };
  }
  function send(event: ApplicationEvent, n: number, index = 0, epoch = prefix) {
    streams[index]!.enqueue(
      new TextEncoder().encode(
        `id: ${epoch}${n}\nevent: ${event.type}\ndata: ${JSON.stringify(event)}\n\n`,
      ),
    );
  }
  return {
    hub,
    fetcher,
    streams,
    cancelled,
    subscribe,
    send,
    close: () => cleanups.forEach((stop) => stop()),
  };
}

test("six app instances occupy one event connection and late joiners get current witnesses", async () => {
  const f = fixture();
  try {
    const first = f.subscribe();
    await flush();
    f.send(ready, 0);
    f.send(
      { version: 1, type: "changed", change: { ...change, witness: "two" } },
      1,
    );
    await flush();
    const others = Array.from({ length: 5 }, () => f.subscribe());
    expect(f.fetcher).toHaveBeenCalledTimes(1);
    for (const observer of others)
      expect(observer.deliver).toHaveBeenCalledWith(
        {
          version: 1,
          type: "ready",
          witnesses: [{ ...change, witness: "two" }],
        },
        `${prefix}1`,
      );
    first.stop();
    expect(f.cancelled).toEqual([]);
    f.send(
      { version: 1, type: "changed", change: { ...change, witness: "three" } },
      2,
    );
    await flush();
    for (const observer of others)
      expect(observer.deliver).toHaveBeenCalledTimes(2);
    f.close();
    await flush();
    expect(f.cancelled).toEqual([0]);
  } finally {
    f.close();
  }
});

test("credential scopes never share authorization or witnesses and release independently", async () => {
  const f = fixture();
  try {
    const local = f.subscribe();
    const alice = f.subscribe("alice-test-token");
    const bob = f.subscribe("bob-test-token");
    await flush();
    expect(f.fetcher).toHaveBeenCalledTimes(3);
    expect(
      f.fetcher.mock.calls.map(
        (call) => (call[1]?.headers as Record<string, string>).authorization,
      ),
    ).toEqual([undefined, "Bearer alice-test-token", "Bearer bob-test-token"]);
    f.send(ready, 0, 1);
    await flush();
    expect(alice.deliver).toHaveBeenCalledOnce();
    expect(local.deliver).not.toHaveBeenCalled();
    expect(bob.deliver).not.toHaveBeenCalled();
    alice.stop();
    await flush();
    expect(f.cancelled).toEqual([1]);
  } finally {
    f.close();
  }
});

test("last release and immediate reacquisition cannot retain or cancel the new connection", async () => {
  const f = fixture();
  try {
    const first = f.subscribe();
    first.stop();
    const next = f.subscribe();
    await flush();
    expect(f.fetcher).toHaveBeenCalledTimes(2);
    expect(f.cancelled).toEqual([0]);
    f.send(ready, 0, 1);
    await flush();
    expect(next.deliver).toHaveBeenCalledOnce();
    expect(first.deliver).not.toHaveBeenCalled();
  } finally {
    f.close();
  }
});

test("foreign continuity resets a returning subscriber without resetting current subscribers", async () => {
  const f = fixture();
  try {
    const current = f.subscribe();
    await flush();
    f.send(ready, 4);
    await flush();
    const returning = f.subscribe(
      null,
      `jse1:${"cc".repeat(16)}:${"bb".repeat(16)}:9`,
    );
    expect(returning.deliver.mock.calls[0]?.[0]).toMatchObject({
      type: "reset",
      witnesses: [change],
    });
    expect(current.deliver).toHaveBeenCalledTimes(1);
    const retained = f.subscribe(null, `${prefix}2`);
    expect(retained.deliver.mock.calls[0]?.[0].type).toBe("ready");
  } finally {
    f.close();
  }
});

test("stream renewal is shared and malformed input releases the old body before retry", async () => {
  const f = fixture();
  try {
    const first = f.subscribe();
    const second = f.subscribe();
    await flush();
    f.send(ready, 0);
    await flush();
    f.streams[0]!.enqueue(
      new TextEncoder().encode("event: changed\nid: bad\ndata: {}\n\n"),
    );
    await flush();
    expect(first.end).toHaveBeenCalledWith("invalid_application_event");
    expect(second.end).toHaveBeenCalledWith("invalid_application_event");
    expect(f.cancelled).toEqual([0]);
    f.subscribe(null, `${prefix}0`);
    f.subscribe(null, `${prefix}0`);
    await flush();
    expect(f.fetcher).toHaveBeenCalledTimes(2);
    f.streams[1]!.close();
    await flush();
  } finally {
    f.close();
  }
});

test("denied authorization is delivered to all scoped observers without caching a ready result", async () => {
  const denied = vi.fn(async () => new Response(null, { status: 403 }));
  const hub = new ApplicationEventHub(denied);
  const a = { deliver: vi.fn(), end: vi.fn() };
  const b = { deliver: vi.fn(), end: vi.fn() };
  const stopA = hub.subscribe("denied-test-token", undefined, a);
  const stopB = hub.subscribe("denied-test-token", undefined, b);
  await vi.waitFor(() => expect(a.end).toHaveBeenCalledWith("access_denied"));
  expect(b.end).toHaveBeenCalledWith("access_denied");
  expect(denied).toHaveBeenCalledOnce();
  expect(a.deliver).not.toHaveBeenCalled();
  stopA();
  stopB();
});

test("stale event cursors cannot contaminate the snapshot offered to another app", async () => {
  const f = fixture();
  try {
    const first = f.subscribe();
    await flush();
    f.send(ready, 4);
    f.send(
      { version: 1, type: "changed", change: { ...change, witness: "stale" } },
      3,
    );
    await flush();
    expect(first.deliver).toHaveBeenCalledOnce();
    expect(first.end).toHaveBeenCalledWith("invalid_application_cursor");
    const next = f.subscribe();
    expect(next.deliver).not.toHaveBeenCalled();
    expect(f.fetcher).toHaveBeenCalledTimes(2);
  } finally {
    f.close();
  }
});

test("witness fanout remains bounded as repositories change during a stream", async () => {
  const f = fixture();
  try {
    const first = f.subscribe();
    await flush();
    const witnesses = Array.from({ length: 4096 }, (_, i) => ({
      ...change,
      repository: i.toString(16).padStart(32, "0"),
    }));
    f.send({ version: 1, type: "ready", witnesses }, 0);
    f.send({ version: 1, type: "changed", change }, 1);
    await flush();
    expect(first.deliver).toHaveBeenCalledOnce();
    expect(first.end).toHaveBeenCalledWith("event_frame_limit");
    expect(f.cancelled).toEqual([0]);
  } finally {
    f.close();
  }
});

test("late snapshots invalidate the full domain instead of only its last changed identity", async () => {
  const f = fixture();
  try {
    const current = f.subscribe();
    await flush();
    f.send(ready, 0);
    f.send(
      {
        version: 1,
        type: "changed",
        change: { ...change, identity: "first", witness: "two" },
      },
      1,
    );
    f.send(
      {
        version: 1,
        type: "changed",
        change: { ...change, identity: "second", witness: "three" },
      },
      2,
    );
    await flush();
    expect(current.deliver.mock.calls.at(-1)?.[0]).toMatchObject({
      change: { identity: "second" },
    });
    const returning = f.subscribe(null, `${prefix}0`);
    expect(returning.deliver.mock.calls[0]?.[0]).toEqual({
      version: 1,
      type: "ready",
      witnesses: [{ ...change, witness: "three" }],
    });
  } finally {
    f.close();
  }
});

test("returning owners retain a missed repository reset even after later domain changes", async () => {
  const f = fixture();
  try {
    f.subscribe();
    await flush();
    f.send(ready, 0);
    const topology: FreshnessChange = {
      domain: "repository",
      repository: null,
      witness: "new-topology",
    };
    f.send({ version: 1, type: "changed", change: topology }, 1);
    f.send(
      {
        version: 1,
        type: "changed",
        change: {
          domain: "agent_settings",
          repository: null,
          witness: "defaults",
        },
      },
      2,
    );
    await flush();
    const returning = f.subscribe(null, `${prefix}0`);
    expect(returning.deliver.mock.calls[0]?.[0]).toMatchObject({
      type: "reset",
    });
    expect(returning.deliver.mock.calls[0]?.[0]).toMatchObject({
      witnesses: [
        topology,
        { domain: "agent_settings", repository: null, witness: "defaults" },
      ],
    });
    expect(
      f.subscribe(null, `${prefix}1`).deliver.mock.calls[0]?.[0],
    ).toMatchObject({ type: "ready" });
  } finally {
    f.close();
  }
});
