<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { onDestroy, untrack } from "svelte";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import type { StoreController } from "../../controller/contracts.js";
  import { createCollaborationCanvas } from "../../collaboration/collaboration-canvas.js";
  import { anchorLabel } from "../../collaboration/presentation.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";
  import StoreReviewAnchorIdentity from "./StoreReviewAnchorIdentity.svelte";
  import StoreReviewDiscussion from "./StoreReviewDiscussion.svelte";
  import "./store-collaboration.css";

  let {
    controller,
    layoutStorage,
  }: { controller: StoreController; layoutStorage?: Storage } = $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const canvas = createCollaborationCanvas("reviews", storage);
  const view = $derived(controller.state);
  const detail = $derived(view.reviewThread);
  onDestroy(() => void canvas.dispose());

  $effect(() => {
    canvas.layoutReady;
    const id = view.route.reviewThreadId;
    untrack(() => {
      canvas.clear();
      if (id) canvas.select(0, id);
    });
  });
</script>

<section
  class="store-collaboration"
  aria-label="Repository reviews"
  aria-busy={view.status.reviews === "loading"}
>
  <ColumnCanvas.Root controller={canvas}>
    <ColumnCanvas.Column
      id="collection"
      title="Review threads"
      count={view.reviewThreads.length}
    >
      <ColumnCanvas.Body>
        {#if view.status.error}<CommunityProjectsColumnAlert
            title="Reviews unavailable"
            description={view.status.error}
            tone="destructive"
          />{/if}
        {#if view.status.reviews === "loading" && view.reviewThreads.length === 0}
          <StoreCollaborationSkeleton label="Loading review threads" />
        {:else if view.reviewThreads.length === 0}
          <Empty.Root class="store-collaboration__empty"
            ><Empty.Header
              ><Empty.Media variant="icon"
                ><MessageSquareIcon aria-hidden="true" /></Empty.Media
              ><Empty.Title>No review threads</Empty.Title><Empty.Description
                >Exact repository and code discussions will appear here.</Empty.Description
              ></Empty.Header
            ></Empty.Root
          >
        {:else}
          <div role="list" aria-label="Review threads">
            <CommunityProjectsVirtualRows
              rows={view.reviewThreads}
              getKey={(thread) => thread.thread_id}
              estimateSize={() => 78}
              hasMore={view.canLoadMoreReviews}
              loading={view.status.reviews === "refreshing"}
              loadingAsListItem
              loadingLabel="Loading more review threads"
              onLoadMore={() => void controller.actions.loadMore("reviews")}
            >
              {#snippet row(thread)}<div role="listitem">
                  <div
                    class="store-collaboration__row"
                    data-selected={view.route.reviewThreadId ===
                    thread.thread_id
                      ? "true"
                      : undefined}
                  >
                    <button
                      type="button"
                      class="store-collaboration__row-select"
                      aria-pressed={view.route.reviewThreadId ===
                        thread.thread_id}
                      onclick={() =>
                        controller.actions.selectReviewThread(thread.thread_id)}
                    >
                      <span class="store-collaboration__row-title"
                        >{anchorLabel(thread.anchor)}</span
                      >
                      <span class="store-collaboration__row-meta"
                        ><CommunityChip
                          tone={thread.status === "open" ? "accent" : "success"}
                          >{thread.status}</CommunityChip
                        >{thread.comment_count} comments · version {thread.thread_version}</span
                      >
                    </button>
                    <span class="store-collaboration__row-actions">
                      <StoreIdentityCopy
                        value={thread.thread_id}
                        label="review thread ID"
                      />
                      <StoreReviewAnchorIdentity anchor={thread.anchor} />
                    </span>
                  </div>
                </div>{/snippet}
            </CommunityProjectsVirtualRows>
          </div>
        {/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
    <ColumnCanvas.Column id="detail" title="Discussion">
      <ColumnCanvas.Body>
        {#if view.status.reviewAction === "loading" && !detail}
          <StoreCollaborationSkeleton label="Loading review discussion" />
        {:else}<StoreReviewDiscussion {controller} />{/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
  </ColumnCanvas.Root>
</section>
