import test from "node:test";
import assert from "node:assert/strict";
import {
  agentMentionToken,
  agentMentionMatches,
  mentionedAgentIds,
  authoredPrompt,
  promptDelivery,
  promptDeliveryObservations,
} from "@lapismd/jj-conversations";

test("canonical mention tokens retain exact setup IDs and separate attribution from keys", () => {
  const ids = ["Helper", "helper", "Agent 🦉", "ab".repeat(16)];
  const value = ids.map(agentMentionToken).join(" ");
  assert.deepEqual(
    mentionedAgentIds(value + " " + agentMentionToken(ids[0])),
    ids,
  );
  assert.deepEqual(
    agentMentionMatches(value).map((match) => match.agentId),
    ids,
  );
  assert.ok(
    agentMentionMatches(value).every(
      (match) => !Object.hasOwn(match, "pubkey"),
    ),
  );
  assert.equal(authoredPrompt(`${value} Explain this`), "Explain this");
  for (const invalid of [
    "jj-store-agent:ff;",
    "jj-store-agent:0;",
    "jj-store-agent:FF;",
    agentMentionToken("bad\nname"),
    agentMentionToken("a".repeat(65)),
  ]) {
    assert.deepEqual(agentMentionMatches(invalid), []);
  }
});

test("delivery views retain full-width sequences and late queue receipts cannot revive completion", () => {
  const event = (sequence, type, delivery) => ({
    sequence,
    payload: { requestId: "request", type, sentContext: { delivery } },
  });
  const sequence = 9007199254740995n;
  const events = [
    event(sequence, "started"),
    event(sequence + 1n, "context_delivery", "queued"),
  ];
  assert.equal(promptDelivery(events, "request"), "started");
  assert.equal(
    promptDeliveryObservations(events).get("request").sequence,
    sequence + 1n,
  );
  assert.equal(
    promptDelivery(
      [
        ...events,
        event(sequence + 2n, "record"),
        event(sequence + 3n, "context_delivery", "sent"),
      ],
      "request",
    ),
    "record",
  );
  assert.equal(
    promptDelivery([event(sequence, "context_delivery", "unknown")], "request"),
    "unknown",
  );
});
