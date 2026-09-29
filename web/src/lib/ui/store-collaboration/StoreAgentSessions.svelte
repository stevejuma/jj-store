<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { onDestroy, untrack } from "svelte";
  import BotIcon from "@lucide/svelte/icons/bot";
  import { CommunityMessage } from "@lapismd/lapis-community/components/community-message";
  import type { StoreController } from "../../controller/contracts.js";
  import { createCollaborationCanvas } from "../../collaboration/collaboration-canvas.js";
  import {
    agentMember,
    integrationLabel,
    shortIdentity,
    transcriptMessage,
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
  const canvas = createCollaborationCanvas("agents", storage);
  const state = $derived(controller.state);
  const selected = $derived(state.agentSession);
  const member = $derived(selected ? agentMember(selected.session) : undefined);
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
  <ColumnCanvas.Root controller={canvas}>
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
        {#if state.status.agents === "loading" && state.agentSessions.length === 0}
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
              {#snippet row(session)}<div role="listitem">
                  <ColumnCanvas.Item
                    class="store-collaboration__row"
                    selected={state.route.agentSessionId === session.session_id}
                    onclick={() =>
                      controller.actions.selectAgentSession(session.session_id)}
                  >
                    <span class="store-collaboration__row-title"
                      >{integrationLabel(session.integration)} · {shortIdentity(
                        session.session_id,
                      )}</span
                    >
                    <span class="store-collaboration__row-meta"
                      ><CommunityChip
                        tone={session.state === "active"
                          ? "success"
                          : "neutral"}>{session.state}</CommunityChip
                      >{session.transcript_count} transcripts · {session.tool_count}
                      tools</span
                    >
                  </ColumnCanvas.Item>
                </div>{/snippet}
            </CommunityProjectsVirtualRows>
          </div>
        {/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
    <ColumnCanvas.Column id="detail" title="Agent response">
      <ColumnCanvas.Body>
        {#if state.status.transcript === "loading" && !selected}
          <StoreCollaborationSkeleton label="Loading agent response" />
        {:else if selected && member}
          <header class="store-collaboration__detail-header">
            <span class="store-collaboration__detail-title"
              >{member.name} · {shortIdentity(
                selected.session.session_id,
              )}</span
            >
            <span class="store-collaboration__detail-meta"
              ><CommunityChip
                tone={selected.session.state === "active"
                  ? "success"
                  : "neutral"}>{selected.session.state}</CommunityChip
              >Pinned activity root {shortIdentity(
                selected.snapshot.agent_root,
              )} · code snapshot {shortIdentity(
                state.route.snapshotOperationId ?? "none",
              )}</span
            >
          </header>
          {#if state.agentTranscript.length}
            <div
              class="store-collaboration__messages"
              role="list"
              aria-label="Agent response transcript"
            >
              <CommunityProjectsVirtualRows
                rows={state.agentTranscript}
                getKey={(chunk) => String(chunk.sequence)}
                estimateSize={() => 118}
                hasMore={state.canLoadMoreTranscript}
                loading={state.status.transcript === "refreshing"}
                loadingAsListItem
                loadingLabel="Loading more transcript activity"
                onLoadMore={() =>
                  void controller.actions.loadMore("transcript")}
              >
                {#snippet row(chunk)}<div
                    class="store-collaboration__message"
                    role="listitem"
                  >
                    <CommunityMessage
                      message={transcriptMessage(chunk, selected.session)}
                      author={member}
                      members={[member]}
                      avatarShape="rounded"
                    />
                  </div>{/snippet}
              </CommunityProjectsVirtualRows>
            </div>
          {:else}<Empty.Root class="store-collaboration__empty"
              ><Empty.Header
                ><Empty.Media variant="icon"
                  ><BotIcon aria-hidden="true" /></Empty.Media
                ><Empty.Title>No transcript activity</Empty.Title
                ><Empty.Description
                  >This session has no selected transcript records.</Empty.Description
                ></Empty.Header
              ></Empty.Root
            >{/if}
        {:else}<Empty.Root class="store-collaboration__empty"
            ><Empty.Header
              ><Empty.Media variant="icon"
                ><BotIcon aria-hidden="true" /></Empty.Media
              ><Empty.Title>Select an agent session</Empty.Title
              ><Empty.Description
                >Choose a session to inspect its pinned, sanitized response
                activity.</Empty.Description
              ></Empty.Header
            ></Empty.Root
          >{/if}
      </ColumnCanvas.Body>
    </ColumnCanvas.Column>
  </ColumnCanvas.Root>
</section>
