<script lang="ts">
  import FolderGitIcon from "@lucide/svelte/icons/folder-git-2";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { captureSourceMember } from "./capture-source-presentation.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  import { emptyRoute } from "../../controller/global-search-presentation.js";
  import type { Model } from "../../api/client.js";
  let {
    source,
    label,
    repositoryName,
    onRepository,
    navigationReason = "",
  }: {
    source: Model<"CaptureSourceCoverage">;
    label: string;
    repositoryName: (id: string) => string;
    onRepository?: (id: string) => void;
    navigationReason?: string;
  } = $props();
  const repository = $derived(source.last_bound_repository);
  const href = $derived(
    repository
      ? storeRouteToHash({ ...emptyRoute(repository), page: "files" })
      : null,
  );
</script>

<div class="store-capture-source__identity">
  <CommunityMemberAvatar
    member={captureSourceMember(source)}
    size="small"
    decorative
  />
  <div class="store-settings__stack">
    <strong>{label}</strong>
    {#if repository && href}
      <a
        class="store-capture-source__repository"
        href={navigationReason ? undefined : href}
        role={navigationReason ? "link" : undefined}
        aria-disabled={!!navigationReason}
        tabindex={navigationReason ? -1 : undefined}
        title={navigationReason || undefined}
        onclick={(event) => {
          if (navigationReason || onRepository) event.preventDefault();
          if (!navigationReason) onRepository?.(repository);
        }}
        ><FolderGitIcon aria-hidden="true" /><span
          >{repositoryName(repository)}</span
        ></a
      >
    {:else}
      <span class="store-settings__row-description"
        >No recorded repository binding</span
      >
    {/if}
    <span class="store-settings__row-description">{source.product_version}</span
    >
  </div>
</div>

<style>
  .store-capture-source__identity {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }
  .store-capture-source__repository {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    width: fit-content;
    font-size: 0.875rem;
    color: var(--muted-foreground);
    text-decoration: none;
  }
  .store-capture-source__repository:hover {
    color: var(--foreground);
    text-decoration: underline;
  }
  .store-capture-source__repository[aria-disabled="true"] {
    opacity: 0.5;
    cursor: default;
  }
  .store-capture-source__repository :global(svg) {
    width: 1rem;
    height: 1rem;
    flex: none;
  }
</style>
