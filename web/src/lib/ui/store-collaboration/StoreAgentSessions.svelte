<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { onDestroy, untrack } from "svelte";
  import BotIcon from "@lucide/svelte/icons/bot";
  import MessagesSquareIcon from "@lucide/svelte/icons/messages-square";
  import WrenchIcon from "@lucide/svelte/icons/wrench";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    COLLABORATION_COMPACT_BREAKPOINT,
    createCollaborationCanvas,
  } from "../../collaboration/collaboration-canvas.js";
  import {
    agentMember,
    agentSessionTiming,
  } from "../../collaboration/presentation.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import StoreAgentContext from "./StoreAgentContext.svelte";
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import "./store-collaboration.css";

  let {
    controller,
    layoutStorage,
  }: { controller: StoreController; layoutStorage?: Storage } = $props();
  const storage = untrack(() => resolveLayoutStorage(layoutStorage));
  const canvas = createCollaborationCanvas("agents", storage);
  const state = $derived(controller.state);
  const selected = $derived(state.agentSession);
  onDestroy(() => void canvas.dispose());

  $effect(() => {
    canvas.layoutReady;
    const id = state.route.agentSessionId;
    untrack(() => {
      canvas.clear();
      if (id) canvas.select(0, id);
    });
  });
</script>

<section
  class="store-collaboration"
  aria-label="Agent activity"
  aria-busy={state.status.agents === "loading"}
>
  <ColumnCanvas.Root
    controller={canvas}
    compactBreakpoint={COLLABORATION_COMPACT_BREAKPOINT}
  >
    <ColumnCanvas.Column
      id="collection"
      title="Agent sessions"
      count={state.agentSessions.length}
    >
      <ColumnCanvas.Body>
        {#if state.status.error}<CommunityProjectsColumnAlert
            title="Agent activity unavailable"
            description={state.status.error}
            tone="destructive"
          />{/if}
        {#if (state.status.connection === "connecting" || state.status.navigation === "loading" || state.status.agents === "loading") && state.agentSessions.length === 0}
          <StoreCollaborationSkeleton label="Loading agent sessions" />
        {:else if state.agentSessions.length === 0}
          <Empty.Root class="store-collaboration__empty">
            <Empty.Header
              ><Empty.Media variant="icon"
                ><BotIcon aria-hidden="true" /></Empty.Media
              ><Empty.Title>No agent sessions</Empty.Title><Empty.Description
                >Captured Codex, Cursor, and OpenCode sessions will appear here.</Empty.Description
              ></Empty.Header
            >
          </Empty.Root>
        {:else}
          <div role="list" aria-label="Agent sessions">
            <CommunityProjectsVirtualRows
              rows={state.agentSessions}
              getKey={(session) => session.session_id}
              estimateSize={() => 78}
              hasMore={state.canLoadMoreAgents}
              loading={state.status.agents === "refreshing"}
              loadingAsListItem
              loadingLabel="Loading more agent sessions"
              onLoadMore={() => void controller.actions.loadMore("agents")}
            >
              {#snippet row(session)}
                {@const agent = agentMember(session)}
                {@const timing = agentSessionTiming(session)}
                {@const association =
                  state.agentSession?.session.session_id === session.session_id
                    ? state.agentSession.code[0]
                    : state.agentRelations.find(
                        (row) =>
                          row.association?.session_id === session.session_id,
                      )?.association}
                <div role="listitem">
                  <div
                    class="store-collaboration__row"
                    data-kind="agent"
                    data-selected={state.route.agentSessionId ===
                    session.session_id
                      ? "true"
                      : undefined}
                  >
                    <button
                      type="button"
                      class="store-collaboration__row-select"
                      aria-label={`Open ${agent.name} ${session.state} agent session`}
                      aria-pressed={state.route.agentSessionId ===
                        session.session_id}
                      onclick={() =>
                        controller.actions.selectAgentSession(
                          session.session_id,
                        )}
                    ></button>
                    <span class="store-collaboration__row-leading">
                      <CommunityMemberAvatar
                        member={agent}
                        size="small"
                        showPresence
                        decorative
                      />
                    </span>
                    <span class="store-collaboration__row-main">
                      <span class="store-collaboration__row-title"
                        >{agent.name}</span
                      >
                      <span class="store-collaboration__row-meta"
                        ><CommunityChip
                          size="compact"
                          tone={session.state === "active"
                            ? "success"
                            : "neutral"}>{session.state}</CommunityChip
                        >{#if timing}<span
                            class="store-collaboration__agent-start"
                            >Started <CommunityRelativeTime
                              timestamp={timing.startedAt}
                            /></span
                          >{/if}<span class="store-collaboration__agent-metrics"
                          ><span
                            class="store-collaboration__metric"
                            aria-label={`${session.transcript_count} transcripts`}
                            title={`${session.transcript_count} transcripts`}
                            ><MessagesSquareIcon
                              aria-hidden="true"
                            />{session.transcript_count}</span
                          ><span
                            class="store-collaboration__metric"
                            aria-label={`${session.tool_count} tools`}
                            title={`${session.tool_count} tools`}
                            ><WrenchIcon
                              aria-hidden="true"
                            />{session.tool_count}</span
                          >{#if timing}<CommunityChip
                              class="store-collaboration__agent-duration"
                              variant="subtle"
                              size="compact"
                              ariaLabel={`Session duration ${timing.durationLabel}`}
                              title={`Session duration ${timing.durationLabel}`}
                              >{timing.durationLabel}</CommunityChip
                            >{/if}</span
                        ></span
                      >
                    </span>
                    <span class="store-collaboration__row-actions">
                      {#if association}<CommunityProjectsRevisionGraphIds
                          commitId={association.after_commits[0] ??
                            association.before_commits[0] ??
                            ""}
                          changeId={association.changes[0]}
                          placement="inline"
                        />{/if}
                      <StoreRecordMenu
                        label="Agent session actions"
                        items={[
                          {
                            label: "Copy session ID",
                            value: session.session_id,
                          },
                          {
                            label: "Copy snapshot ID",
                            value: session.snapshot_id,
                          },
                          {
                            label: "Copy parent session ID",
                            value: session.parent_session_id,
                          },
                        ]}
                      />
                    </span>
                  </div>
                </div>
              {/snippet}
            </CommunityProjectsVirtualRows>
          </div>
        {/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
    <ColumnCanvas.Column id="detail" title="Agent response">
      <ColumnCanvas.Body>
        {#if (state.status.transcript === "loading" || ((state.status.connection === "connecting" || state.status.navigation === "loading" || state.status.agents === "loading") && state.route.agentSessionId)) && !selected}
          <StoreCollaborationSkeleton label="Loading agent response" />
        {:else}<StoreAgentContext {controller} />{/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
  </ColumnCanvas.Root>
</section>
