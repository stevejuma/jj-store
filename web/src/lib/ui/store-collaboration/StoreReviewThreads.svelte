<script lang="ts">
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import GitPullRequestIcon from "@lucide/svelte/icons/git-pull-request";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import type { StoreController } from "../../controller/contracts.js";
  import { anchorAgentMember } from "../../collaboration/presentation.js";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import { estimateCommunityProjectRow } from "../community-projects/virtual-project-row-estimates.js";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import StoreWorkReviewRow from "../store-work/StoreWorkReviewRow.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import StoreDiscussionWorkspace from "../store-discussion/StoreDiscussionWorkspace.svelte";
  import StoreReviewDetail from "./StoreReviewDetail.svelte";
  import "../store-work/community-work-layout.css";
  import "../store-work/community-work-rows.css";
  import "../store-work/community-work-collections.css";
  import "../store-work/community-work-reviews.css";
  import "./store-collaboration.css";
  let { controller }: { controller: StoreController; layoutStorage?: Storage } =
    $props();
  const view = $derived(controller.state),
    detail = $derived(view.reviewThread);
</script>

<section
  class="store-collaboration"
  aria-label="Repository reviews"
  aria-busy={view.status.reviews === "loading"}
>
  <StoreDiscussionWorkspace
    open={Boolean(view.route.reviewThreadId)}
    title="Review discussion"
    onClose={() => controller.actions.selectReviewThread(null)}
  >
    {#snippet context()}
      <ScrollArea class="store-collaboration__review-scroll" type="hover">
        <div class="store-projects__review-directory">
          <header class="community-projects__surface-header">
            <div>
              <GitPullRequestIcon aria-hidden="true" />
              <h2>Reviews</h2>
            </div>
            <Button
              variant="ghost"
              size="icon-sm"
              aria-label="Refresh reviews"
              title="Refresh reviews"
              disabled={view.status.reviews === "loading" ||
                view.status.reviews === "refreshing"}
              onclick={() => controller.actions.refresh()}
              ><RefreshIcon aria-hidden="true" /></Button
            >
          </header>
          {#if view.status.error}<CommunityProjectsColumnAlert
              title="Reviews unavailable"
              description={view.status.error}
              tone="destructive"
            />{/if}
          {#if (view.status.connection === "connecting" || view.status.navigation === "loading" || view.status.reviews === "loading") && !view.reviewThreads.length}
            <div class="store-reviews__loading">
              <StoreCollaborationSkeleton label="Loading review threads" />
            </div>
          {:else if !view.reviewThreads.length}<StoreWorkEmpty
              kind="reviews"
              title="No review threads"
              description="Exact repository and code discussions will appear here."
            />
          {:else}
            <div
              class="community-projects__review-group-list community-projects__list"
              role="list"
              aria-label="Review threads"
            >
              <CommunityProjectsVirtualRows
                rows={view.reviewThreads}
                getKey={(thread) => thread.thread_id}
                estimateSize={(thread) =>
                  estimateCommunityProjectRow({
                    kind: "review",
                    text: thread?.presentation?.excerpt ?? undefined,
                  })}
                hasMore={view.canLoadMoreReviews}
                loading={view.status.reviews === "refreshing"}
                loadingAsListItem
                loadingLabel="Loading more reviews"
                onLoadMore={() => void controller.actions.loadMore("reviews")}
              >
                {#snippet row(thread)}
                  <div role="listitem">
                    <StoreWorkReviewRow
                      {thread}
                      agent={anchorAgentMember(
                        thread.anchor,
                        view.agentSessions,
                      )}
                      onOpen={(id) => controller.actions.selectReviewThread(id)}
                    />
                  </div>
                {/snippet}
              </CommunityProjectsVirtualRows>
            </div>
          {/if}
        </div>
      </ScrollArea>{/snippet}
    {#snippet discussion()}
      {#if (view.status.connection === "connecting" || view.status.navigation === "loading" || view.status.reviews === "loading" || view.status.reviewAction === "loading") && !detail}
        <div class="store-reviews__loading">
          <StoreCollaborationSkeleton label="Loading review details" />
        </div>
      {:else if detail}{#key detail.thread.thread_id}<StoreReviewDetail
            {controller}
          />{/key}
      {:else}<CommunityProjectsColumnAlert
          title="Review unavailable"
          description={view.status.error ||
            "The selected review could not be loaded."}
        />{/if}
    {/snippet}
  </StoreDiscussionWorkspace>
</section>

<style>
  :global(.store-collaboration__review-scroll) {
    width: 100%;
    min-width: 0;
    height: 100%;
    min-height: 0;
    flex: 1 1 auto;
  }
  .store-reviews__loading {
    padding: 1rem 1.5rem;
  }
</style>
