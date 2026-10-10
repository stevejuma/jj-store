<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import * as Popover from "@lapismd/design-core/shadcn/popover";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    controller,
    rail = false,
  }: { controller: StoreController; rail?: boolean } = $props();
  const detail = $derived(controller.threads?.detail?.thread);
  const repositories = $derived(
    controller.state.repositories.filter(
      (r) =>
        !!r.permission &&
        !["unavailable", "identity_changed"].includes(r.availability ?? ""),
    ),
  );
  const members = $derived(
    detail?.value.codebase ??
      (detail ? [controller.state.route.repository] : []),
  );
  let open = $state(false),
    selected = $state<string[]>([]);
  function opening(value: boolean) {
    open = value;
    if (value) selected = [...members];
  }
  async function save() {
    if (!detail) return;
    const result = await controller.threadWorkspaces
      ?.submit({
        type: "codebase",
        home: controller.state.route.repository,
        thread: detail.value.thread_id,
        expected: detail.reference,
        repositories: selected,
      })
      .catch(() => undefined);
    if (result) open = false;
  }
</script>

{#if rail}
  <section class="thread-codebase" aria-label="Codebase">
    <h2>Codebase</h2>
    {#each members as id (id)}
      <a
        href={`#/repos/${id}/files`}
        title={repositories.find((r) => r.id === id)?.name ?? id}
        ><FolderGitIcon size={16} aria-hidden="true" /><span class="truncate"
          >{repositories.find((r) => r.id === id)?.name ?? id}</span
        ></a
      >
    {:else}<p>No repositories attached.</p>{/each}
  </section>
{:else if detail && controller.threadWorkspaces}
  <Popover.Root {open} onOpenChange={opening}>
    <Popover.Trigger
      >{#snippet child({ props })}<Button
          {...props}
          variant="secondary"
          size="xs"
          class="conversation-context-trigger"
          aria-label="Thread repositories"
          ><FolderGitIcon
            data-icon="inline-start"
            aria-hidden="true"
          />{members.length
            ? `Codebase · ${members.length}`
            : "Add repositories"}</Button
        >{/snippet}</Popover.Trigger
    >
    <Popover.Content
      side="top"
      align="start"
      style="--ui-popover-width: min(36rem, calc(100vw - 2rem))"
    >
      <h2>Thread Codebase</h2>
      <p>Choose up to eight repositories. Agents use separate workspaces.</p>
      <StoreRepositoryPicker
        {repositories}
        bind:values={selected}
        multiple
        inline
        label="Thread Codebase"
        disabled={controller.threadWorkspaces.busy ||
          !!controller.threadWorkspaces.pending}
      />
      <StoreNotice
        noticeKey="thread-codebase"
        title="Codebase action needs attention"
        message={controller.threadWorkspaces.error}
      />
      {#if controller.threadWorkspaces.pending}<Button
          disabled={controller.threadWorkspaces.busy}
          onclick={() =>
            void controller.threadWorkspaces?.retry().catch(() => {})}
          >Check same action</Button
        >{:else}<Button
          disabled={controller.threadWorkspaces.busy}
          onclick={() => void save()}>Apply</Button
        >{/if}
    </Popover.Content>
  </Popover.Root>
{/if}

<style>
  .thread-codebase {
    padding: 0.75rem;
    min-width: 0;
  }
  h2 {
    font-size: var(--community-text-body);
    font-weight: 600;
    margin: 0 0 0.5rem;
  }
  p {
    font-size: var(--community-text-secondary);
    color: var(--muted-foreground);
  }
  .thread-codebase a {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    padding: 0.375rem;
  }
  .thread-codebase a:hover {
    background: var(--muted);
  }
</style>
