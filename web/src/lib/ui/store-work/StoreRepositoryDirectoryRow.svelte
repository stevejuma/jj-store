<script lang="ts">
  // CommunityProjects.svelte repositoryListRow (Apache-2.0).
  import {
    CommunityChip,
    CommunityMemberHoverCard,
    CommunityRelativeTime,
  } from "@lapismd/lapis-community/components";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import type { Snippet } from "svelte";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import type { RepositoryDirectoryEntry } from "../../controller/projects-repositories.svelte.js";
  import StoreRepositoryActivity from "./StoreRepositoryActivity.svelte";
  import {
    repositoryActivity,
    repositoryContributor,
  } from "./repository-directory-presentation.js";
  import "./community-work-directory.css";
  let {
    repository,
    entries = [],
    project,
    accessible = true,
    status = "",
    onOpen,
    actions,
  }: {
    repository: Pick<StoreRepositorySummary, "id" | "name" | "permission">;
    entries?: readonly RepositoryDirectoryEntry[];
    project?: { home: string; id: string };
    accessible?: boolean;
    status?: string;
    onOpen: (id: string) => void | Promise<void>;
    actions?: Snippet;
  } = $props();
  const entry = $derived(
    entries.find((entry) => entry.repository.id === repository.id),
  );
  const activity = $derived(
    repositoryActivity(entries, repository.id, project),
  );
</script>

<article
  class="community-projects__row community-projects__repository-row store-projects__repository-directory-row"
  aria-busy={entry?.loading}
>
  <span class="community-projects__row-icon"
    ><GitBranchIcon aria-hidden="true" /></span
  >
  <button
    type="button"
    class="community-projects__row-main community-projects__repository-target"
    disabled={!accessible}
    onclick={() => onOpen(repository.id)}
    aria-label={`Open repository ${repository.name}`}
  >
    <span
      ><strong title={repository.id}>{repository.name}</strong><span
        title={entry?.summary ? "Latest selected change" : undefined}
      >
        {entry?.summary?.description ||
          (accessible
            ? entry?.loading
              ? "Loading repository details…"
              : "No change description"
            : "Separate access required")}
      </span></span
    >
    <span class="store-projects__repository-access"
      >{accessible
        ? `${repository.permission ?? "Local"} access`
        : ""}{#if status}<CommunityChip size="compact">{status}</CommunityChip
        >{/if}</span
    >
  </button>
  {#if accessible}<div
      class="community-projects__row-meta store-projects__repository-metadata"
    >
      <CommunityMemberHoverCard
        member={repositoryContributor(entry?.summary?.contributor)}
        avatarSize="xsmall"
      />
      <StoreRepositoryActivity {...activity} />
      {#if activity.updatedAtMillis !== null}<CommunityRelativeTime
          timestamp={Number(activity.updatedAtMillis) / 1000}
        />{:else}<span
          class="store-projects__repository-date"
          title="No recorded activity date">—</span
        >{/if}
      {#if actions}{@render actions()}{/if}
    </div>{:else if actions}{@render actions()}{/if}
  {#if entry?.error}<span class="store-projects__repository-error" role="alert"
      >{entry.error}</span
    >{/if}
</article>
