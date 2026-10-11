<script lang="ts">
  import type { StoreController } from "../../controller/contracts.js";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import ServerIcon from "@lucide/svelte/icons/server";
  let {
    controller,
    onOpen,
    navigationBlocked,
  }: {
    controller: StoreController;
    onOpen: (operation?: string) => Promise<void>;
    navigationBlocked: boolean;
  } = $props();
  const view = $derived(controller.state);
  const snapshot = $derived(view.settings.snapshot);
  const native = $derived(snapshot && "epoch" in snapshot ? snapshot : null);
  const local = $derived(snapshot && "profile" in snapshot ? snapshot : null);
  const isLocal = $derived(view.repository.profile === "git_sha1_jj045");
  const number = (value: number | bigint | null | undefined) =>
    value === null || value === undefined
      ? "Not available"
      : value.toLocaleString("en-GB");
</script>

<section class="store-settings__section" aria-labelledby="repository-identity">
  <header class="store-repository-identity__header">
    <h2 id="repository-identity">Identity and snapshot</h2>
    <CommunityChip
      variant="subtle"
      tone={isLocal ? "success" : "accent"}
      size="compact"
      title={isLocal
        ? "Folder-backed JJ repository"
        : "Native repository managed by jj-store"}
    >
      {#snippet leading()}{#if isLocal}<FolderGitIcon
            aria-hidden="true"
          />{:else}<ServerIcon aria-hidden="true" />{/if}{/snippet}
      {isLocal ? "Local JJ" : "Server Hosted"}
    </CommunityChip>
  </header>
  <dl class="store-settings__rows">
    <StoreReadOnlyRow label="Name" value={view.repository.name} />
    <StoreReadOnlyRow
      copyable
      label="Repository ID"
      copyLabel="Repository ID"
      value={view.repository.id}
      onOpen={controller.actions.openRoute ? () => void onOpen() : undefined}
      openLabel="Open repository"
      openDisabled={!!navigationBlocked}
    />
    <StoreReadOnlyRow label="Permission" value={view.repository.permission} />
    <StoreReadOnlyRow
      copyable
      label="Selected operation"
      copyLabel="Selected operation"
      value={view.route.snapshotOperationId}
      onOpen={controller.actions.openRoute && view.route.snapshotOperationId
        ? () => void onOpen(view.route.snapshotOperationId!)
        : undefined}
      openLabel="Open selected operation"
      openDisabled={!!navigationBlocked}
      emptyLabel="Choose an operation head"
    />
    {#each view.operationHeads as head, index (head)}
      <StoreReadOnlyRow
        copyable
        label={`Operation head ${index + 1}`}
        value={head}
        onOpen={controller.actions.openRoute
          ? () => void onOpen(head)
          : undefined}
        openLabel={`Open operation head ${index + 1}`}
        openDisabled={!!navigationBlocked}
      />
    {/each}
    <StoreReadOnlyRow
      label="Conflicts"
      value={view.conflictCount}
      emptyLabel="Not loaded"
    />
    {#if local}<StoreReadOnlyRow
        label="Profile"
        value="Git-backed JJ (SHA-1)"
      />
      <StoreReadOnlyRow
        label="Folder generation"
        value={local.generation}
        display={number(local.generation)}
      />
      <StoreReadOnlyRow
        copyable
        label="Companion metadata"
        copyLabel="Companion metadata"
        value={local.metadata_commit}
      />{/if}
    {#if native}<StoreReadOnlyRow
        copyable
        label="Epoch"
        copyLabel="Epoch"
        value={native?.epoch}
        emptyLabel="Unavailable"
      />
      <StoreReadOnlyRow
        label="Published position"
        value={native?.published_seq}
        display={number(native?.published_seq)}
      />
      <StoreReadOnlyRow
        label="Manifest revision"
        value={native?.manifest_revision}
        display={number(native?.manifest_revision)}
      />
      <StoreReadOnlyRow
        label="Code state version"
        value={native?.code_state_version}
        display={number(native?.code_state_version)}
      />{/if}
  </dl>
</section>

<style>
  .store-repository-identity__header {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.6rem;
  }
</style>
