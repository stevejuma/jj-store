import { afterEach, expect, test, vi } from "vitest";
import { websocketApplicationEvents } from "../../src/lib/api/application-events-websocket.js";
import { ApplicationEventHub } from "../../src/lib/api/application-event-hub.js";
import { sharedApplicationEvents } from "../../src/lib/api/shared-application-events.js";
import { EventSocket, installSockets } from "./event-websocket-fixture.js";
const prefix = `jse1:${"aa".repeat(16)}:${"bb".repeat(16)}:`;
const ready = {
  id: `${prefix}0`,
  event: { version: 1, type: "ready", witnesses: [] },
};
afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});
function fixture(token: string | null = null) {
  vi.useFakeTimers();
  installSockets();
  const abort = new AbortController(),
    deliver = vi.fn();
  const promise = websocketApplicationEvents(
    token,
    `${prefix}0`,
    abort.signal,
    deliver,
  );
  return { abort, deliver, promise, socket: EventSocket.instances[0]! };
}
test("credentials and resume cursor travel only in the first message; heartbeat and renewal do not deliver data", async () => {
  const f = fixture("test-only-token");
  expect(f.socket.url).toBe("ws://127.0.0.1:7460/v1/events/ws");
  expect(f.socket.send).not.toHaveBeenCalled();
  f.socket.onopen?.();
  expect(JSON.parse(f.socket.send.mock.calls[0]![0])).toEqual({
    version: 1,
    token: "test-only-token",
    after: `${prefix}0`,
  });
  f.socket.receive(ready);
  await vi.advanceTimersByTimeAsync(40000);
  f.socket.receive({ version: 1, type: "heartbeat" });
  await vi.advanceTimersByTimeAsync(40000);
  expect(f.deliver).toHaveBeenCalledOnce();
  expect(f.socket.close).not.toHaveBeenCalled();
  f.socket.onclose?.({ code: 1000 });
  await f.promise;
  expect(f.socket.close).toHaveBeenCalledOnce();
});
test("abort, silent startup and stalled delivery all release their sockets", async () => {
  const f = fixture();
  const rejected = expect(f.promise).rejects.toMatchObject({
    name: "AbortError",
  });
  f.abort.abort();
  await rejected;
  expect(f.socket.close).toHaveBeenCalledOnce();
  for (const established of [false, true]) {
    const g = fixture();
    if (established) g.socket.receive(ready);
    const timeout = expect(g.promise).rejects.toMatchObject({
      code: "event_unavailable",
    });
    await vi.advanceTimersByTimeAsync(established ? 45000 : 35000);
    await timeout;
    expect(g.socket.close).toHaveBeenCalledOnce();
  }
});
test("denial and invalid frames fail closed without HTTP fallback", async () => {
  const f = fixture();
  const denied = expect(f.promise).rejects.toMatchObject({
    code: "access_denied",
  });
  f.socket.onclose?.({ code: 4403 });
  await denied;
  for (const bad of [
    null,
    {},
    { ...ready, id: "bad" },
    { ...ready, event: { version: 2, type: "ready", witnesses: [] } },
    {
      ...ready,
      event: { version: 1, type: "changed", change: { domain: "unknown" } },
    },
  ]) {
    const g = fixture();
    const rejected = expect(g.promise).rejects.toBeDefined();
    g.socket.receive(bad);
    await rejected;
    expect(g.socket.close).toHaveBeenCalledOnce();
  }
});
test("twenty subscribers share one default WebSocket; final release closes it and credentials stay isolated", async () => {
  installSockets();
  const hub = new ApplicationEventHub();
  const consumers = Array.from({ length: 20 }, () => ({
    deliver: vi.fn(),
    end: vi.fn(),
  }));
  const stops = consumers.map((c) => hub.subscribe(null, undefined, c));
  expect(EventSocket.instances).toHaveLength(1);
  EventSocket.instances[0]!.receive(ready);
  consumers.forEach((c) => expect(c.deliver).toHaveBeenCalledOnce());
  const stopOther = hub.subscribe("other", undefined, {
    deliver: vi.fn(),
    end: vi.fn(),
  });
  expect(EventSocket.instances).toHaveLength(2);
  stops[0]!();
  expect(EventSocket.instances[0]!.close).not.toHaveBeenCalled();
  stops.forEach((stop) => stop());
  stopOther();
  expect(EventSocket.instances[0]!.close).toHaveBeenCalledOnce();
  expect(EventSocket.instances[1]!.close).toHaveBeenCalledOnce();
});
test("unsupported or failed SharedWorker uses WebSocket only, including many independent tabs", async () => {
  installSockets();
  vi.stubGlobal("window", new EventTarget());
  vi.stubGlobal("SharedWorker", undefined);
  const fetcher = vi.fn();
  vi.stubGlobal("fetch", fetcher);
  const runs = Array.from({ length: 8 }, () => {
    const abort = new AbortController();
    const promise = sharedApplicationEvents(
      null,
      undefined,
      abort.signal,
      vi.fn(),
    ).catch(() => {});
    return { abort, promise };
  });
  expect(EventSocket.instances).toHaveLength(8);
  expect(fetcher).not.toHaveBeenCalled();
  runs.forEach((r) => r.abort.abort());
  await Promise.all(runs.map((r) => r.promise));
  EventSocket.instances.forEach((s) => expect(s.close).toHaveBeenCalledOnce());
});
