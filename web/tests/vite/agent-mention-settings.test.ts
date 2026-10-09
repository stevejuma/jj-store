import { expect, test, vi } from "vitest";
import {
  AgentSettings,
  type AgentSettingsValue,
} from "../../src/lib/controller/agent-settings.svelte.js";
import { LocalClient } from "../../src/lib/api/local-client.js";
const initial = (): AgentSettingsValue => ({
  revision: 0,
  runner: "codex",
  preferences: { model: null, effort: null, mode: null },
  automatically_mention: false,
  keep_awake: false,
  keychain_supported: true,
  metadata: null,
  options: {
    inherit_defaults: false,
    provider: null,
    endpoint: null,
    environment: {},
  },
});
for (const lost of [false, true])
  test(`mention preference preserves unsaved defaults and secrets${lost ? " after a lost response" : ""}`, async () => {
    let saved = initial();
    const calls: Record<string, any>[] = [];
    const client = new LocalClient(
      "",
      vi.fn(async (_url, init) => {
        if (!init?.body) return Response.json(saved);
        const body = JSON.parse(String(init.body));
        calls.push(body);
        if (calls.length === 1) {
          saved = { ...body.settings, revision: 1 };
          if (lost) throw new TypeError("Offline");
        }
        return Response.json(saved);
      }),
    );
    const settings = new AgentSettings(client);
    try {
      await settings.refresh();
      settings.draft = initial();
      settings.draft.preferences.model = "unsaved-model";
      settings.secretDrafts = { draft: "unsaved-secret" };
      expect(await settings.setAutomaticallyMention(true)).toBe(!lost);
      if (lost) {
        expect(settings.value?.automatically_mention).toBe(false);
        expect(settings.pending).not.toBeNull();
        expect(await settings.retry()).toBe(true);
        expect(calls[1]).toEqual(calls[0]);
      }
      expect(saved.preferences.model).toBeNull();
      expect(saved.automatically_mention).toBe(true);
      expect(settings.draft.preferences.model).toBe("unsaved-model");
      expect(settings.draft.automatically_mention).toBe(true);
      expect(settings.draft.revision).toBe(1);
      expect(settings.secretDrafts).toEqual({ draft: "unsaved-secret" });
      expect(JSON.stringify(calls)).not.toContain("unsaved");
      saved = { ...saved, revision: 2, automatically_mention: false };
      await settings.refresh();
      expect(settings.draft.automatically_mention).toBe(false);
      expect(settings.draft.revision).toBe(1); // External edits still need explicit rebase.
      expect(settings.draft.preferences.model).toBe("unsaved-model");
    } finally {
      settings.dispose();
      client.close();
    }
  });

test("an older settings observation cannot replace a newer saved preference", async () => {
  const settings = new AgentSettings(
    new LocalClient(
      "",
      async () =>
        new Response(
          '{"revision":9007199254740992,"automatically_mention":false}',
          { headers: { "content-type": "application/json" } },
        ),
    ),
  );
  try {
    settings.value = {
      ...initial(),
      revision: 9007199254740993n,
      automatically_mention: true,
    };
    await settings.refresh();
    expect(settings.value.automatically_mention).toBe(true);
    expect(settings.value.revision).toBe(9007199254740993n);
  } finally {
    settings.dispose();
    settings.client.close();
  }
});
