import { expect, test } from "vitest";
import { LocalInboxThreads } from "../../src/lib/controller/local-inbox-threads.js";
import { threadSnapshotKey } from "../../src/lib/controller/thread-types.js";
const repository = "11".repeat(16),
  actor = "22".repeat(16),
  commit = "33".repeat(20);
const snapshot = {
  profile: "local_jj_git",
  repo_id: repository,
  metadata_commit: commit,
  state_version: 4,
};
const json = (value: unknown) =>
  new Response(JSON.stringify(value), {
    headers: { "content-type": "application/json" },
  });
test("local inbox commands retain exact IDs and use the device actor and local domain contract", async () => {
  const calls: { path: string; init?: RequestInit }[] = [];
  const fetcher = async (input: RequestInfo | URL, init?: RequestInit) => {
    const path = String(input);
    calls.push({ path, init });
    if (path.endsWith("/snapshot")) return json({ snapshot, actor });
    const command = JSON.parse(String(init?.body));
    return json({ status: "outcome_unknown", request_id: command.request_id });
  };
  const source = new LocalInboxThreads(repository, fetcher as typeof fetch);
  await source.prepare();
  expect(source.actor).toBe(actor);
  const request = {
    version: 1 as const,
    request_id: "44".repeat(16),
    command: {
      type: "create" as const,
      thread_id: "55".repeat(16),
      title: "Exact",
      baseline: null,
    },
  };
  await source.command("threads", request);
  expect(calls.at(-1)?.path).toBe(
    `/v1/local/repos/${repository}/collaboration/commands`,
  );
  expect(JSON.parse(String(calls.at(-1)?.init?.body))).toEqual({
    ...request,
    command: { domain: "thread", intent: request.command },
  });
  expect(calls.every((c) => c.init?.credentials === "omit")).toBe(true);
  source.close();
});
test("local thread and event reads share a companion pin without native snapshot fields", async () => {
  let selections = 0;
  const bodies: unknown[] = [];
  const source = new LocalInboxThreads(repository, (async (input, init) => {
    if (String(input).endsWith("/snapshot")) {
      selections++;
      return json({ snapshot, actor });
    }
    const body = JSON.parse(String(init?.body));
    bodies.push(body);
    if (String(input).endsWith("/thread"))
      return json({
        snapshot,
        thread: { reference: { kind: "thread_version", id: "66".repeat(32) } },
      });
    return json({
      data: {
        snapshot,
        items: { type: "events", thread_id: "55".repeat(16), events: [] },
      },
      next_cursor: null,
    });
  }) as typeof fetch);
  const [detail, page] = await Promise.all([
    source.thread("55".repeat(16)),
    source.events("55".repeat(16)),
  ]);
  expect(selections).toBe(1);
  expect(
    bodies.every(
      (b) => (b as { metadata_commit: string }).metadata_commit === commit,
    ),
  ).toBe(true);
  expect(threadSnapshotKey(detail.snapshot)).toBe(
    threadSnapshotKey(page.snapshot),
  );
  expect(detail.snapshot).not.toHaveProperty("epoch");
  expect(detail.snapshot).not.toHaveProperty("published_seq");
  await source.invalidate("threads");
  await source.events("55".repeat(16));
  expect(selections).toBe(2);
  source.close();
});
test("local inbox rejects foreign pins and invalid retained image bytes", async () => {
  const source = new LocalInboxThreads(repository, (async (input) =>
    String(input).endsWith("/snapshot")
      ? json({ snapshot: { ...snapshot, repo_id: "99".repeat(16) }, actor })
      : json({ media_type: "png", bytes: [300] })) as typeof fetch);
  await expect(source.prepare()).rejects.toMatchObject({
    code: "local_identity_changed",
  });
  await expect(source.asset("66".repeat(32))).rejects.toMatchObject({
    code: "invalid_reaction_asset",
  });
  source.close();
});

test("local repository conversations expose the companion transport and close its reads", async () => {
  const { HttpLocalStoreDataSource } =
    await import("../../src/lib/controller/http-local-store-data-source.js");
  const source = new HttpLocalStoreDataSource(repository, (async (input) =>
    String(input).endsWith("/snapshot")
      ? json({ snapshot, actor })
      : json({
          data: { snapshot, items: { type: "threads", threads: [] } },
          next_cursor: null,
        })) as typeof fetch);
  expect(source.threads).toBeDefined();
  await expect(source.threads!.list()).resolves.toMatchObject({
    snapshot,
    items: { type: "threads" },
  });
  source.close();
  await expect(source.threads!.list()).rejects.toThrow();
});
