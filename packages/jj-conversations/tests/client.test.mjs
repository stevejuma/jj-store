import test from "node:test";
import assert from "node:assert/strict";
import {
  ConversationClient,
  ConversationClientError,
} from "@lapismd/jj-conversations";
const request = (id = "request", text = "Hello") => ({ request_id: id, text });
const encoder = JSON.stringify;
const receipt = (request_id, delivery = "started") => ({
  request_id,
  publication: "committed",
  delivery,
  capture: "pending",
  sequence: 9007199254740995n,
});
const code = (name) => (error) =>
  error instanceof ConversationClientError && error.code === name;

test("concurrent callers share one exact submission and input/receipt mutations stay isolated", async () => {
  let finish,
    calls = 0,
    encoded;
  const client = new ConversationClient(
    "vault-a",
    {
      submit: async (value) => {
        calls++;
        encoded = value;
        return new Promise((resolve) => {
          finish = resolve;
        });
      },
      lookup: async () => undefined,
    },
    encoder,
  );
  const command = request();
  const first = client.send(command),
    second = client.send({ ...command });
  command.text = "changed after admission";
  await Promise.resolve();
  assert.equal(calls, 1);
  assert.equal(encoded.body, JSON.stringify(request()));
  assert.ok(Object.isFrozen(encoded));
  const native = receipt("request");
  finish(native);
  const a = await first,
    b = await second;
  a.delivery = "mutated";
  native.request_id = "foreign";
  assert.equal(b.delivery, "started");
  assert.equal(client.snapshot("request").receipt.request_id, "request");
  assert.equal((await client.send(request())).sequence, 9007199254740995n);
  assert.equal(calls, 1);
  assert.throws(() => client.send(command), code("identity_conflict"));
});

test("response loss keeps the original request unknown and refresh never submits", async () => {
  let posts = 0,
    reads = 0,
    available;
  const client = new ConversationClient(
    "vault-a",
    {
      submit: async () => {
        posts++;
        throw new Error("private transport detail");
      },
      lookup: async (id) => {
        reads++;
        assert.equal(id, "request");
        return available;
      },
    },
    encoder,
  );
  await assert.rejects(client.send(request()), code("unknown_outcome"));
  await assert.rejects(client.send(request()), code("unknown_outcome"));
  assert.equal(client.snapshot("request").state, "unknown");
  assert.equal(await client.refresh("request"), undefined);
  assert.equal(client.snapshot("request").state, "unknown");
  available = receipt("request", "outcome_unknown");
  const value = await client.refresh("request");
  assert.equal(value.publication, "committed");
  assert.equal(value.delivery, "outcome_unknown");
  assert.equal(value.capture, "pending");
  assert.equal(client.snapshot("request").state, "receipt");
  assert.equal(posts, 1);
  assert.equal(reads, 2);
});

test("restart restores only a scoped identity and cannot manufacture a new command body", async () => {
  let posts = 0;
  const client = new ConversationClient(
    "vault-a",
    {
      submit: async () => {
        posts++;
        return receipt("request");
      },
      lookup: async (id) => receipt(id),
    },
    encoder,
  );
  assert.equal(client.restoreUnknown("request").state, "unknown");
  assert.throws(() => client.send(request()), code("unresolved"));
  assert.equal((await client.refresh("request")).sequence, 9007199254740995n);
  assert.equal(posts, 0);
});

test("foreign receipts never replace retained identity or permit another submission", async () => {
  let posts = 0;
  const client = new ConversationClient(
    "vault-a",
    {
      submit: async () => {
        posts++;
        return receipt("foreign");
      },
      lookup: async () => receipt("foreign"),
    },
    encoder,
  );
  await assert.rejects(
    client.send(request()),
    code("receipt_identity_mismatch"),
  );
  await assert.rejects(
    client.refresh("request"),
    code("receipt_identity_mismatch"),
  );
  assert.equal(client.snapshot("request").receipt, undefined);
  assert.equal(client.snapshot("request").state, "unknown");
  await assert.rejects(client.send(request()), code("unknown_outcome"));
  assert.equal(posts, 1);
});

test("capacity never evicts unknown work and finality remains an explicit host decision", async () => {
  const client = new ConversationClient(
    "vault-a",
    {
      submit: async ({ request_id }) => receipt(request_id),
      lookup: async (id) => receipt(id),
    },
    encoder,
    { maxEntries: 1 },
  );
  client.restoreUnknown("old");
  assert.throws(() => client.prepare(request("new")), code("capacity"));
  assert.throws(() => client.retire("old", () => true), code("unresolved"));
  await client.refresh("old");
  assert.throws(() => client.retire("old", () => false), code("unresolved"));
  client.retire(
    "old",
    (value) =>
      value.publication === "committed" && value.delivery === "started",
  );
  assert.equal((await client.send(request("new"))).request_id, "new");
  assert.equal((await client.drain()).length, 1);
});

test("request scopes share no ambient ledger and errors contain no private transport text", async () => {
  const make = (scope) =>
    new ConversationClient(
      scope,
      {
        submit: async () => {
          throw new Error("private root /secret/home credential");
        },
        lookup: async () => {
          throw new Error("private status detail");
        },
      },
      encoder,
    );
  const first = make("vault-a"),
    second = make("vault-b");
  await assert.rejects(
    first.send(request()),
    (error) =>
      !error.message.includes("private") && code("unknown_outcome")(error),
  );
  assert.equal(second.snapshot("request"), undefined);
  await assert.rejects(
    first.refresh("request"),
    (error) =>
      !error.message.includes("private") && code("unknown_outcome")(error),
  );
  assert.throws(
    () => first.prepare(request("bad\nidentity")),
    code("invalid_request"),
  );
});
