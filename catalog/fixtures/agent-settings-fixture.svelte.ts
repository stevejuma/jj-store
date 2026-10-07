import type { AgentSettingsValue } from "../../web/src/lib/controller/agent-settings.svelte.js";
import { fixtureJsonResponse as json } from "./agent-execution-session.js";
export class AgentSettingsFixture {
  value = $state<AgentSettingsValue>({
    revision: 0,
    runner: "codex",
    preferences: { model: null, effort: null, mode: null },
    options: {
      inherit_defaults: false,
      provider: null,
      endpoint: null,
      environment: {},
    },
    automatically_mention: false,
    keep_awake: false,
    keychain_supported: false,
  });
  respond(path: string, init?: RequestInit) {
    if (!path.endsWith("/agents/settings")) return;
    if (!init?.body) return json(this.value);
    const body = JSON.parse(String(init.body));
    if (Number(body.expected_revision) !== Number(this.value.revision))
      return json({ code: "agent_selection_changed" }, 409);
    this.value = {
      ...body.settings,
      revision: Number(this.value.revision) + 1,
    };
    return json(this.value);
  }
}
