<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { RepositoryBlob } from "../../controller/contracts.js";
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import CommunitySidebarToggleIcon from "../community-app-shell/CommunitySidebarToggleIcon.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import CommunityProjectsGitAuthorAvatar from "./CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "./CommunityProjectsRevisionGraphIds.svelte";

  let {
    blob,
    latestCommit,
    now,
    fileTreeOpen,
    onToggleFileTree,
  }: {
    blob: RepositoryBlob;
    latestCommit?: RepositoryLogRevision;
    now?: number;
    fileTreeOpen: boolean;
    onToggleFileTree: () => void;
  } = $props();
</script>

<section
  class="community-projects__file-panel-info"
  aria-label="Selected file metadata"
>
  <Button
    type="button"
    variant="ghost"
    size="icon-sm"
    class="community-projects__file-panel-tree-toggle"
    aria-label={fileTreeOpen
      ? "Hide repository file tree"
      : "Show repository file tree"}
    title={fileTreeOpen
      ? "Hide repository file tree"
      : "Show repository file tree"}
    onclick={onToggleFileTree}
  >
    <CommunitySidebarToggleIcon side="left" expanded={fileTreeOpen} />
  </Button>
  {#if latestCommit}
    <CommunityProjectsGitAuthorAvatar
      email={latestCommit.authorEmail}
      name={latestCommit.author}
      size="small"
    />
    <span class="community-projects__file-panel-info-message">
      <strong>{latestCommit.author}</strong> committed
      <span>{latestCommit.message}</span>
    </span>
    <div class="community-projects__file-panel-info-actions">
      <CommunityProjectsRevisionGraphIds
        commitId={latestCommit.eventId}
        changeId={latestCommit.changeId}
        placement="inline"
      />
      {#if latestCommit.createdAt}
        <CommunityRelativeTime timestamp={latestCommit.createdAt} {now} />
      {/if}
    </div>
  {:else}
    <span class="community-projects__file-panel-info-message">
      {blob.path}
    </span>
  {/if}
</section>
