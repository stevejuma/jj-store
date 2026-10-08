<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreAgentSettings from "../../web/src/lib/ui/store-settings/StoreAgentSettings.svelte";
  import {
    AgentRuntimes,
    type RuntimeStatus,
    type RuntimeCommand,
    type RuntimeOutcome,
  } from "../../web/src/lib/controller/agent-runtimes.svelte.js";
  let {
    scenario = "install",
    lost = false,
    prerequisites = false,
    loading = false,
  }: {
    scenario?: "install" | "update" | "ready";
    lost?: boolean;
    prerequisites?: boolean;
    loading?: boolean;
  } = $props();
  let commands = $state<RuntimeCommand[]>([]);
  let failed = false;
  let unresolved = untrack(() => lost);
  let release!: () => void;
  const gate = untrack(() => loading)
    ? new Promise<void>((r) => {
        release = r;
      })
    : Promise.resolve();
  let outcome: RuntimeOutcome | null = null;
  const value: RuntimeStatus = untrack(() => ({
    version: 1,
    node_version: prerequisites ? null : "24.18.0",
    npm_available: !prerequisites,
    install_supported: !prerequisites,
    install_unavailable: prerequisites
      ? "Install Node.js 22 or newer with npm, then Check again."
      : null,
    pending: null,
    runtimes: [
      {
        id: "codex",
        label: "Codex",
        state:
          scenario === "install"
            ? "adapter_needed"
            : scenario === "update"
              ? "update_available"
              : "ready",
        adapter_version:
          scenario === "install"
            ? null
            : scenario === "update"
              ? "1.4.0"
              : "1.13.1",
        cli_version: "0.156.1",
        managed: scenario === "ready",
        action: scenario === "ready" ? null : scenario,
        detail:
          "Codex CLI and ACP adapter. Authenticate with Codex before connecting.",
        guide_url: "https://developers.openai.com/codex/cli",
        target_version: "1.13.1",
        target_cli_version: "0.156.1",
      },
      {
        id: "cursor",
        label: "Cursor",
        state: "ready",
        adapter_version: null,
        cli_version: null,
        managed: false,
        action: null,
        detail:
          "Cursor supplies its own ACP runtime. Install or update using Cursor's guide, then run cursor-agent login.",
        guide_url: "https://cursor.com/docs/cli/installation",
        target_version: null,
        target_cli_version: null,
      },
      {
        id: "local",
        label: "JJ Agent",
        state: "ready",
        adapter_version: null,
        cli_version: null,
        managed: true,
        action: null,
        detail: "Bundled ACP agent and coding tools, adapted from buzz-agent.",
        guide_url: "/help/guides/agent-execution/",
        target_version: null,
        target_cli_version: null,
      },
    ],
  }));
  const management = new AgentRuntimes(() => true, undefined, {
    close() {},
    request: (async (path: string, body?: RuntimeCommand) => {
      await gate;
      if (body) {
        commands = [...commands, structuredClone(body)];
        outcome = {
          version: 1,
          request_id: body.request_id,
          runtime_id: body.runtime_id,
          state: "installing",
          error: null,
        };
        if (lost) throw new Error("response lost");
        return structuredClone(outcome);
      }
      if (path.includes("outcome")) {
        if (!unresolved) {
          outcome = { ...outcome!, state: "completed" };
          value.runtimes[0] = {
            ...value.runtimes[0],
            state: "ready",
            managed: true,
            action: null,
            adapter_version: "1.13.1",
          };
        }
        return structuredClone(outcome);
      }
      if (failed) throw new Error("Runtime checks are offline");
      return structuredClone(value);
    }) as never,
  });
  onDestroy(() => management.destroy());
</script>

<StoreAgentSettings runtimes={management} />
<div class="fixture-controls" aria-label="Fixture controls">
  <Button
    variant="outline"
    onclick={() => {
      failed = true;
      void management.refresh();
    }}>Fail runtime checks</Button
  >
  <Button
    variant="outline"
    onclick={() => {
      failed = false;
      void management.refresh();
    }}>Recover runtime checks</Button
  >
  {#if lost}<Button
      variant="outline"
      onclick={() => {
        unresolved = false;
        void management.refresh();
      }}>Complete original installation</Button
    >{/if}
  {#if loading}<Button variant="outline" onclick={() => release()}
      >Finish loading</Button
    >{/if}
  <output aria-label="Installation command count">{commands.length}</output>
  <output aria-label="Installation command actions"
    >{commands.map((c) => `${c.runtime_id}:${c.action}`).join(",")}</output
  >
</div>

<style>
  .fixture-controls {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 1rem;
  }
</style>
