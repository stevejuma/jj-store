import { expect, test, vi } from "vitest";
import { QueryClient } from "@tanstack/svelte-query";
import { createHttpBlame } from "../../src/lib/controller/http-blame.js";
import { ApiError, type Model } from "../../src/lib/api/client.js";
import type { BlameViewState } from "../../src/lib/controller/blame-types.js";
const selection = {
  repository: "ab".repeat(16),
  operation: "cd".repeat(64),
  commit: "ef".repeat(20),
  path: "file.txt",
  lineCount: 2,
};
const summary = {
  id: selection.commit,
  change_id: "34".repeat(16),
  description: "Real commit\nbody",
  author: {
    name: "Writer",
    email: "writer@example.invalid",
    timestamp: { millis: 1000, offset_minutes: 0 },
  },
  committer: {
    name: "Writer",
    email: "writer@example.invalid",
    timestamp: { millis: 2000, offset_minutes: 0 },
  },
};
const page: Model<"BlamePage"> = {
  version: 1,
  repository: selection.repository,
  operation: selection.operation,
  commit: selection.commit,
  path: selection.path,
  file_id: "12".repeat(20),
  line_count: 2,
  ranges: [{ start: 1, end: 3, commit_id: selection.commit }],
  commits: { [selection.commit]: summary },
  complete: true,
  continuation: null,
  history_commits: 1,
  stats: {
    metadata_nodes: 1,
    object_reads: 1,
    object_bytes: 1,
    work: 1,
    predicate_file_bytes: 0,
    predicate_lines: 0,
  },
};
test("HTTP blame maps real origins and completed revisits perform no reads", async () => {
  const client = new QueryClient();
  const request = vi.fn(async () => page);
  const provider = createHttpBlame({
    client,
    scope: "native-epoch",
    active: () => true,
    request,
  });
  const states: BlameViewState[] = [];
  const stop = provider.observe(selection, (s) => states.push(s));
  await vi.waitFor(() => expect(states.at(-1)?.status).toBe("ready"));
  expect(states.at(-1)?.progress?.commits[selection.commit]).toMatchObject({
    commitId: selection.commit,
    author: "Writer",
    message: "Real commit\nbody",
  });
  stop();
  const again = provider.observe(selection, () => {});
  expect(request).toHaveBeenCalledTimes(1);
  again();
  client.clear();
});
test("preparation reports state and navigation cancels retry", async () => {
  const client = new QueryClient();
  const request = vi.fn(async () => {
    throw new ApiError("index_rebuild_required");
  });
  const states: BlameViewState[] = [];
  const provider = createHttpBlame({
    client,
    scope: "local-generation",
    active: () => true,
    request,
  });
  const stop = provider.observe(selection, (s) => states.push(s));
  await vi.waitFor(() => expect(states.at(-1)?.preparing).toBe(true));
  stop();
  await new Promise((resolve) => setTimeout(resolve, 1100));
  expect(request).toHaveBeenCalledTimes(1);
  client.clear();
});
test("expired progress restarts only the same selection and foreign snapshots fail", async () => {
  const client = new QueryClient();
  const request = vi
    .fn()
    .mockRejectedValueOnce(new ApiError("blame_reset"))
    .mockResolvedValue(page);
  const states: BlameViewState[] = [];
  const provider = createHttpBlame({
    client,
    scope: "reset",
    active: () => true,
    request,
  });
  const stop = provider.observe(selection, (s) => states.push(s));
  await vi.waitFor(() => expect(states.at(-1)?.status).toBe("ready"));
  expect(request.mock.calls.map((c) => c.slice(0, 2))).toEqual([
    [selection, null],
    [selection, null],
  ]);
  stop();
  client.clear();
  const bad = createHttpBlame({
    client,
    scope: "bad",
    active: () => true,
    request: async () => ({ ...page, operation: "other" }),
  });
  const failures: BlameViewState[] = [];
  const end = bad.observe(selection, (s) => failures.push(s));
  await vi.waitFor(() => expect(failures.at(-1)?.status).toBe("failed"));
  end();
  client.clear();
});

test("terminal preparation failures stop polling", async () => {
  const client = new QueryClient();
  const request = vi.fn(async () => {
    throw new ApiError("resource_limit");
  });
  const states: BlameViewState[] = [];
  const provider = createHttpBlame({
    client,
    scope: "failure",
    active: () => true,
    request,
  });
  const stop = provider.observe(selection, (s) => states.push(s));
  await vi.waitFor(() => expect(states.at(-1)?.status).toBe("failed"));
  await new Promise((resolve) => setTimeout(resolve, 1100));
  expect(request).toHaveBeenCalledTimes(1);
  stop();
  client.clear();
});
