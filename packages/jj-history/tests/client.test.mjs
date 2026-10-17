import test from "node:test";
import assert from "node:assert/strict";
import { stringify } from "lossless-json";
import {
  HistoryClient,
  HistoryProtocolError,
  MAX_REQUEST_BYTES,
  MAX_RESPONSE_BYTES,
} from "../dist/index.js";
import { tauriHistory } from "../dist/tauri.js";
import { encode, decode } from "../dist/wire.js";
const operation = "ab".repeat(64);
const repository = "cd".repeat(16);
const handle = "3f7ead75-e6d2-43f0-92dd-f7f3236bc634";
const frame = (data) => stringify({ version: 1, data });
const heads = {
  version: 1,
  profile: "local",
  repository,
  generation: 18446744073709551615n,
  heads: [operation],
};
const queryInfo = {
  version: 1,
  jj_version: "0.45.1",
  max_query_bytes: 128,
  callables: [],
  aliases: [],
  operators: [],
  operation_fields: [],
  unsupported: [],
};

test("one vault uses exact commands, explicit handle lifetime and lossless native integers", async () => {
  const calls = [];
  const client = new HistoryClient("vault 🦉", {
    invoke: async (command, request) => {
      calls.push([command, JSON.parse(request)]);
      return frame(
        {
          heads,
          open: handle,
          status: {
            state: "ready",
            data: {
              version: 1,
              profile: "local",
              repository,
              generation: 18446744073709551615n,
              operation,
            },
          },
          read: { kind: "query_info", data: queryInfo },
          close: null,
        }[command],
      );
    },
  });
  assert.deepEqual(await client.heads(), heads);
  const selected = await client.open(operation);
  assert.equal(selected, handle);
  assert.equal(
    (await client.status(handle)).data.generation,
    18446744073709551615n,
  );
  assert.deepEqual(await client.read(handle, { kind: "query_info" }), {
    kind: "query_info",
    data: queryInfo,
  });
  assert.equal(await client.close(handle), undefined);
  assert.deepEqual(
    calls.map((c) => c[0]),
    ["heads", "open", "status", "read", "close"],
  );
  for (const [, value] of calls) {
    assert.equal(value.version, 1);
    assert.equal(value.data.vault, "vault 🦉");
    assert.equal(Object.hasOwn(value.data, "root"), false);
  }
  assert.equal(calls[1][1].data.operation, operation);
  assert.equal(calls[3][1].data.handle, handle);
});

test("timestamps, unsigned ranges, UTF-8 and escaped strings retain exact values", () => {
  const nodes = [
    {
      id: operation,
      parents: [],
      view: operation,
      description: '笔记 "quote"\\line',
      user: "u",
      host: "h",
      start: { millis: -9223372036854775808n, offset_minutes: 0 },
      end: { millis: 9223372036854775807n, offset_minutes: 0 },
      is_snapshot: false,
      attributes: {},
      predecessor_data_present: false,
    },
  ];
  const response = {
    kind: "graph",
    data: {
      version: 1,
      operation,
      data: { graph: "operations", nodes },
      stats: { visited: 0, bytes: 0, elapsed_millis: 0 },
    },
  };
  response.data.stats = {
    metadata_nodes: 0,
    predicate_file_bytes: 0,
    predicate_lines: 0,
    object_reads: 0,
    object_bytes: 0,
    work: 18446744073709551615n,
  };
  assert.deepEqual(decode("read_response", frame(response)), response);
  const request = encode("read_request", {
    vault: "v",
    handle,
    query: {
      kind: "graph",
      query: [
        {
          graph: "log",
          revset: "all()",
          user_email: "u",
          at_time_millis: -9223372036854775808n,
        },
        { limit: 18446744073709551615n },
      ],
    },
  });
  assert.ok(request.includes("-9223372036854775808"));
  assert.ok(request.includes("18446744073709551615"));
  assert.equal(
    decode("heads_response", frame({ ...heads, generation: 42 })).generation,
    42,
  );
  for (const generation of [-1n, 18446744073709551616n])
    assert.throws(
      () => decode("heads_response", frame({ ...heads, generation })),
      HistoryProtocolError,
    );
  const outer = JSON.parse(JSON.stringify(frame(heads)));
  assert.equal(decode("heads_response", outer).generation, heads.generation);
});

test("unsafe inputs, extra authority, invalid identities and tuples fail before invocation", async () => {
  let calls = 0;
  const client = new HistoryClient("v", {
    invoke: async () => {
      calls++;
      return frame(null);
    },
  });
  await assert.rejects(client.open("short"), HistoryProtocolError);
  for (const limit of [
    Number.MAX_SAFE_INTEGER + 1,
    -1,
    18446744073709551616n,
    1.5,
    Infinity,
  ]) {
    await assert.rejects(
      client.read(handle, { kind: "code_summary", query: { limit } }),
      HistoryProtocolError,
    );
  }
  await assert.rejects(
    client.read(handle, { kind: "graph", query: [{ graph: "operations" }] }),
    HistoryProtocolError,
  );
  assert.throws(
    () => encode("heads_request", { vault: "v", root: "outside" }),
    HistoryProtocolError,
  );
  assert.throws(
    () =>
      encode("heads_request", { vault: "🦉".repeat(MAX_REQUEST_BYTES / 4) }),
    HistoryProtocolError,
  );
  assert.throws(
    () =>
      encode("read_request", {
        vault: "v",
        handle,
        query: {
          kind: "graph",
          query: [
            {
              graph: "log",
              revset: "all()",
              user_email: "u",
              at_time_millis: -9223372036854775809n,
            },
            { limit: 1 },
          ],
        },
      }),
    HistoryProtocolError,
  );
  assert.equal(calls, 0);
});

test("malformed, duplicate, versioned and oversized replies never become typed data", () => {
  for (const text of [
    null,
    {},
    "{",
    '{"version":1,"version":1,"data":null}',
    frame({}),
    '{"version":2,"data":null}',
    '{"version":1e0,"data":null}',
    '{"version":1,"data":null,"window":"other"}',
    " ".repeat(MAX_RESPONSE_BYTES + 1),
  ]) {
    assert.throws(() => decode("close_response", text), HistoryProtocolError);
  }
  assert.throws(
    () => decode("heads_response", frame({ ...heads, heads: ["wrong"] })),
    HistoryProtocolError,
  );
  assert.throws(
    () =>
      decode(
        "heads_response",
        frame(heads).replace("18446744073709551615", "9007199254740992.5"),
      ),
    HistoryProtocolError,
  );
});

test("lost outcomes propagate once, failed close remains caller-owned and responses match the query", async () => {
  let calls = 0;
  const failure = new Error("unknown outcome");
  const client = new HistoryClient("v", {
    invoke: async () => {
      calls++;
      throw failure;
    },
  });
  await assert.rejects(client.open(operation), (e) => e === failure);
  assert.equal(calls, 1);
  await assert.rejects(client.close(handle), (e) => e === failure);
  assert.equal(calls, 2);
  const mismatched = new HistoryClient("v", {
    invoke: async () => frame({ kind: "query_info", data: queryInfo }),
  });
  await assert.rejects(
    mismatched.read(handle, {
      kind: "commit",
      query: { commit: "ab".repeat(20) },
    }),
    HistoryProtocolError,
  );
});

test("optional Tauri entry uses exact namespace and only a text request", async () => {
  let call;
  const client = tauriHistory("v", async (...args) => {
    call = args;
    return frame(heads);
  });
  await client.heads();
  assert.deepEqual(call, [
    "plugin:jj-store|heads",
    { request: '{"version":1,"data":{"vault":"v"}}' },
  ]);
});

test("record keys preserve proto spelling and repeated escaped keys reject", () => {
  const info = { ...queryInfo, aliases: ["__proto__", "constructor"] };
  assert.deepEqual(
    decode("read_response", frame({ kind: "query_info", data: info })).data
      .aliases,
    info.aliases,
  );
  for (const text of [
    '{"version":1,"data":null,"da\\u0074a":null}',
    '{"version":1,"data":null,"data":{}}',
    '{"version":1,"data":null,}',
    '{"version":01,"data":null}',
    '{"version":1.0,"data":null}',
    "[".repeat(130) + "]".repeat(130),
  ])
    assert.throws(() => decode("close_response", text), HistoryProtocolError);
  const source = {
    kind: "graph",
    data: {
      version: 1,
      operation,
      data: {
        graph: "operations",
        nodes: [
          {
            id: operation,
            parents: [],
            view: operation,
            description: "",
            user: "u",
            host: "h",
            start: { millis: 0, offset_minutes: 0 },
            end: { millis: 0, offset_minutes: 0 },
            is_snapshot: false,
            attributes: JSON.parse(
              '{"__proto__":"own","constructor":"also own"}',
            ),
            predecessor_data_present: false,
          },
        ],
      },
      stats: {
        metadata_nodes: 0,
        predicate_file_bytes: 0,
        predicate_lines: 0,
        object_reads: 0,
        object_bytes: 0,
        work: 0,
      },
    },
  };
  const result = decode("read_response", frame(source));
  assert.deepEqual(result, source);
  assert.ok(Object.hasOwn(result.data.data.nodes[0].attributes, "__proto__"));
});
