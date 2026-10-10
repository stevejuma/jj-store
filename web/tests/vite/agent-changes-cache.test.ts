import { expect, test, vi } from "vitest";
import { AgentChangesCache } from "../../src/lib/controller/agent-changes-cache.js";
import type { AgentChangeSelection } from "../../src/lib/controller/agent-workspace-changes.js";
const selection = {
  repository: "repo",
  profile: "native",
  operation: "op",
  base_commit: "before",
  commit: "after",
} as AgentChangeSelection;

test("shared immutable previews survive one observer leaving and are reused on remount", async () => {
  let finish!: () => void;
  const read = vi.fn(async (_selection, _fetcher, signal: AbortSignal) => {
    await new Promise<void>((resolve) => {
      finish = resolve;
    });
    signal.throwIfAborted();
    return { files: [], complete: true };
  });
  const cache = new AgentChangesCache(fetch, read),
    a = new AbortController(),
    b = new AbortController();
  const first = cache.read(selection, a.signal).catch(() => "aborted");
  const second = cache.read(selection, b.signal);
  a.abort();
  finish();
  expect(await first).toBe("aborted");
  expect(await second).toEqual({ files: [], complete: true });
  await cache.read(selection, new AbortController().signal);
  expect(read).toHaveBeenCalledTimes(1);
  cache.destroy();
});

test("last observer cancellation and failed reads cannot establish cached success", async () => {
  const read = vi.fn(async (_selection, _fetcher, signal: AbortSignal) => {
    await new Promise<void>((_resolve, reject) =>
      signal.addEventListener("abort", () => reject(signal.reason), {
        once: true,
      }),
    );
    return { files: [], complete: true };
  });
  const cache = new AgentChangesCache(fetch, read),
    abort = new AbortController();
  const first = cache.read(selection, abort.signal).catch(() => "aborted");
  abort.abort();
  expect(await first).toBe("aborted");
  read.mockRejectedValueOnce(new Error("Unavailable"));
  await expect(
    cache.read(selection, new AbortController().signal),
  ).rejects.toThrow("Unavailable");
  read.mockResolvedValueOnce({ files: [], complete: true });
  await cache.read(selection, new AbortController().signal);
  expect(read).toHaveBeenCalledTimes(3);
  cache.destroy();
});

test("comparison identity and bounded retention prevent wrong or unbounded reuse", async () => {
  const read = vi.fn(async () => ({ files: [], complete: true }));
  const cache = new AgentChangesCache(fetch, read),
    signal = new AbortController().signal;
  for (let i = 0; i < 9; i++)
    await cache.read({ ...selection, operation: String(i) }, signal);
  await cache.read({ ...selection, operation: "8" }, signal);
  expect(read).toHaveBeenCalledTimes(9);
  await cache.read({ ...selection, operation: "0" }, signal);
  await cache.read(
    { ...selection, operation: "0", base_commit: "other" },
    signal,
  );
  expect(read).toHaveBeenCalledTimes(11);
  cache.destroy();
});
