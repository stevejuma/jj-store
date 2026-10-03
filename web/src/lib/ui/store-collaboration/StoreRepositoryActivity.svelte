<script lang="ts">
  // CommunityProjectsActivityFeed timeline and commit card (Apache-2.0).
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import GitCommitIcon from "@lucide/svelte/icons/git-commit-horizontal";
  import type { NativeActivityCommit } from "../../controller/native-activity-types.js";
  import type { StoreController } from "../../controller/contracts.js";
  import { initialStoreRoute } from "../../controller/store-route-history.svelte.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  import CommunityProjectsGitAuthorAvatar from "../community-projects/CommunityProjectsGitAuthorAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import StoreNativeAgentContext from "./StoreNativeAgentContext.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import { estimateCommunityProjectRow } from "../community-projects/virtual-project-row-estimates.js";
  import "./community-native-activity.css";
  let {
    commits,
    controller,
    query = "",
    loading = false,
    hasMore = false,
    onLoadMore,
  }: {
    commits: readonly NativeActivityCommit[];
    controller: StoreController;
    query?: string;
    loading?: boolean;
    hasMore?: boolean;
    onLoadMore?: () => void;
  } = $props();
  const visible = $derived(
    commits.filter((row) =>
      `${row.revision.message} ${row.revision.author} ${row.revision.eventId}`
        .toLowerCase()
        .includes(query.toLowerCase()),
    ),
  );
  function href(row: NativeActivityCommit) {
    return storeRouteToHash({
      ...initialStoreRoute(),
      page: "log",
      repository: row.repository,
      snapshotOperationId: row.operation,
      commitId: row.revision.eventId,
      changeId: row.revision.changeId,
    });
  }
</script>

{#if visible.length}<div
    class="community-projects__activity"
    role="list"
    aria-label="Repository activity"
  >
    <CommunityProjectsVirtualRows
      rows={visible}
      getKey={(row) => `${row.repository}:${row.revision.eventId}`}
      estimateSize={(row) =>
        estimateCommunityProjectRow(
          row
            ? {
                kind: "activity",
                text: row.revision.message,
                hasBody: row.revision.message.includes("\n"),
                hasSupplement: Boolean(
                  row.agents.discover || row.agents.sessions.length,
                ),
              }
            : undefined,
        )}
      {loading}
      {hasMore}
      {onLoadMore}
      loadingAsListItem
      loadingLabel="Loading more activity"
    >
      {#snippet row(row, index)}
        {@const revision = row.revision}
        <div
          class="community-projects__activity-item community-projects__activity-item--commit"
          role="listitem"
        >
          <CommunityProjectsGitAuthorAvatar
            name={revision.author}
            email={revision.authorEmail}
          />
          <span
            class="community-projects__activity-line"
            data-last={index === visible.length - 1 && !hasMore}
            aria-hidden="true"
          ></span>
          <div class="community-projects__activity-copy">
            <div class="community-projects__activity-meta">
              <div class="community-projects__activity-byline">
                <div>
                  <strong>{revision.author}</strong><span>committed in</span
                  ><span
                    >{controller.state.repositories.find(
                      (repo) => repo.id === row.repository,
                    )?.name ?? controller.state.repository.name}</span
                  >
                </div>
                <CommunityRelativeTime timestamp={revision.createdAt} />
              </div>
              <CommunityProjectsRevisionGraphIds
                commitId={revision.eventId}
                changeId={revision.changeId}
                placement="inline"
              />
            </div>
            <a
              class="community-projects__activity-card"
              href={href(row) ?? "#"}
              aria-label={`Open commit ${revision.message.split("\n")[0]}`}
            >
              <span
                class="community-projects__activity-title"
                data-activity-kind="commit"
                ><GitCommitIcon aria-hidden="true" /><strong
                  >{revision.message.split("\n")[0]}</strong
                ></span
              >
              {#if revision.message.includes("\n")}<div
                  class="community-projects__activity-body"
                >
                  <CommunityMarkdownContent
                    content={revision.message.slice(
                      revision.message.indexOf("\n") + 1,
                    )}
                  />
                </div>{/if}
            </a>
            <StoreNativeAgentContext context={row.agents} {controller} />
          </div>
        </div>
      {/snippet}
    </CommunityProjectsVirtualRows>
  </div>{:else}<StoreWorkEmpty
    kind="activity"
    title="No matching activity"
    description="Try another commit message, author or native ID."
  />{/if}
