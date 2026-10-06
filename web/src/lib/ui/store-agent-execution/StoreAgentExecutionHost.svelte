<script lang="ts">
  // @community-copy CommunityAgents.svelte (Apache-2.0); local lifecycle directory.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSetup } from "../../controller/agent-execution-types.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreAgentLauncher from "./StoreAgentLauncher.svelte";
  import StoreAgentConversation from "./StoreAgentConversation.svelte";
  import StoreAgentSetupEditor from "./StoreAgentSetupEditor.svelte";
  import "./agent-execution.css";
  let {
    manager,
    onReview,
  }: {
    manager: AgentExecution;
    onReview?: (repository: string, commit: string, operation: string) => void;
  } = $props();
  let editorOpen = $state(false);
  let editing = $state<AgentSetup | undefined>();
</script>

<StoreNotice
  scope="agent-execution"
  noticeKey={String(manager.pending?.body.request_id ?? "runtime")}
  title="Agent execution"
  message={manager.error || manager.connectionError || manager.activityError}
  pending={manager.busy}
  action={{
    id: "refresh-agent-execution",
    label: manager.pending ? "Check outcome" : "Retry",
    run: () => void manager.refresh(),
  }}
/>
<Dialog.Root bind:open={manager.open}
  ><Dialog.Content class="agent-execution-dialog"
    ><Dialog.Header
      ><Dialog.Title>Agent sessions</Dialog.Title><Dialog.Description
        >Run agents in isolated local JJ workspaces and retain their activity
        for review.</Dialog.Description
      ></Dialog.Header
    >
    <div class="agent-execution-shell">
      <ScrollArea class="agent-execution-directory"
        ><nav aria-label="Local agent sessions">
          <Button variant="outline" onclick={() => (manager.launchOpen = true)}
            >New session</Button
          >
          <h3>Sessions</h3>
          {#each manager.value?.sessions ?? [] as session (session.id)}<Button
              class="agent-execution-session-row"
              variant={manager.selected === session.id && !manager.launchOpen
                ? "secondary"
                : "ghost"}
              onclick={() => {
                manager.launchOpen = false;
                void manager.select(session.id);
              }}
              ><span aria-hidden="true">{session.avatar}</span><span
                ><strong>{session.title}</strong><small
                  >{session.runner} · {session.state}</small
                ></span
              ></Button
            >{/each}
          <h3>Saved setups</h3>
          <Button
            variant="ghost"
            onclick={() => {
              editing = undefined;
              editorOpen = true;
            }}>Add saved setup</Button
          >
          {#each manager.value?.setups ?? [] as setup (setup.id)}<div
              class="agent-execution-setup-row"
            >
              <Button
                variant="ghost"
                onclick={() => {
                  editing = setup;
                  editorOpen = true;
                }}>{setup.avatar} {setup.name}</Button
              ><Button
                variant="ghost"
                size="sm"
                aria-label={`Remove ${setup.name} setup`}
                disabled={manager.busy || !!manager.pending}
                onclick={() => manager.saveSetup(undefined, setup.id)}
                >Remove</Button
              >
            </div>{/each}
        </nav></ScrollArea
      >
      <div class="agent-execution-content">
        {#if manager.launchOpen}<ScrollArea
            ><StoreAgentLauncher {manager} /></ScrollArea
          >{:else if manager.session}<StoreAgentConversation
            {manager}
            {onReview}
          />{:else}<p>Select a session or create one to get started.</p>{/if}
      </div>
    </div>
    {#if manager.pending}<p role="status">
        Request {String(manager.pending.body.request_id)} · {manager.outcome
          ?.state ?? "checking outcome"}. Draft and request identity retained.
      </p>{/if}
    {#if manager.pending && manager.outcome?.state === "not_known"}<Button
        variant="outline"
        disabled={manager.busy}
        onclick={() => manager.retryRetainedRequest()}
        >Submit retained request</Button
      >{/if}
    {#if manager.pending && manager.outcome?.state === "outcome_unknown"}<Button
        variant="outline"
        onclick={() => manager.retainAsUnresolved()}
        >Keep unresolved request and review workspace</Button
      >{/if}
  </Dialog.Content></Dialog.Root
>
{#if editorOpen}{#key editing?.id ?? "new"}<StoreAgentSetupEditor
      {manager}
      setup={editing}
      bind:open={editorOpen}
    />{/key}{/if}
