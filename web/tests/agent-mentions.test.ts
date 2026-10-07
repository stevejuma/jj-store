import { test } from "node:test";
import assert from "node:assert/strict";
import {
  agentMentionToken,
  mentionedAgentIds,
} from "../src/lib/community/agent-mentions.js";
import { contentMentionPubkeys } from "../src/lib/community/content-mentions.js";
import {
  readAgentAudience,
  writeAgentAudience,
} from "../src/lib/community/agent-audience.js";
test("agent tags preserve exact setup IDs and never become member keys", () => {
  const ids = ["Case-Sensitive", "case-sensitive", "Agent 🦉", "ab".repeat(16)];
  const value = ids.map(agentMentionToken).join(" ");
  assert.deepEqual(mentionedAgentIds(value), ids);
  assert.deepEqual(contentMentionPubkeys(value), []);
  assert.deepEqual(
    mentionedAgentIds(`${agentMentionToken(ids[0]!)}Explain this`),
    [ids[0]],
  );
  assert.deepEqual(
    mentionedAgentIds(`${value} ${agentMentionToken(ids[0]!)}`),
    ids,
  );
  assert.deepEqual(
    mentionedAgentIds("@Case-Sensitive jj-store-agent:ff jj-store-agent:1"),
    [],
  );
});
test("persistent audience is opt-in, scoped and respects removal", () => {
  const values = new Map<string, string>();
  const storage = {
    getItem: (key: string) => values.get(key) ?? null,
    setItem: (key: string, value: string) => values.set(key, value),
  } as Storage;
  assert.deepEqual(readAgentAudience("actor/home/conversation", storage), {
    enabled: false,
    ids: [],
  });
  writeAgentAudience(
    "actor/home/conversation",
    true,
    ["Helper", "helper"],
    storage,
  );
  assert.deepEqual(readAgentAudience("actor/home/conversation", storage), {
    enabled: true,
    ids: ["Helper", "helper"],
  });
  assert.deepEqual(
    readAgentAudience("another-actor/home/conversation", storage),
    { enabled: false, ids: [] },
  );
  writeAgentAudience("actor/home/conversation", false, ["Helper"], storage);
  assert.deepEqual(readAgentAudience("actor/home/conversation", storage), {
    enabled: false,
    ids: [],
  });
});
test("denied audience storage preserves a usable empty preference", () => {
  const denied = {
    getItem() {
      throw new Error("denied");
    },
    setItem() {
      throw new Error("denied");
    },
  } as unknown as Storage;
  assert.doesNotThrow(() =>
    writeAgentAudience("personal", true, ["Helper"], denied),
  );
  assert.deepEqual(readAgentAudience("personal", denied), {
    enabled: false,
    ids: [],
  });
});
