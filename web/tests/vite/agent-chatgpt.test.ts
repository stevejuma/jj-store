import { expect, test, vi } from "vitest";
import { AgentChatGpt } from "../../src/lib/controller/agent-chatgpt.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
import { changeAccount } from "../../src/lib/controller/agent-configuration.js";
const response = (value: unknown, status = 200) =>
  new Response(JSON.stringify(value), {
    status,
    headers: { "content-type": "application/json" },
  });
const initial = () => ({ supported: true, accounts: [], attempts: [] });
test("lost Connect replies retain the exact request identity and never send credentials", async () => {
  const commands: unknown[] = [];
  const client = new LocalClient("", async (_, init) => {
    commands.push(JSON.parse(String(init?.body)));
    if (commands.length === 1) throw new Error("reply lost");
    return response(initial());
  });
  const accounts = new AgentChatGpt(client, () => {});
  try {
    expect(await accounts.connect()).toBe(false);
    expect(accounts.pending?.action).toBe("connect");
    expect(await accounts.connect("another")).toBe(false);
    expect(await accounts.retry()).toBe(true);
    expect(commands).toHaveLength(2);
    expect(commands[1]).toEqual(commands[0]);
    expect(Object.keys(commands[0] as object).sort()).toEqual([
      "account",
      "action",
      "request_id",
    ]);
    expect(accounts.pending).toBeNull();
  } finally {
    accounts.dispose();
  }
});
test("proven command refusal releases the draft; polling recovers after an outage", async () => {
  vi.useFakeTimers();
  let reads = 0;
  const accounts = new AgentChatGpt(
    new LocalClient("", async (_, init) => {
      if (init?.body) return response({ code: "invalid_agent_execution" }, 400);
      reads++;
      if (reads === 2) throw new Error("offline");
      return response({
        ...initial(),
        attempts: [
          {
            id: "attempt",
            account: "one",
            state: reads >= 3 ? "connected" : "authorizing",
            error: null,
          },
        ],
      });
    }),
    () => {},
  );
  try {
    expect(await accounts.connect()).toBe(false);
    expect(accounts.pending).toBeNull();
    expect(await accounts.refresh()).toBe(true);
    await vi.advanceTimersByTimeAsync(1000);
    expect(accounts.error).toBe("offline");
    await vi.advanceTimersByTimeAsync(1000);
    expect(accounts.value?.attempts[0].state).toBe("connected");
    expect(accounts.error).toBe("");
    accounts.dispose();
    await vi.advanceTimersByTimeAsync(5000);
    expect(reads).toBe(3);
  } finally {
    accounts.dispose();
    vi.useRealTimers();
  }
});
test("selecting a different account resets account-specific model and effort only", () => {
  const draft = {
    runner: "local",
    options: {
      inherit_defaults: false,
      provider: "chatgpt-plan",
      account: "one",
      endpoint: null,
      environment: {},
    },
    preferences: { model: "one-model", effort: "high", mode: "ask" },
  };
  changeAccount(draft, "two");
  expect(draft.preferences).toEqual({ model: null, effort: null, mode: "ask" });
  expect(draft.options.account).toBe("two");
});

test("malformed account responses retain the last valid view without an unhandled poll", async () => {
  let valid = true;
  const accounts = new AgentChatGpt(
    new LocalClient("", async () =>
      response(valid ? initial() : { revision: 0 }),
    ),
    () => {},
  );
  try {
    await accounts.refresh();
    valid = false;
    expect(await accounts.refresh()).toBe(false);
    expect(accounts.value).toEqual(initial());
    expect(accounts.error).toBe("Invalid ChatGPT account status");
  } finally {
    accounts.dispose();
  }
});
