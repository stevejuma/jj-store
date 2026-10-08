<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { LocalClient } from "../../web/src/lib/api/local-client.js";
  import { AgentCompute } from "../../web/src/lib/controller/agent-compute.svelte.js";
  import type { ComputeValue } from "../../web/src/lib/controller/agent-compute.svelte.js";
  import StoreComputeSettings from "../../web/src/lib/ui/store-settings/StoreComputeSettings.svelte";
  import StoreAgentSentContext from "../../web/src/lib/ui/store-agent-execution/StoreAgentSentContext.svelte";
  import { stringify } from "lossless-json";
  let {
    context = false,
    paired = false,
    picker = "selected",
    memoryBytes = 17179869184,
    memoryLimit = 4,
    selectedModel = "fixture-gguf",
    installedModel = false,
    serving = false,
  }: {
    context?: boolean;
    paired?: boolean;
    picker?: "selected" | "cancelled" | "error";
    memoryBytes?: number | null;
    memoryLimit?: number | null;
    selectedModel?: string;
    installedModel?: boolean;
    serving?: boolean;
  } = $props();
  let commands = $state<Record<string, unknown>[]>([]);
  let status: ComputeValue = untrack(() => ({
    hardware: {
      os: "macOS",
      architecture: "arm64",
      cpu_threads: 8,
      memory_bytes: memoryBytes,
    },
    supported: true,
    model_file_picker: true,
    state: serving ? "serving" : "off",
    runtime_prepared: false,
    settings: {
      revision: 0,
      model: selectedModel,
      memory_gb: memoryLimit,
      sharing: false,
      peers: paired
        ? {
            ["ab".repeat(32)]: {
              name: "Trusted inference machine",
              invitation: { id: "ab".repeat(32), direct_addresses: [] },
            },
          }
        : {},
      relay_urls: [],
      target_peer: null,
    },
    models: [
      { id: "fixture-gguf", name: "Fixture GGUF", installed: false },
      {
        id: "unsloth/gemma-4-E4B-it-GGUF:Q4_K_M",
        name: "Gemma 4 E4B",
        installed: installedModel,
      },
    ],
    identity: paired ? "cd".repeat(32) : null,
    invitation: paired
      ? { id: "cd".repeat(32), direct_addresses: ["127.0.0.1:32100"] }
      : null,
    progress: null,
    error: null,
    operation: null,
  }));
  const client = untrack(
    () =>
      new LocalClient("", async (_input, init) => {
        if (String(_input).endsWith("/compute/pick")) {
          if (picker === "error")
            return Response.json(
              { code: "model_file_picker_unavailable" },
              { status: 503 },
            );
          return Response.json({
            version: 1,
            path: picker === "cancelled" ? null : "/models/My model.gguf",
          });
        }
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
