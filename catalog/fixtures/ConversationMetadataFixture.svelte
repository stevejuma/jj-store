<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { AgentExecution } from "../../web/src/lib/controller/agent-execution.svelte.js";
  import type { MetadataView } from "../../web/src/lib/controller/conversation-metadata.svelte.js";
  import StoreConversationMetadata from "../../web/src/lib/ui/store-agent-execution/StoreConversationMetadata.svelte";
  import StoreConversationMetadataSettings from "../../web/src/lib/ui/store-settings/StoreConversationMetadataSettings.svelte";
  import { AgentSettingsFixture } from "./agent-settings-fixture.svelte.js";
  import { fixtureJsonResponse as json } from "./agent-execution-session.js";
  let {
    lost = false,
    settingsMode = false,
    interrupted = false,
  }: {
    lost?: boolean;
    settingsMode?: boolean;
    interrupted?: boolean;
  } = $props();
  let mounted = $state(true),
    posts = $state<Record<string, unknown>[]>([]),
    lostOnce = false;
  const settings = new AgentSettingsFixture();
  settings.value.metadata = {
    model: "fixture-model",
    options: {
      provider: "openai-compat",
      endpoint: "https://fixture.invalid/v1",
      environment: {},
      inherit_defaults: false,
    },
  };
  let view = $state<MetadataView>({
    version: 1,
    repository: "11".repeat(16),
    target: { type: "conversation", id: "44".repeat(16) },
    thread: "55".repeat(16),
    revision: null,
    value: { custom_title: null, generated: null, agent_titles: {} },
    sources: [],
    title: "Review initial request",
    fresh: false,
    writable: true,
    unavailable: null,
    details: [],
    job: null,
  });
  const query = { session: "44".repeat(16) };
  if (untrack(() => interrupted))
    view.job = {
      request_id: "bb".repeat(16),
      state: "outcome_unknown",
      error: "interrupted_metadata_request_not_replayed",
      result: null,
      original_command: {
        version: 1,
        request_id: "bb".repeat(16),
        query: { ...query, repository: null },
        expected: null,
        action: { action: "refresh" },
      },
    };
  const manager: AgentExecution = new AgentExecution(
    () => true,
    async (input, init): Promise<Response> => {
      const path = String(input);
      const response = settings.respond(path, init);
      if (response) return response;
      if (path.includes("/metadata/job")) return json(view.job);
      if (path.includes("/metadata")) {
        if (!init?.body) return json(view);
        const command = JSON.parse(String(init.body));
        posts.push(command);
        if (view.job?.request_id !== command.request_id) {
          if (command.action.action === "rename")
            view.value.custom_title = command.action.title;
          if (["refresh", "regenerate_title"].includes(command.action.action)) {
            view.value.generated = {
              title: "Review generated metadata",
              summary: "Fixture generated summary.",
              keywords: ["discovery-keyword"],
              goals: ["Review metadata"],
              sources: [],
              provider: "openai-compat",
              model: "fixture-model",
              prompt_version: 1,
              generated_at_millis: 1_700_000_000_000,
              truncated: false,
            };
            if (command.action.action === "regenerate_title")
              view.value.custom_title = null;
          }
          view.title =
            view.value.custom_title ??
            view.value.generated?.title ??
            "Review initial request";
          view.fresh = true;
          view.job = {
            request_id: command.request_id,
            state: "completed",
            error: null,
            result: null,
            original_command: command,
          };
          view.revision = {
            kind: "conversation_metadata",
            id: "a1".repeat(32),
          };
        }
        if (lost && !lostOnce) {
          lostOnce = true;
          throw Error("Simulated lost response; original request retained");
        }
        return json(view.job);
      }
      if (path.endsWith("/catalog"))
        return json({
          runner: "local",
          status: "ready",
          facts: {
            models: {
              availableModels: [
                { modelId: "fixture-model", name: "Metadata model" },
              ],
            },
          },
        });
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
        providers: [
          {
            id: "openai-compat",
            label: "OpenAI-compatible",
            runtime_provider: "openai-compat",
            endpoint_env: "OPENAI_COMPAT_BASE_URL",
            default_endpoint: null,
            endpoint_required: true,
            credential_env: null,
            model_required: true,
            managed: false,
            guidance: "Fixture provider",
          },
        ],
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

<Button variant="ghost" onclick={() => (mounted = !mounted)}
  >Toggle details mount</Button
>
{#if mounted}{#if settingsMode}<StoreConversationMetadataSettings
      {manager}
    />{:else}<StoreConversationMetadata {manager} {query} />{/if}{/if}
<output aria-label="Metadata commands">{JSON.stringify(posts)}</output>
<output aria-label="Saved metadata configuration"
  >{JSON.stringify(settings.value.metadata)}</output
>
