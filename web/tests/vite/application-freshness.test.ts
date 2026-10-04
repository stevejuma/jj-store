import { afterEach, expect, test, vi } from "vitest";
import { ApplicationFreshness } from "../../src/lib/controller/application-freshness.js";
import { ApplicationEventParser } from "../../src/lib/api/application-events.js";
const repository = "11".repeat(16),
  prefix = `jse1:${"aa".repeat(16)}:${"bb".repeat(16)}:`;
const change = (domain = "threads", witness = "one") => ({
  repository,
  domain,
  witness,
});
function frame(type: string, revision: number, value: object = {}) {
  return `id: ${prefix}${revision}\nevent: ${type}\ndata: ${JSON.stringify({ version: 1, type, ...value })}\n\n`;
}
afterEach(() => vi.useRealTimers());
test("split frames and heartbeats validate before advancing continuity", () => {
  const deliver = vi.fn();
  const parser = new ApplicationEventParser(deliver);
  const ready = frame("ready", 0, { witnesses: [change()] });
  parser.push(ready.slice(0, 15));
  expect(deliver).not.toHaveBeenCalled();
  parser.push(ready.slice(15) + ":keep-alive\n\n");
  expect(deliver).toHaveBeenCalledTimes(1);
  expect(() =>
    parser.push(
      frame("changed", 1, { change: { ...change(), repository: "foreign" } }),
    ),
  ).toThrow();
  expect(deliver).toHaveBeenCalledTimes(1);
});
function fixture() {
  vi.useFakeTimers();
  const streams: ReadableStreamDefaultController<Uint8Array>[] = [];
  const encoder = new TextEncoder();
  const send = (text: string, index = streams.length - 1) =>
    streams[index]!.enqueue(encoder.encode(text));
  const fetcher = vi.fn(async () => {
    const stream = new ReadableStream<Uint8Array>({
      start(controller) {
        streams.push(controller);
        controller.enqueue(
          encoder.encode(frame("ready", 0, { witnesses: [change()] })),
        );
      },
    });
    return new Response(stream, {
      headers: { "content-type": "text/event-stream" },
    });
  });
  const freshness = new ApplicationFreshness(fetcher);
  return { freshness, fetcher, streams, send };
}
test("healthy idle and unchanged five-minute renewal perform zero consumer reads", async () => {
  const f = fixture();
  await f.freshness.configure(true, null);
  const refresh = vi.fn(async () => true);
  const subscription = f.freshness.watch({
    matches: (c) => c.domain === "threads",
    refresh,
    initial: false,
  });
  for (let i = 0; i < 20; i++) {
    f.send(":keep-alive\n\n");
    await vi.advanceTimersByTimeAsync(15000);
  }
  expect(refresh).not.toHaveBeenCalled();
  f.streams[0]!.close();
  await vi.advanceTimersByTimeAsync(1);
  expect(f.fetcher).toHaveBeenCalledTimes(2);
  expect(f.freshness.healthy).toBe(true);
  expect(refresh).not.toHaveBeenCalled();
  subscription.stop();
  f.freshness.destroy();
});
test("scoped dirtiness survives failed reads and events arriving during a read", async () => {
  const f = fixture();
  await f.freshness.configure(true, null);
  const refresh = vi.fn(async () => true).mockResolvedValueOnce(false);
  const unaffected = vi.fn(async () => true);
  const a = f.freshness.watch({
    matches: (c) => c.domain === "threads",
    refresh,
    initial: false,
  });
  const b = f.freshness.watch({
    matches: (c) => c.domain === "storage",
    refresh: unaffected,
    initial: false,
  });
  f.send(frame("changed", 1, { change: change() }));
  await vi.advanceTimersByTimeAsync(151);
  expect(refresh).toHaveBeenCalledTimes(1);
  expect(unaffected).not.toHaveBeenCalled();
  await vi.advanceTimersByTimeAsync(6000);
  expect(refresh).toHaveBeenCalledTimes(2);
  let complete!: (value: boolean) => void;
  refresh.mockImplementationOnce(
    () =>
      new Promise((done) => {
        complete = done;
      }),
  );
  f.send(frame("changed", 2, { change: change("threads", "two") }));
  await vi.advanceTimersByTimeAsync(151);
  f.send(frame("changed", 3, { change: change("threads", "three") }));
  await vi.advanceTimersByTimeAsync(1);
  complete(true);
  await vi.advanceTimersByTimeAsync(6000);
  expect(refresh).toHaveBeenCalledTimes(4);
  a.stop();
  b.stop();
  f.freshness.destroy();
});
test("an outage enables bounded fallback and recovery stops it without a reload", async () => {
  const f = fixture();
  await f.freshness.configure(true, null);
  const refresh = vi.fn(async () => true);
  const watch = f.freshness.watch({
    matches: () => true,
    refresh,
    initial: false,
  });
  f.streams[0]!.error(new Error("offline"));
  await vi.advanceTimersByTimeAsync(6000);
  expect(f.fetcher).toHaveBeenCalledTimes(2);
  expect(f.freshness.healthy).toBe(true);
  const reads = refresh.mock.calls.length;
  f.send(":keep-alive\n\n");
  await vi.advanceTimersByTimeAsync(15000);
  expect(refresh).toHaveBeenCalledTimes(reads);
  watch.stop();
  f.freshness.destroy();
});

test("ready reconciliation visits every matching scope before continuity advances", async () => {
  const f = fixture();
  await f.freshness.configure(true, null);
  const affected = new Set<string>(),
    refresh = vi.fn(async () => true);
  const watch = f.freshness.watch({
    matches: (c) => {
      affected.add(c.repository!);
      return c.domain === "threads";
    },
    refresh,
    initial: false,
  });
  f.send(
    frame("ready", 1, {
      witnesses: [
        change("threads", "two"),
        { ...change("threads", "three"), repository: "22".repeat(16) },
      ],
    }),
  );
  await vi.advanceTimersByTimeAsync(151);
  expect(affected.size).toBe(2);
  expect(refresh).toHaveBeenCalledOnce();
  watch.stop();
  f.freshness.destroy();
});
