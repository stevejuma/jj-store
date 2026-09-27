import test from "node:test";
import assert from "node:assert/strict";
import { Client, EventParser, decode, pretty } from "../src/lib/api/client.ts";
import { layout, displayable } from "../src/lib/graph.ts";

test("native_numbers_remain_exact_and_fractional_values_fail", () => {
  const value = decode(
    '{"position":18446744073709551615,"time":-9223372036854775808,"small":3}',
  ) as Record<string, number | bigint>;
  assert.equal(value.position, 18446744073709551615n);
  assert.equal(value.time, -9223372036854775808n);
  assert.equal(value.small, 3);
  assert.match(pretty(value), /18446744073709551615/);
  assert.throws(() => decode('{"position":1.5}'));
});
test("event_frames_resume_only_after_complete_validated_data", () => {
  const repo = "11".repeat(16),
    epoch = "22".repeat(16);
  const id = `v1:${repo}:${epoch}:18446744073709551615`;
  const data = `{"version":1,"cursor":{"repository":"${repo}","epoch":"${epoch}","position":18446744073709551615},"type":"changed"}`;
  const seen: string[] = [];
  const parser = new EventParser(repo, (_, id) => seen.push(id));
  parser.push(": heartbeat\r\n\r\n");
  parser.push(`id: ${id}\r\nevent: repository-changed\r\ndata: ${data}\r\n`);
  assert.deepEqual(seen, []);
  parser.push("\r\n");
  assert.deepEqual(seen, [id]);
  assert.throws(() =>
    parser.push(
      `id: v1:${repo}:${epoch}:4\nevent: repository-changed\ndata: ${data}\n\n`,
    ),
  );
  assert.throws(() => new EventParser(repo, () => {}).push("x".repeat(131073)));
});
test("credentials_stay_in_headers_and_close_aborts_pending_requests", async () => {
  const repo = "11".repeat(16);
  let signal: AbortSignal | undefined;
  let uri = "";
  let init: RequestInit | undefined;
  const fetcher = async (input: RequestInfo | URL, options?: RequestInit) => {
    uri = String(input);
    init = options;
    signal = options?.signal ?? undefined;
    return new Response('{"version":1}', {
      headers: { "content-type": "application/json" },
    });
  };
  const client = new Client(repo, "test-memory-token", () => {}, fetcher);
  await client.request("snapshot");
  assert.equal(uri, `/v1/repos/${repo}/snapshot`);
  assert.equal(init?.credentials, "omit");
  assert.equal(init?.redirect, "error");
  assert.equal(
    new Headers(init?.headers).get("authorization"),
    "Bearer test-memory-token",
  );
  client.close();
  assert.equal(signal?.aborted, true);
  await assert.rejects(() => client.request("snapshot"), {
    name: "AbortError",
  });
});
test("revocation_closes_the_client_and_calls_its_owner", async () => {
  let denied = 0;
  const client = new Client(
    "11".repeat(16),
    "test-token",
    () => denied++,
    async () => new Response("", { status: 403 }),
  );
  await assert.rejects(() => client.request("snapshot"), /access denied/);
  assert.equal(denied, 1);
  await assert.rejects(() => client.request("snapshot"), {
    name: "AbortError",
  });
});
test("oversized_json_is_rejected_before_rendering", async () => {
  const client = new Client(
    "11".repeat(16),
    "test-token",
    () => {},
    async () =>
      new Response("x".repeat(16 * 1024 * 1024 + 1), {
        headers: { "content-type": "application/json" },
      }),
  );
  await assert.rejects(() => client.request("snapshot"), /response limit/);
  client.close();
});
test("layout_draws_only_declared_edges_and_marks_off_page_targets", () => {
  const result = layout([
    {
      id: "a",
      title: "",
      subtitle: "",
      badges: [],
      edges: [
        { target: "b", indirect: false },
        { target: "missing", indirect: true },
      ],
    },
    { id: "b", title: "", subtitle: "", badges: [], edges: [] },
  ]);
  assert.equal(result.edges.length, 2);
  assert.equal(result.edges[0].dashed, false);
  assert.equal(result.edges[1].dashed, true);
  assert.equal(result.edges[1].to.y, 192);
});

test("empty_file_range_retries_a_bounded_full_read", async () => {
  const repo = "11".repeat(16),
    operation = "22".repeat(64),
    file = "33".repeat(64);
  const ranges: (string | null)[] = [];
  const client = new Client(
    repo,
    "test-token",
    () => {},
    async (_url, options) => {
      ranges.push(new Headers(options?.headers).get("Range"));
      return ranges.length === 1
        ? new Response("", {
            status: 416,
            headers: { "content-range": "bytes */0" },
          })
        : new Response("", {
            headers: {
              "content-type": "application/octet-stream",
              "x-jj-store-operation": operation,
              "x-jj-store-repository": repo,
            },
          });
    },
  );
  assert.deepEqual(
    await client.file(file, operation, new AbortController().signal),
    { text: "", bytes: 0, range: null, hex: "" },
  );
  assert.deepEqual(ranges, ["bytes=0-262143", null]);
  client.close();
});

test("graph_rendering_bounds_nodes_and_edges_before_layout", () => {
  const graph = {
    data: { graph: "operations", nodes: [{ parents: Array(4097).fill("a") }] },
  } as unknown as import("../src/lib/api/client").Graph;
  assert.equal(displayable(graph), false);
  graph.data.nodes = Array(1001).fill({ parents: [] });
  assert.equal(displayable(graph), false);
  graph.data.nodes = [{ parents: [] }] as typeof graph.data.nodes;
  assert.equal(displayable(graph), true);
});
