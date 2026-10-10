import { EventSocket, installSockets } from "./event-websocket-fixture.js";
import { afterEach, expect, test, vi } from "vitest";
import { sharedApplicationEvents } from "../../src/lib/api/shared-application-events.js";
const cursor = `jse1:${"aa".repeat(16)}:${"bb".repeat(16)}:0`;
const ready = {
  type: "event",
  id: cursor,
  value: { version: 1, type: "ready", witnesses: [] },
};
class WorkerStub {
  static instances: WorkerStub[] = [];
  onerror: (() => void) | null = null;
  port = {
    onmessage: null as null | ((message: { data: unknown }) => void),
    onmessageerror: null as null | (() => void),
    start: vi.fn(),
    postMessage: vi.fn(),
    close: vi.fn(),
  };
  constructor(
    readonly url: URL,
    readonly options: SharedWorkerOptions,
  ) {
    WorkerStub.instances.push(this);
  }
  receive(data: unknown) {
    this.port.onmessage?.({ data });
  }
}
function setup() {
  vi.useFakeTimers();
  installSockets();
  WorkerStub.instances = [];
  const window = new EventTarget();
  vi.stubGlobal("window", window);
  vi.stubGlobal("SharedWorker", WorkerStub);
  const fetcher = vi.fn(async () => new Response(null, { status: 503 }));
  vi.stubGlobal("fetch", fetcher);
  const abort = new AbortController();
  const deliver = vi.fn();
  const promise = sharedApplicationEvents(
    null,
    undefined,
    abort.signal,
    deliver,
  );
  return {
    window,
    fetcher,
    abort,
    deliver,
    promise,
    worker: WorkerStub.instances[0]!,
  };
}
afterEach(() => {
  vi.unstubAllGlobals();
  vi.useRealTimers();
});

test("browser ownership joins the stable worker and abort releases only its port", async () => {
  const f = setup();
  expect(f.worker.options).toEqual({
    type: "module",
  });
  expect(f.worker.port.postMessage).toHaveBeenCalledWith({
    type: "subscribe",
    token: null,
    after: undefined,
  });
  f.worker.receive(ready);
  expect(f.deliver).toHaveBeenCalledWith(ready.value, cursor);
  await vi.advanceTimersByTimeAsync(36000);
  expect(f.worker.port.close).not.toHaveBeenCalled();
  const rejection = expect(f.promise).rejects.toMatchObject({
    name: "AbortError",
  });
  f.abort.abort();
  await rejection;
  expect(f.worker.port.postMessage).toHaveBeenLastCalledWith({
    type: "unsubscribe",
  });
  expect(f.worker.port.close).toHaveBeenCalledOnce();
  expect(f.fetcher).not.toHaveBeenCalled();
});

test("page exit releases the subscription before its document disappears", async () => {
  const f = setup();
  f.worker.receive(ready);
  f.window.dispatchEvent(new Event("pagehide"));
  await f.promise;
  expect(f.worker.port.close).toHaveBeenCalledOnce();
  f.abort.abort();
  expect(f.worker.port.close).toHaveBeenCalledOnce();
});

test("denial propagates without opening a second direct transport", async () => {
  const f = setup();
  const rejection = expect(f.promise).rejects.toMatchObject({
    code: "access_denied",
  });
  f.worker.receive({ type: "end", error: "access_denied" });
  await rejection;
  expect(f.fetcher).not.toHaveBeenCalled();
  expect(f.worker.port.close).toHaveBeenCalledOnce();
});

test("failed worker startup falls back, while a silent startup has a bounded deadline", async () => {
  const f = setup();
  const rejection = expect(f.promise).rejects.toMatchObject({
    code: "event_unavailable",
  });
  f.worker.onerror?.();
  await Promise.resolve();
  EventSocket.instances[0]!.onerror?.();
  await rejection;
  expect(EventSocket.instances).toHaveLength(1);
  expect(f.fetcher).not.toHaveBeenCalled();
  expect(f.worker.port.close).toHaveBeenCalledOnce();
  const abort = new AbortController();
  const stalled = sharedApplicationEvents(
    null,
    undefined,
    abort.signal,
    vi.fn(),
  );
  const timeout = expect(stalled).rejects.toMatchObject({
    code: "event_unavailable",
  });
  await vi.advanceTimersByTimeAsync(35000);
  await timeout;
  expect(WorkerStub.instances[1]!.port.close).toHaveBeenCalledOnce();
  expect(f.fetcher).not.toHaveBeenCalled();
});

test("a consumer parser failure detaches its port and never accepts subsequent hints", async () => {
  const f = setup();
  f.deliver.mockImplementation(() => {
    throw new Error("invalid cursor");
  });
  const rejection = expect(f.promise).rejects.toThrow("invalid cursor");
  f.worker.receive(ready);
  await rejection;
  f.worker.receive(ready);
  expect(f.deliver).toHaveBeenCalledOnce();
  expect(f.worker.port.close).toHaveBeenCalledOnce();
});
