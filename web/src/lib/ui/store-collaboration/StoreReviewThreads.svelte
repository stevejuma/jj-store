<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { onDestroy, untrack } from "svelte";
  import MessageSquareIcon from "@lucide/svelte/icons/message-square";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import { CommunityMessage } from "@lapismd/lapis-community/components/community-message";
  import type { StoreController } from "../../controller/contracts.js";
  import { createCollaborationCanvas } from "../../collaboration/collaboration-canvas.js";
  import {
    anchorLabel,
    principalMember,
    reviewMessage,
    shortIdentity,
  } from "../../collaboration/presentation.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import "./store-collaboration.css";

  let {
    controller,
    layoutStorage,
  }: { controller: StoreController; layoutStorage?: Storage } = $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const canvas = createCollaborationCanvas("reviews", storage);
  const view = $derived(controller.state);
  const detail = $derived(view.reviewThread);
  const members = $derived(
    detail
      ? [
          ...new Map(
            detail.comments.map((comment) => [
              comment.author,
              principalMember(comment.author),
            ]),
          ).values(),
        ]
      : [],
  );
  let draft = $state("");
  onDestroy(() => void canvas.dispose());

  $effect(() => {
    canvas.layoutReady;
    const id = view.route.reviewThreadId;
    untrack(() => {
      canvas.clear();
      if (id) canvas.select(0, id);
    });
  });

  async function submit(content: string) {
    await controller.actions.replyToReview(content);
    if (controller.state.status.reviewAction === "ready") draft = "";
  }
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
                  <ColumnCanvas.Item
                    class="store-collaboration__row"
                    selected={view.route.reviewThreadId === thread.thread_id}
                    onclick={() =>
                      controller.actions.selectReviewThread(thread.thread_id)}
                    ><span class="store-collaboration__row-title"
                      >Thread {shortIdentity(thread.thread_id)}</span
                    ><span class="store-collaboration__row-meta"
                      ><CommunityChip
                        tone={thread.status === "open" ? "accent" : "success"}
                        >{thread.status}</CommunityChip
                      >{thread.comment_count} comments · version {thread.thread_version}</span
                    ></ColumnCanvas.Item
                  >
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
        {:else if detail}
          <header class="store-collaboration__detail-header">
            <span class="store-collaboration__detail-title"
              >{anchorLabel(detail.anchor.value)}</span
            >
            <span class="store-collaboration__detail-meta"
              ><CommunityChip
                tone={detail.thread.status === "open" ? "accent" : "success"}
                >{detail.thread.status}</CommunityChip
              >Pinned review root {shortIdentity(detail.snapshot.review_root)} · code
              snapshot {shortIdentity(
                view.route.snapshotOperationId ?? "none",
              )}</span
            >
            <div class="store-collaboration__actions">
              <Button
                variant="outline"
                size="sm"
                disabled={view.status.reviewAction === "refreshing"}
                onclick={() =>
                  controller.actions.setReviewResolved(
                    detail.thread.status === "open",
                  )}
                >{detail.thread.status === "open"
                  ? "Resolve"
                  : "Reopen"}</Button
              >
            </div>
          </header>
          {#if view.reviewActionMessage}<CommunityProjectsColumnAlert
              title={view.status.reviewAction === "unavailable"
                ? "Review outcome unknown"
                : "Review updated"}
              description={view.reviewActionMessage}
            >
              {#if view.reviewActionRequestId}<Button
                  variant="outline"
                  size="sm"
                  onclick={() => controller.actions.checkReviewAction()}
                  ><RefreshCwIcon
                    data-icon="inline-start"
                    aria-hidden="true"
                  />Check status</Button
                >{/if}
            </CommunityProjectsColumnAlert>{/if}
          {#if detail.thread.status === "open"}
            <div class="store-collaboration__composer">
              <CommunityChatComposer
                mode="form"
                showSubmitButton
                bind:value={draft}
                {members}
                disabled={view.status.reviewAction === "refreshing"}
                busy={view.status.reviewAction === "refreshing"}
                ariaLabel="Reply to review thread"
                submitLabel="Reply"
                placeholder="Reply with Markdown"
                onSubmit={(submission) => submit(submission.content)}
              />
            </div>
          {/if}
          <div
            class="store-collaboration__messages"
            role="list"
            aria-label="Review comments"
          >
            <CommunityProjectsVirtualRows
              rows={detail.comments}
              getKey={(comment) => comment.comment_id}
              estimateSize={() => 118}
            >
              {#snippet row(comment)}{@const author = principalMember(
                  comment.author,
                )}
                <div class="store-collaboration__message" role="listitem">
                  <CommunityMessage
                    message={reviewMessage(comment)}
                    {author}
                    {members}
                    highlighted={comment.parent_comment_id !== null}
                  />
                </div>{/snippet}
            </CommunityProjectsVirtualRows>
          </div>
        {:else}<Empty.Root class="store-collaboration__empty"
            ><Empty.Header
              ><Empty.Media variant="icon"
                ><MessageSquareIcon aria-hidden="true" /></Empty.Media
              ><Empty.Title>Select a review thread</Empty.Title
              ><Empty.Description
                >Choose a thread to inspect its exact anchor and discussion.</Empty.Description
              ></Empty.Header
            ></Empty.Root
          >{/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
  </ColumnCanvas.Root>
</section>
