<script lang="ts">
  import type { Snippet } from "svelte";
  import type { Model } from "../../api/client.js";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import ActivityIcon from "@lucide/svelte/icons/activity";
  let {
    binding,
    repository,
    children,
  }: {
    binding: Model<"LocalBinding">;
    repository: boolean;
    children: Snippet;
  } = $props();
</script>

{#if repository}
  <dl class="store-settings__rows">
    <StoreReadOnlyRow label="Name" value={binding.name} />
    <StoreReadOnlyRow
      label="Folder path"
      value={binding.path}
      Icon={FolderGitIcon}
    />
    <StoreReadOnlyRow
      label="Availability"
      value={binding.availability.replaceAll("_", " ")}
      Icon={ActivityIcon}
    />
  </dl>
  <StoreSettingsFooter>{@render children()}</StoreSettingsFooter>
{:else}
  <div>
    <dt>
      <strong>{binding.name}</strong>
      <p class="store-settings__row-description">{binding.path}</p>
      <p class="store-settings__row-description">
        {binding.availability.replaceAll("_", " ")}
      </p>
    </dt>
    <dd class="store-settings__actions">{@render children()}</dd>
  </div>
{/if}
