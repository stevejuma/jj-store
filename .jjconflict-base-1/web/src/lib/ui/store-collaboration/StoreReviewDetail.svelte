<script lang="ts">
  import { sectionDisclosure } from "../disclosure-layout.svelte.js";
  // CommunityProjectsPullRequestReview/DetailSurface, adapted native anchors (Apache-2.0).
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import * as Chat from "@lapismd/design-core/ai/chat";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import GitPullRequestIcon from "@lucide/svelte/icons/git-pull-request";
  import UserIcon from "@lucide/svelte/icons/user";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import CircleCheckIcon from "@lucide/svelte/icons/circle-check";
  import RotateCcwIcon from "@lucide/svelte/icons/rotate-ccw";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    anchorLabel,
    principalMember,
    agentMember,
  } from "../../collaboration/presentation.js";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";
  import StoreReviewAnchorIdentity from "./StoreReviewAnchorIdentity.svelte";
  import StoreReviewConversation from "./StoreReviewConversation.svelte";
  import StoreReviewComposer from "./StoreReviewComposer.svelte";
  import { reviewDraft } from "./review-drafts.svelte.js";
  import StoreRepositoryActivity from "./StoreRepositoryActivity.svelte";
  import StoreNativeAgentContext from "./StoreNativeAgentContext.svelte";
  import StoreReviewFiles from "./StoreReviewFiles.svelte";
  import StoreReviewCommits from "./StoreReviewCommits.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import "./community-native-review.css";
  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state),
    detail = $derived(view.reviewThread),
    context = $derived(view.reviewContext);
  const first = $derived(
    detail?.comments.find((comment) => comment.parent_comment_id === null),
  );
  const title = $derived(
    detail?.thread.presentation?.excerpt?.split("\n")[0] ||
      (detail ? anchorLabel(detail.anchor.value) : "Review"),
  );
  const draft = $derived(
    reviewDraft(controller, detail?.thread.thread_id ?? ""),
  );
  const writable = $derived(
    view.repository.permission === "write" ||
      view.repository.permission === "admin",
  );
  const section = (id: string, fallback = true) =>
    sectionDisclosure(
      () => ["review", detail?.thread.thread_id ?? "", id],
      fallback,
    );
  const description = section("description"),
    files = section("files", false),
    commits = section("commits", false),
    activity = section("activity");
</script>

{#snippet chevron(open: boolean)}{#if open}<ChevronDownIcon
      aria-hidden="true"
    />{:else}<ChevronRightIcon aria-hidden="true" />{/if}{/snippet}

{#if detail}
  <article
    class="community-projects__review-detail"
    aria-label="Review details"
  >
    <header class="community-projects__review-header">
      <Button
        variant="ghost"
        size="icon-sm"
        class="community-projects__review-back"
        aria-label="Back to reviews"
        title="Back to reviews"
        onclick={() => controller.actions.selectReviewThread(null)}
        ><ArrowLeftIcon aria-hidden="true" /></Button
      >
      <div>
        <h2>{title} <span>#{detail.thread.thread_id.slice(0, 8)}</span></h2>
        <p>
          Review opened <CommunityRelativeTime
            timestamp={Number(
              detail.thread.presentation?.created_at_millis ??
                first?.created_at_millis ??
                0,
            ) / 1000}
          />
        </p>
      </div>
    </header>
    <ScrollArea
      class="store-collaboration__review-scroll"
      type="hover"
      role="region"
      aria-label="Review content"
    >
      <div class="community-projects__review-body">
        <dl class="community-projects__review-meta">
          <div>
            <dt>
              <GitPullRequestIcon aria-hidden="true" /><span>Anchor</span>
            </dt>
            <dd>
              {#if context?.agents.sessions[0]}<CommunityMemberAvatar
                  member={agentMember(context.agents.sessions[0].session)}
                  size="small"
                  decorative
                />{/if}
              {anchorLabel(detail.anchor.value)}<StoreReviewAnchorIdentity
                anchor={detail.anchor.value}
              />
            </dd>
          </div>
          <div>
            <dt><UserIcon aria-hidden="true" /><span>Author</span></dt>
            <dd>
              {principalMember(
                detail.thread.presentation?.created_by ??
                  first?.author ??
                  "local",
              ).name}
            </dd>
          </div>
          <div>
            <dt>
              <GitPullRequestIcon aria-hidden="true" /><span>Status</span>
            </dt>
            <dd>
              {detail.thread.status}<Button
                variant="secondary"
                size="xs"
                disabled={view.status.reviewAction === "refreshing" ||
                  view.status.reviewAction === "loading"}
                onclick={() =>
                  controller.actions.setReviewResolved(
                    detail.thread.status === "open",
                  )}
                >{#if detail.thread.status === "open"}<CircleCheckIcon
                    data-icon="inline-start"
                    aria-hidden="true"
                  />Resolve{:else}<RotateCcwIcon
                    data-icon="inline-start"
                    aria-hidden="true"
                  />Reopen{/if}</Button
              >
            </dd>
          </div>
          <div>
            <dt>Snapshot</dt>
            <dd>
              {#if detail.snapshot.review_root}<StoreIdentityCopy
                  value={detail.snapshot.review_root}
                  label="review root"
                />{/if}<StoreIdentityCopy
                value={detail.thread.thread_id}
                label="review thread ID"
              />
            </dd>
          </div>
        </dl>
        {#if view.reviewActionMessage}<CommunityProjectsColumnAlert
            title={view.status.reviewAction === "unavailable"
              ? "Review outcome unknown"
              : "Review updated"}
            description={view.reviewActionMessage}
            >{#if view.reviewActionRequestId}<Button
                variant="outline"
                size="sm"
                onclick={() => controller.actions.checkReviewAction()}
                ><RefreshIcon
                  data-icon="inline-start"
                  aria-hidden="true"
                />Check status</Button
              >{/if}</CommunityProjectsColumnAlert
          >{/if}
        <Collapsible.Root
          class="community-projects__review-section"
          bind:open={description.open}
        >
          <Collapsible.Trigger
            class="community-projects__review-section-trigger"
            ><span>Description</span>{@render chevron(
              description.open,
            )}</Collapsible.Trigger
          >
          <Collapsible.Content
            class="community-projects__review-section-content"
            >{#if first?.content.state === "visible"}<CommunityMarkdownContent
                content={first.content.text}
                fullWidth
              />{:else}<p class="community-projects__muted">
                The original description is unavailable.
              </p>{/if}</Collapsible.Content
          >
        </Collapsible.Root>
        <Collapsible.Root
          class="community-projects__review-section community-projects__review-section--files"
          bind:open={files.open}
        >
          <Collapsible.Trigger
            class="community-projects__review-section-trigger community-projects__review-section-trigger--files"
            ><span>Files changed</span>{@render chevron(
              files.open,
            )}{#if files.open && context}<span
                class="community-projects__review-section-count"
                >{context.files.length}</span
              >{/if}</Collapsible.Trigger
          >
          <Collapsible.Content class="community-projects__review-files-content">
            {#if !context && view.status.reviewAction === "loading"}<StoreCollaborationSkeleton
                label="Loading review files"
              />{:else if context}<StoreReviewFiles
                {context}
                {controller}
              />{/if}
          </Collapsible.Content>
        </Collapsible.Root>
        <Collapsible.Root
          class="community-projects__review-section"
          bind:open={commits.open}
        >
          <Collapsible.Trigger
            class="community-projects__review-section-trigger"
            ><span>Commits</span>{@render chevron(
              commits.open,
            )}</Collapsible.Trigger
          >
          <Collapsible.Content
            ><StoreReviewCommits
              commits={context?.commits ?? []}
            /></Collapsible.Content
          >
        </Collapsible.Root>
        <Collapsible.Root
          class="community-projects__review-section"
          bind:open={activity.open}
        >
          <Collapsible.Trigger
            class="community-projects__review-section-trigger"
            ><span>Activity</span>{@render chevron(
              activity.open,
            )}</Collapsible.Trigger
          >
          <Collapsible.Content>
            {#if context?.commits.length}<StoreRepositoryActivity
                commits={context.commits}
                {controller}
              />{/if}
            {#if context?.agents}<div
                class="community-projects__review-section-content"
              >
                <StoreNativeAgentContext
                  context={context.agents}
                  {controller}
                />
              </div>{/if}
            {#each detail.events.filter((event) => !["created", "reply_added"].includes(event.event.action)) as event (event.event_id)}<Chat.SystemMessage
                >{principalMember(event.actor).name}
                {event.event.action.replaceAll("_", " ")}
                <CommunityRelativeTime
                  timestamp={Number(event.accepted_at_millis) / 1000}
                /></Chat.SystemMessage
              >{/each}
            <StoreReviewConversation
              {controller}
              showDiff={false}
              composerPlacement="external"
            />
          </Collapsible.Content>
        </Collapsible.Root>
      </div>
    </ScrollArea>
    {#if writable && (detail.thread.status === "open" || draft.reply || draft.edit)}
      <footer
        class="community-projects__review-footer"
        aria-label="Review comment composer"
      >
        <StoreReviewComposer
          {controller}
          inline={false}
          path={undefined}
          page
        />
      </footer>
    {/if}
  </article>
{/if}
