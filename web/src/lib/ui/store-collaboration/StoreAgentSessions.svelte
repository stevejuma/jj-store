<script lang="ts">
  import * as ColumnCanvas from "@lapismd/design-core/shadcn/column-canvas";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import { onDestroy, untrack } from "svelte";
  import BotIcon from "@lucide/svelte/icons/bot";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    COLLABORATION_COMPACT_BREAKPOINT,
    createCollaborationCanvas,
  } from "../../collaboration/collaboration-canvas.js";
  import { resolveLayoutStorage } from "../layout-storage.js";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import StoreAgentSessionRow from "./StoreAgentSessionRow.svelte";
  import StoreAgentContext from "./StoreAgentContext.svelte";
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
                {@const association =
                  state.agentSession?.session.session_id === session.session_id
                    ? state.agentSession.code[0]
                    : state.agentRelations.find(
                        (row) =>
                          row.association?.session_id === session.session_id,
                      )?.association}
                <StoreAgentSessionRow
                  {session}
                  {association}
                  selectedId={state.route.agentSessionId}
                  onSelect={controller.actions.selectAgentSession}
                />
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
