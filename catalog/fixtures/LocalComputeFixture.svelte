<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { LocalClient } from "../../web/src/lib/api/local-client.js";
  import { AgentCompute } from "../../web/src/lib/controller/agent-compute.svelte.js";
  import type { ComputeValue } from "../../web/src/lib/controller/agent-compute.svelte.js";
  import StoreComputeSettings from "../../web/src/lib/ui/store-settings/StoreComputeSettings.svelte";
  import StoreAgentSentContext from "../../web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte";
  import { stringify } from "lossless-json";
  let { context = false }: { context?: boolean } = $props();
  let commands = $state<Record<string, unknown>[]>([]);
  let status: ComputeValue = {
    hardware: {
      os: "macOS",
      architecture: "arm64",
      cpu_threads: 8,
      memory_bytes: 17179869184,
    },
    supported: true,
    state: "off",
    runtime_prepared: false,
    settings: {
      revision: 0,
      model: "fixture-gguf",
      memory_gb: 4,
      sharing: false,
      peers: {},
      relay_urls: [],
      target_peer: null,
    },
    models: [{ id: "fixture-gguf", name: "Fixture GGUF", installed: false }],
    identity: null,
    invitation: null,
    progress: null,
    error: null,
    operation: null,
  };
  const client = untrack(
    () =>
      new LocalClient("", async (_input, init) => {
        if (init?.body) {
          const body = JSON.parse(String(init.body));
          commands = [...commands, body];
          const action = body.action;
          status.settings.revision = Number(status.settings.revision) + 1;
          if (action.action === "configure")
            status.settings = {
              ...action.settings,
              revision: status.settings.revision,
            };
          if (action.action === "prepare") status.runtime_prepared = true;
          if (action.action === "download") status.models[0].installed = true;
          if (action.action === "start") status.state = "serving";
          if (action.action === "stop") status.state = "off";
          return Response.json({
            version: 1,
            request_id: body.request_id,
            session_id: null,
            state: "completed",
            error: null,
          });
        }
        return Response.json(status);
      }),
  );
  const compute = untrack(() => new AgentCompute(client));
  onDestroy(() => {
    compute.stop();
    client.close();
  });
  const captured = {
    version: 1,
    delivery: "sent",
    blocks: [
      {
        title: "Message origin",
        data: {
          origin: {
            scope: "file",
            context: stringify({
              repository: "captured-repository",
              path: "src/example.rs",
              revision: "captured-revision",
              operation: "captured-operation",
              file: { content: "fn captured_file() {}", truncated: false },
              task_content: {
                title: "Captured task",
                body: "**Historical** task body",
                version: 9007199254740993n,
              },
            }),
          },
        },
      },
    ],
  };
</script>

{#if context}
  <StoreAgentSentContext value={captured} />
{:else}
  <StoreComputeSettings management={compute} />
  <output aria-label="Compute commands">{JSON.stringify(commands)}</output>
{/if}
