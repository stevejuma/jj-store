<script lang="ts">
  import { onDestroy } from "svelte";
  import StoreAgentSetupEditor from "../../web/src/lib/ui/store-agent-execution/StoreAgentSetupEditor.svelte";
  import StoreAgentDefaults from "../../web/src/lib/ui/store-settings/StoreAgentDefaults.svelte";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
  import { fixtureJsonResponse as json } from "./agent-execution-session.js";
  let {
    defaults = false,
    failure = false,
  }: { defaults?: boolean; failure?: boolean } = $props();
  let open = $state(true);
  let requests = $state<Record<string, unknown>[]>([]);
  let saved = $state("");
  const settings = new AgentSettingsFixture();
  settings.value.runner = "local";
  settings.value.keychain_supported = true;
  const providers = [
    ["local", "This machine"],
    ["paired", "JJ Store shared compute"],
    ["anthropic", "Anthropic"],
    ["openai-api", "OpenAI"],
    ["openai-compat", "OpenAI-compatible"],
    ["openrouter", "OpenRouter"],
    ["databricks", "Databricks"],
    ["databricks_v2", "Databricks v2"],
  ].map(([id, label]) => ({
    id,
    label,
    runtime_provider: id,
    endpoint_env: id.startsWith("databricks")
      ? "DATABRICKS_HOST"
      : "OPENAI_COMPAT_BASE_URL",
    default_endpoint: null,
    endpoint_required: false,
    credential_env: id === "anthropic" ? "ANTHROPIC_API_KEY" : null,
    model_required: !["local", "paired"].includes(id),
    managed: ["local", "paired"].includes(id),
    guidance: "Configure provider credentials in Advanced.",
  }));
  const manager: AgentExecution = new AgentExecution(
    () => true,
    async (input, init): Promise<Response> => {
      const path = String(input),
        body = init?.body ? JSON.parse(String(init.body)) : {};
      const response = settings.respond(path, init);
      if (response) return response;
      if (path.endsWith("/catalog")) {
        requests.push({
          path,
          provider: body.options?.provider,
          secrets: Object.keys(body.secrets ?? {}),
        });
        const provider = body.options?.provider ?? "local";
        return json({
          runner: body.runner,
          status: failure ? "authentication_required" : "ready",
          facts: failure
            ? {}
            : {
                models: {
                  availableModels: [
                    {
                      modelId: `${provider}-model`,
                      name: `${provider} available model`,
                    },
                  ],
                },
                modes: {
                  availableModes: [{ id: "ask", name: "Ask before tools" }],
                },
              },
        });
      }
      if (path.endsWith("/compute"))
        return json({
          supported: true,
          model_file_picker: false,
          state: "serving",
          settings: {
            revision: 1,
            model: "local-model",
            memory_gb: null,
            sharing: false,
            peers: { trusted: { name: "Trusted host", invitation: {} } },
            relay_urls: [],
            target_peer: "trusted",
          },
          models: [],
          runtime_prepared: true,
          hardware: {},
          invitation: null,
          identity: null,
          progress: null,
          error: null,
          operation: null,
        });
      if (path.endsWith("/setups")) {
        saved = JSON.stringify(body.setup);
        return json({ ...manager.value, revision: 1, setups: [body.setup] });
      }
      if (path.endsWith("/agents")) return json(manager.value);
      return json({ code: "fixture_unavailable" }, 503);
    },
  );
  manager.value = {
    version: 1,
    revision: 0,
    runtime: "ready",
    runners: [
      {
        id: "local",
        label: "JJ Agent",
        available: true,
        qualified: false,
        install_hint: null,
        providers,
      },
      {
        id: "codex",
        label: "Codex",
        available: true,
        qualified: false,
        install_hint: null,
        providers: [],
      },
    ],
    setups: [],
    sessions: [],
    conversations: [],
    live_agents: [],
  };
  manager.settings.value = settings.value;
  onDestroy(() => manager.destroy());
</script>

{#if defaults}<StoreAgentDefaults {manager} />{:else}<StoreAgentSetupEditor
    {manager}
    bind:open
  />{/if}
<output aria-label="Catalog preview requests">{JSON.stringify(requests)}</output
>
<output aria-label="Saved agent configuration">{saved}</output>
