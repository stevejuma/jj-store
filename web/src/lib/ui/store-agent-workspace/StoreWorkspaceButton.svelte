<script lang="ts">
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import { onMount, untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
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
  <Menu.Root bind:open>
    <Menu.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          variant="ghost"
          size="xs"
          class="workspace-trigger"
          title={label}
          aria-label={`Workspace: ${label}`}
          ><FolderGitIcon data-icon="inline-start" aria-hidden="true" /><span
            class="truncate">{label}</span
          ></Button
        >
      {/snippet}
    </Menu.Trigger>
    <Menu.Content
      align="start"
      side="top"
      aria-label="Workspace"
      class="w-64 max-w-[calc(100vw-2rem)]"
    >
      <Menu.Group>
        <Menu.Label>Workspace</Menu.Label>
        {#if browser.bindings.length > 1}
          <Menu.Sub>
            <Menu.SubTrigger
              ><FolderGitIcon aria-hidden="true" />Inspect workspace</Menu.SubTrigger
            >
            <Menu.SubContent
              aria-label="Inspect workspace"
              class="max-w-[calc(100vw-2rem)]"
            >
              <Menu.RadioGroup
                value={browser.bindingKey}
                onValueChange={(value) => void browser.selectBinding(value)}
              >
                {#each browser.bindings as b (browser.key(b))}
                  <Menu.RadioItem value={browser.key(b)}>
                    <span
                      class="truncate"
                      title={`${b.repositoryName} / ${b.workspace} · ${b.agent}`}
                    >
                      {b.repositoryName} / {b.workspace} · {b.agent}
                    </span>
                  </Menu.RadioItem>
                {/each}
              </Menu.RadioGroup>
            </Menu.SubContent>
          </Menu.Sub>
        {/if}
        {#if browser.details && browser.details.repository === browser.binding.repository && browser.details.session === browser.binding.session}
          {#if branch}<Menu.Label
              ><span class="workspace-branch"
                ><GitBranchIcon size={14} aria-hidden="true" /><span
                  class="truncate"
                  title={branch}>{branch}</span
                ></span
              ></Menu.Label
            >{/if}
          <Menu.Item onSelect={() => inspection?.showWorkspace(browser)}>
            <FilesIcon aria-hidden="true" />View files
          </Menu.Item>
          <Menu.Sub>
            <Menu.SubTrigger
              ><CopyIcon aria-hidden="true" />Copy</Menu.SubTrigger
            >
            <Menu.SubContent aria-label="Copy" class="w-48">
              <Menu.Group>
                <Menu.Item
                  onSelect={() =>
                    void copy(browser.details!.location, "Project path")}
                >
                  <FolderGitIcon aria-hidden="true" />Project path
                </Menu.Item>
                <Menu.Item
                  onSelect={() =>
                    void copy(browser.details!.workspace, "Workspace name")}
                >
                  <CopyIcon aria-hidden="true" />Workspace name
                </Menu.Item>
              </Menu.Group>
            </Menu.SubContent>
          </Menu.Sub>
        {:else}
          <Menu.Label
            ><span role="status"
              >{browser.listing.error || "Loading workspace…"}</span
            ></Menu.Label
          >
        {/if}
      </Menu.Group>
    </Menu.Content>
  </Menu.Root>
  <span class="sr-only" role="status" aria-live="polite">{copied}</span>
{/if}

<style>
  :global(.workspace-trigger) {
    min-width: 0;
    max-width: min(18rem, 100%);
  }
  .workspace-branch {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    min-width: 0;
  }
</style>
