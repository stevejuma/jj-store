import { expect, test, vi } from "vitest";
import { LocalClient } from "../../src/lib/api/local-client.js";
import { decode, pretty } from "../../src/lib/api/client.js";
import { HttpWorkingTree } from "../../src/lib/controller/http-working-tree.js";

const repository = "b".repeat(32),
  operation = "a".repeat(128);
const generation = 18446744073709551615n;
const selection = { bindingGeneration: generation, operation };
const state = (executable = false) => ({
  kind: { type: "file", executable },
  bytes: 12,
  content_digest: "d".repeat(64),
});
function observation() {
  return {
    snapshot: {
      version: 1,
      repository,
      operation,
      binding_generation: generation,
      view: "c".repeat(128),
      workspace: "default",
      working_copy: "1".repeat(40),
      trees: ["2".repeat(40)],
      tree_labels: [],
      physical_workspace: "3".repeat(64),
      metadata_fingerprint: "4".repeat(64),
      physical_fingerprint: "5".repeat(64),
    },
    changes: [
      { path: "added", status: "added", before: null, after: state() },
      { path: "mode", status: "modified", before: state(), after: state(true) },
      { path: "removed", status: "deleted", before: state(), after: null },
    ],
    obstacle_count: 1,
  };
}
function fixture(value: unknown = observation()) {
  const fetcher = vi.fn<typeof fetch>(
    async () =>
      new Response(pretty(value), {
        headers: { "content-type": "application/json" },
      }),
  );
  const client = new LocalClient(repository, fetcher);
  return { fetcher, client, transport: new HttpWorkingTree(client) };
}
test("physical observation preserves exact integers and sends only pinned registered inputs", async () => {
  const { fetcher, transport } = fixture();
  const result = await transport.observe(selection);
  expect(result.snapshot.binding_generation).toBe(generation);
  expect(result.changes[1]?.after?.kind).toEqual({
    type: "file",
    executable: true,
  });
  const [url, init] = fetcher.mock.calls[0]!;
  expect(url).toBe(`/v1/local/repos/${repository}/working-tree/observe`);
  expect(decode(String(init?.body))).toEqual({
    version: 1,
    binding_generation: generation,
    operation,
  });
  expect(init).toMatchObject({
    method: "POST",
    credentials: "omit",
    redirect: "error",
    cache: "no-store",
  });
  expect(init?.headers).toMatchObject({ "x-jj-store-protocol": "1" });
});
test("physical observation rejects invalid request pins before transport", async () => {
  const { fetcher, transport } = fixture();
  for (const bindingGeneration of [
    0,
    -1,
    1.5,
    Number.MAX_SAFE_INTEGER + 1,
    generation + 1n,
  ])
    await expect(
      transport.observe({ ...selection, bindingGeneration }),
    ).rejects.toMatchObject({ code: "invalid_working_tree_selection" });
  await expect(
    transport.observe({ ...selection, operation: "short" }),
  ).rejects.toMatchObject({ code: "invalid_working_tree_selection" });
  expect(fetcher).not.toHaveBeenCalled();
});
test("physical summaries retain ordered conflict labels, symlink states and Rust UTF-8 path ordering", async () => {
  const value = observation();
  value.snapshot.trees = ["1".repeat(40), "2".repeat(40), "3".repeat(40)];
  value.snapshot.tree_labels = ["left", "base", "right"];
  value.changes = ["\ue000", "\u{1f600}"].map((path) => ({
    path,
    status: "added",
    before: null,
    after: state(),
  }));
  value.obstacle_count = 2;
  const result = await fixture(value).transport.observe(selection);
  expect(result.snapshot.tree_labels).toEqual(["left", "base", "right"]);
  expect(result.changes.map((change) => change.path)).toEqual([
    "\ue000",
    "\u{1f600}",
  ]);
  const link = observation() as any;
  link.changes[0].after.kind = { type: "symlink" };
  expect(
    (await fixture(link).transport.observe(selection)).changes[0]?.after?.kind,
  ).toEqual({ type: "symlink" });
});
test("physical observation enforces cumulative byte and complete term bounds", async () => {
  for (const mutate of [
    (v: any) => (v.snapshot.trees = ["1".repeat(40), "2".repeat(40)]),
    (v: any) => (v.snapshot.tree_labels = ["left", "base", "right"]),
    (v: any) => (v.snapshot.workspace = "\ud800"),
    (v: any) => {
      v.changes = [1, 2, 3, 4, 5].map((i) => ({
        path: `file${i}`,
        status: "added",
        before: null,
        after: { ...state(), bytes: 2 * 1024 * 1024 },
      }));
      v.obstacle_count = 5;
    },
  ]) {
    const value = observation();
    mutate(value);
    await expect(
      fixture(value).transport.observe(selection),
    ).rejects.toMatchObject({ code: "invalid_working_tree_observation" });
  }
});
test("physical observation rejects mismatched snapshots, unsafe paths and incoherent complete summaries", async () => {
  const variants: ((value: any) => void)[] = [
    (v) => (v.snapshot.repository = "e".repeat(32)),
    (v) => (v.snapshot.operation = "e".repeat(128)),
    (v) => (v.snapshot.binding_generation = 2),
    (v) => (v.snapshot.view = "e".repeat(64)),
    (v) => (v.snapshot.working_copy = "e".repeat(128)),
    (v) => (v.snapshot.physical_workspace = "short"),
    (v) => (v.snapshot.metadata_fingerprint = "E".repeat(64)),
    (v) => (v.snapshot.version = 2),
    (v) => (v.snapshot.workspace = "bad\nname"),
    (v) => (v.snapshot.trees = []),
    (v) => (v.snapshot.tree_labels = ["wrong"]),
    (v) => (v.snapshot.trees = Array(67).fill("1".repeat(40))),
    (v) => (v.changes[0].path = "/absolute"),
    (v) => (v.changes[0].path = "../escape"),
    (v) => (v.changes[0].path = "nested/.jj/checkout"),
    (v) => (v.changes[0].path = "nested/.Git/config"),
    (v) => (v.changes[0].path = "back\\slash"),
    (v) => (v.changes[0].after.bytes = 2 * 1024 * 1024 + 1),
    (v) => (v.changes[0].after.bytes = Number.MAX_SAFE_INTEGER + 1),
    (v) => (v.changes[0].after.kind.executable = "true"),
    (v) => (v.changes[0].before = state()),
    (v) => (v.changes[1].after = state()),
    (v) => (v.changes[2].before = null),
    (v) => v.changes.reverse(),
    (v) => v.changes.push(v.changes[2]),
    (v) => (v.changes = Array(4097).fill(v.changes[0])),
    (v) => (v.obstacle_count = 4097),
    (v) => (v.obstacle_count = 0),
    (v) => (v.policy = "private bytes"),
    (v) => (v.snapshot.path = "/private/policy"),
    (v) => (v.changes[0].after.contents = "unexpected contents"),
  ];
  for (const mutate of variants) {
    const value = observation();
    mutate(value);
    await expect(
      fixture(value).transport.observe(selection),
    ).rejects.toMatchObject({ code: "invalid_working_tree_observation" });
  }
  for (const value of [
    null,
    {},
    { snapshot: null },
    { ...observation(), changes: null },
  ])
    await expect(
      fixture(value).transport.observe(selection),
    ).rejects.toMatchObject({ code: "invalid_working_tree_observation" });
});
test("caller cancellation and transport close reject late responses", async () => {
  const { fetcher, transport, client } = fixture();
  let finish!: (response: Response) => void;
  fetcher.mockImplementation(
    () => new Promise((resolve) => (finish = resolve)),
  );
  const abort = new AbortController();
  const canceled = transport.observe(selection, abort.signal);
  abort.abort();
  expect(fetcher.mock.calls[0]?.[1]?.signal?.aborted).toBe(true);
  finish(
    new Response(pretty(observation()), {
      headers: { "content-type": "application/json" },
    }),
  );
  await expect(canceled).rejects.toMatchObject({ name: "AbortError" });
  const closed = transport.observe(selection);
  client.close();
  expect(fetcher.mock.calls[1]?.[1]?.signal?.aborted).toBe(true);
  finish(
    new Response(pretty(observation()), {
      headers: { "content-type": "application/json" },
    }),
  );
  await expect(closed).rejects.toMatchObject({ name: "AbortError" });
});
test("cancellation rejects a held response-body read before the late body completes", async () => {
  const { transport, fetcher } = fixture();
  let body!: ReadableStreamDefaultController<Uint8Array>;
  fetcher.mockResolvedValue(
    new Response(
      new ReadableStream({ start: (controller) => (body = controller) }),
      {
        headers: { "content-type": "application/json" },
      },
    ),
  );
  const abort = new AbortController();
  const pending = transport.observe(selection, abort.signal);
  await Promise.resolve();
  abort.abort();
  await expect(pending).rejects.toMatchObject({ name: "AbortError" });
  body.enqueue(new TextEncoder().encode(pretty(observation())));
  body.close();
});
test("observation retains server stale and busy outcomes without retry or code reads", async () => {
  for (const code of [
    "stale_working_tree",
    "working_tree_busy",
    "unsupported_working_tree",
  ]) {
    const { transport, fetcher } = fixture();
    fetcher.mockResolvedValue(
      new Response(pretty({ code }), {
        status: 409,
        headers: { "content-type": "application/json" },
      }),
    );
    await expect(transport.observe(selection)).rejects.toMatchObject({ code });
    expect(fetcher).toHaveBeenCalledTimes(1);
  }
});
