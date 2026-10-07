import { expect, test, vi } from "vitest";
import {
  AgentSettings,
  type AgentSettingsValue,
} from "../../src/lib/controller/agent-settings.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";

test("secret writes stay separate from durable defaults and successful saves clear drafts", async () => {
  const calls: { url: string; body: Record<string, unknown> }[] = [];
  const value = {
    revision: 0,
    runner: "codex",
    preferences: { model: null, effort: null, mode: null },
    automatically_mention: false,
    keep_awake: false,
    keychain_supported: true,
    options: {
      inherit_defaults: false,
      provider: null,
      endpoint: null,
      environment: { API_KEY: { source: "keychain", value: "opaque-id" } },
    },
  } as AgentSettingsValue;
  const client = new LocalClient(
    "",
    vi.fn(async (url, init) => {
      const body = JSON.parse(String(init?.body));
      calls.push({ url: String(url), body });
      return Response.json(
        String(url).endsWith("/secrets")
          ? { keychain_supported: true }
          : { ...value, revision: 1 },
      );
    }),
  );
  const settings = new AgentSettings(client);
  const secrets = { "opaque-id": "fixture-credential-value" };
  try {
    expect(await settings.save(value, secrets)).toBe(true);
    expect(calls).toHaveLength(2);
    expect(calls[0]).toEqual({
      url: "/v1/local/agents/secrets",
      body: { id: "opaque-id", value: "fixture-credential-value" },
    });
    expect(JSON.stringify(calls[1])).not.toContain("fixture-credential-value");
    expect(secrets).toEqual({});
    expect(settings.pending).toBeNull();
    expect(settings.value?.revision).toBe(1);
  } finally {
    settings.dispose();
    client.close();
  }
});

test("lost defaults response retains its exact identity and disposal clears credential drafts", async () => {
  const calls: unknown[] = [];
  const client = new LocalClient(
    "",
    vi.fn(async (_url, init) => {
      calls.push(JSON.parse(String(init?.body)));
      throw new TypeError("Offline");
    }),
  );
  const settings = new AgentSettings(client);
  const value = {
    revision: 7,
    options: { environment: {} },
  } as AgentSettingsValue;
  try {
    expect(await settings.save(value)).toBe(false);
    expect(settings.pending).not.toBeNull();
    await settings.retry();
    expect(calls[1]).toEqual(calls[0]);
    settings.secretDrafts = { secret: "fixture-value" };
    settings.setupSecretDrafts = { new: { secret: "fixture-value" } };
    settings.dispose();
    expect(settings.secretDrafts).toEqual({});
    expect(settings.setupSecretDrafts).toEqual({});
  } finally {
    settings.dispose();
    client.close();
  }
});
