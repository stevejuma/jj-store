<script lang="ts">
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import { onMount, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import FilesIcon from "@lucide/svelte/icons/files";
  import CopyIcon from "@lucide/svelte/icons/copy";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import { useConversationInspection } from "../../conversations/inspection.svelte.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  import type { ConversationSession } from "../../conversations/session.svelte.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    manager,
    sessions,
    conversation,
    controller,
  }: {
    manager: AgentExecution;
    sessions: readonly AgentSession[];
    conversation: ConversationSession;
    controller?: StoreController;
  } = $props();
  const { context: notifications } = ownNotifications();
  const inspection = useConversationInspection();
  const browser = untrack(() =>
    inspection?.workspaceBrowser(manager.client, conversation),
  );
  let open = $state(false),
    copied = $state("");
  const label = $derived(
    browser?.binding
      ? `${browser.binding.repositoryName} / ${browser.binding.workspace}`
      : "Workspace",
  );
  const branches = $derived(
    browser?.details?.references.filter(
      (r) => r.commit === browser?.details?.commit,
    ) ?? [],
  );
  const branch = $derived(branches.length === 1 ? branches[0].name : null);
  $effect(() => {
    const value = sessions,
      names = new Map(
        controller?.state.repositories.map((r) => [r.id, r.name]) ?? [],
      );
    untrack(() => browser?.setBindings(value, names));
  });
  $effect(() => {
    if (open && browser?.binding) untrack(() => void browser?.load());
  });
  onMount(() => {
    const timer = window.setInterval(() => {
      if (browser?.railOpen || open) void browser?.load();
    }, 15000);
    return () => {
      clearInterval(timer);
    };
  });
  async function copy(value: string, kind: string) {
    try {
      await navigator.clipboard.writeText(value);
      copied = `${kind} copied`;
    } catch {
      notifications.service.publish({
        scope: notifications.scope,
        key: `workspace-copy:${kind}`,
        title: "Copy failed",
        message: `Could not copy ${kind.toLowerCase()}.`,
        severity: "error",
        lifetime: "timed",
      });
    }
  }
</script>

{#if browser?.binding && inspection?.workspaceEnabled}
  <Popover.Root bind:open>
    <Popover.Trigger
      >{#snippet child({ props })}<Button
          {...props}
          variant="secondary"
          size="sm"
          class="workspace-trigger"
          title={label}
          aria-label={`Workspace: ${label}`}
          ><FolderIcon /><span class="truncate">{label}</span></Button
        >{/snippet}</Popover.Trigger
    >
    <Popover.Content
      align="start"
      side="top"
      class="w-80 max-w-[calc(100vw-2rem)]"
    >
      <div class="workspace-popover">
        <strong>Workspace</strong>
        {#if browser.bindings.length > 1}
          <Select.Root
            type="single"
            value={browser.bindingKey}
            onValueChange={(value) => void browser.selectBinding(value)}
          >
            <Select.Trigger aria-label="Inspect workspace"
              ><span class="truncate"
                >{browser.binding.repositoryName} / {browser.binding.workspace} ·
                {browser.binding.agent}</span
              ></Select.Trigger
            >
            <Select.Content
              ><Select.Group
                >{#each browser.bindings as b (browser.key(b))}<Select.Item
                    value={browser.key(b)}
                    >{b.repositoryName} / {b.workspace} · {b.agent}</Select.Item
                  >{/each}</Select.Group
              ></Select.Content
            >
          </Select.Root>
        {/if}
        {#if browser.details && browser.details.repository === browser.binding.repository && browser.details.session === browser.binding.session}
          <p class="workspace-location">{browser.details.location}</p>
          {#if branch}<p><GitBranchIcon size={14} />{branch}</p>{/if}
          <Button
            variant="ghost"
            size="sm"
            onclick={() => {
              open = false;
              inspection?.showWorkspace(browser);
            }}><FilesIcon />View files</Button
          >
          <Button
            variant="ghost"
            size="sm"
            onclick={() =>
              void copy(browser.details!.location, "Workspace path")}
            ><CopyIcon />Copy workspace path</Button
          >
          {#if branch}<Button
              variant="ghost"
              size="sm"
              onclick={() => void copy(branch, "Branch name")}
              ><CopyIcon />Copy branch name</Button
            >{/if}
        {:else}<p role="status">
            {browser.listing.error || "Loading workspace…"}
          </p>{/if}
        {#if copied}<span role="status">{copied}</span>{/if}
      </div>
    </Popover.Content>
  </Popover.Root>
{/if}

<style>
  :global(.workspace-trigger) {
    min-width: 0;
    max-width: min(18rem, 100%);
  }
  .workspace-popover {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    min-width: 0;
  }
  .workspace-location {
    overflow-wrap: anywhere;
    color: var(--muted-foreground);
  }
  p {
    margin: 0;
    font-size: 0.8125rem;
    display: flex;
    gap: 0.375rem;
    align-items: center;
  }
</style>
