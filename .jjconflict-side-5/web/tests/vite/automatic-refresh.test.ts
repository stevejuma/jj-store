import { afterEach, expect, test, vi } from "vitest";
import { createAutomaticRefresh } from "../../src/lib/controller/automatic-refresh.js";
afterEach(() => vi.useRealTimers());
function fixture() {
  vi.useFakeTimers();
  const abort = new AbortController();
  const refresh = vi.fn(async () => true),
    poll = vi.fn(async () => false),
    report = vi.fn();
  const updates = createAutomaticRefresh({
    active: () => true,
    refresh,
    poll,
    report,
    signal: abort.signal,
  });
  return { abort, updates, refresh, poll, report };
}
test("live events coalesce and an event during a reload schedules another reload", async () => {
  const f = fixture();
  let resolve!: (accepted: boolean) => void;
  f.refresh.mockImplementationOnce(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  f.updates.request();
  f.updates.request();
  f.updates.request();
  await vi.advanceTimersByTimeAsync(150);
  expect(f.refresh).toHaveBeenCalledTimes(1);
  f.updates.request();
  resolve(true);
  await vi.advanceTimersByTimeAsync(5000);
  expect(f.refresh).toHaveBeenCalledTimes(2);
  f.abort.abort();
});
test("edit pauses defer updates and release reloads without a manual action", async () => {
  const f = fixture();
  f.updates.pause("editor", true);
  f.updates.request();
  await vi.advanceTimersByTimeAsync(6000);
  expect(f.refresh).not.toHaveBeenCalled();
  f.updates.pause("editor", false);
  await vi.advanceTimersByTimeAsync(150);
  expect(f.refresh).toHaveBeenCalledTimes(1);
  f.abort.abort();
});
test("missed data is detected by periodic reads and failures retry automatically", async () => {
  const f = fixture();
  f.poll.mockResolvedValueOnce(true);
  f.refresh.mockRejectedValueOnce(new Error("offline"));
  await vi.advanceTimersByTimeAsync(5000);
  expect(f.report).toHaveBeenCalledTimes(1);
  await vi.advanceTimersByTimeAsync(5000);
  expect(f.refresh).toHaveBeenCalledTimes(2);
  f.abort.abort();
  await vi.advanceTimersByTimeAsync(10000);
  expect(f.refresh).toHaveBeenCalledTimes(2);
});
test("session reset discards late polls and queued work", async () => {
  const f = fixture();
  let resolve!: (changed: boolean) => void;
  f.poll.mockImplementationOnce(
    () =>
      new Promise((done) => {
        resolve = done;
      }),
  );
  await vi.advanceTimersByTimeAsync(5000);
  f.updates.reset();
  resolve(true);
  await vi.advanceTimersByTimeAsync(5000);
  expect(f.refresh).not.toHaveBeenCalled();
  f.abort.abort();
});

test("unrelated healthy notices cannot postpone a queued event read", async () => {
  vi.useFakeTimers();
  const abort = new AbortController();
  const refresh = vi.fn(async () => true),
    poll = vi.fn(async () => false);
  const updates = createAutomaticRefresh({
    active: () => true,
    refresh,
    poll,
    report: () => {},
    signal: abort.signal,
    fallback: () => false,
  });
  updates.request();
  await vi.advanceTimersByTimeAsync(20);
  updates.reschedule();
  await vi.advanceTimersByTimeAsync(100);
  updates.reschedule();
  await vi.advanceTimersByTimeAsync(30);
  expect(refresh).toHaveBeenCalledOnce();
  await vi.advanceTimersByTimeAsync(60000);
  expect(refresh).toHaveBeenCalledOnce();
  expect(poll).not.toHaveBeenCalled();
  abort.abort();
});

test("a deferred healthy event retries promptly and stops after an accepted read", async () => {
  vi.useFakeTimers();
  const abort = new AbortController();
  const refresh = vi.fn(async () => true).mockResolvedValueOnce(false);
  const updates = createAutomaticRefresh({
    active: () => true,
    refresh,
    poll: async () => false,
    report: () => {},
    signal: abort.signal,
    fallback: () => false,
  });
  updates.request();
  await vi.advanceTimersByTimeAsync(150);
  expect(refresh).toHaveBeenCalledOnce();
  await vi.advanceTimersByTimeAsync(551);
  expect(refresh).toHaveBeenCalledTimes(2);
  await vi.advanceTimersByTimeAsync(60000);
  expect(refresh).toHaveBeenCalledTimes(2);
  abort.abort();
});
