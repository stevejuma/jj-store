<script lang="ts">
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import StoreWorkCollection from "../store-work/StoreWorkCollection.svelte";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import BotIcon from "@lucide/svelte/icons/bot";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import StoreAgentSessionRow from "./StoreAgentSessionRow.svelte";
  import "./store-collaboration.css";

  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
</script>

<section
  class="store-collaboration"
  aria-label="Agent activity"
  aria-busy={state.status.agents === "loading"}
>
  <ScrollArea type="hover" class="store-agent-sessions__scroll">
    <StoreWorkCollection
      title="Agent sessions"
      count={state.agentSessions.length}
      Icon={BotIcon}
    >
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
            estimateSize={() => 82}
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
    </StoreWorkCollection>
  </ScrollArea>
</section>

<style>
  :global(.store-agent-sessions__scroll) {
    flex: 1;
    min-width: 0;
    min-height: 0;
    height: 100%;
  }
</style>
