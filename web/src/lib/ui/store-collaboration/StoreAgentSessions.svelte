<script lang="ts">
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreWorkCollection from "../store-work/StoreWorkCollection.svelte";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import BotIcon from "@lucide/svelte/icons/bot";
  import type { StoreController } from "../../controller/contracts.js";
  import CommunityProjectsColumnAlert from "../community-projects/CommunityProjectsColumnAlert.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreCollaborationSkeleton from "./StoreCollaborationSkeleton.svelte";
  import StoreAgentSessionRow from "./StoreAgentSessionRow.svelte";
  import StoreAgentExecutionDirectory from "../store-agent-execution/StoreAgentExecutionDirectory.svelte";
  import { managedRecordedAgent } from "../../collaboration/managed-agent.js";
  import "./store-collaboration.css";

  let { controller }: { controller: StoreController } = $props();
  const view = $derived(controller.state);
  let search = $state("");
  const filtered = $derived(
    view.agentSessions.filter((session) =>
      [
        session.conversation_title,
        session.discovery_text,
        session.session_id,
        session.integration,
      ].some((value) =>
        value?.toLocaleLowerCase().includes(search.trim().toLocaleLowerCase()),
      ),
    ),
  );
</script>

<section
  class="store-collaboration"
  aria-label="Agent activity"
  aria-busy={view.status.agents === "loading"}
>
  <ScrollArea type="hover" class="store-agent-sessions__scroll">
    <StoreAgentExecutionDirectory
      manager={controller.execution}
      context={{ repository: view.repository.id }}
    />
    <StoreWorkCollection
      id="agents"
      title="Agent sessions"
      count={view.agentSessions.length}
      Icon={BotIcon}
    >
      {#if view.status.error}
        <CommunityProjectsColumnAlert
          title="Agent activity unavailable"
          description={view.status.error}
          tone="destructive"
          inset="collection"
        />
      {/if}
      {#if (view.status.connection === "connecting" || view.status.navigation === "loading" || view.status.agents === "loading") && view.agentSessions.length === 0}
        <StoreCollaborationSkeleton label="Loading agent sessions" agents />
      {:else if view.agentSessions.length === 0}
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
        <Input
          type="search"
          aria-label="Search loaded recorded conversations"
          placeholder="Search captured conversations"
          bind:value={search}
        />
        <div
          class="store-agent-session-list"
          role="list"
          aria-label="Agent sessions"
        >
          <CommunityProjectsVirtualRows
            rows={filtered}
            getKey={(session) => session.session_id}
            estimateSize={() => 82}
            hasMore={view.canLoadMoreAgents && !search}
            loading={view.status.agents === "refreshing"}
            loadingAsListItem
            loadingLabel="Loading more agent sessions"
            onLoadMore={() => void controller.actions.loadMore("agents")}
          >
            {#snippet row(session)}
              {@const association =
                view.agentSession?.session.session_id === session.session_id
                  ? view.agentSession.code[0]
                  : view.agentRelations.find(
                      (row) =>
                        row.association?.session_id === session.session_id,
                    )?.association}
              <StoreAgentSessionRow
                {session}
                {association}
                managed={managedRecordedAgent(
                  view.repository.id,
                  session,
                  controller.execution?.value?.sessions,
                )}
                collection
                selectedId={view.route.agentSessionId}
                onSelect={controller.actions.selectAgentSession}
              />
            {/snippet}
          </CommunityProjectsVirtualRows>
        </div>
        {#if search && !filtered.length}<p role="status">
            No matches in loaded captures.
          </p>{/if}
        {#if search && view.canLoadMoreAgents}<Button
            variant="ghost"
            size="sm"
            onclick={() => controller.actions.loadMore("agents")}
            >Load more captured conversations</Button
          >{/if}
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
