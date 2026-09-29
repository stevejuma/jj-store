<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import BotIcon from "@lucide/svelte/icons/bot";
  import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
  import { CommunityMessage } from "@lapismd/lapis-community/components/community-message";
  import type { StoreController } from "../../controller/contracts.js";
  import {
    agentMember,
    transcriptMessage,
  } from "../../collaboration/presentation.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import CommunityProjectsVirtualRows from "../community-projects/CommunityProjectsVirtualRows.svelte";
  import StoreAgentTranscript from "./StoreAgentTranscript.svelte";
  import StoreIdentityCopy from "./StoreIdentityCopy.svelte";

  let { controller }: { controller: StoreController } = $props();
  const state = $derived(controller.state);
  const selected = $derived(state.agentSession);
  const agent = $derived(selected ? agentMember(selected.session) : undefined);
  const association = $derived.by(
    () =>
      selected?.code[0] ??
      state.agentRelations.find(
        (relation) =>
          relation.kind === "association" &&
          relation.association?.session_id === selected?.session.session_id,
      )?.association,
  );
  const changeId = $derived(
    association?.changes[0] ?? state.route.changeId ?? undefined,
  );
  const commitId = $derived(
    association?.after_commits[0] ??
      association?.before_commits[0] ??
      state.route.commitId ??
      undefined,
  );
  const changeHref = $derived(
    selected && changeId && commitId
      ? (storeRouteToHash({
          ...state.route,
          page: "log",
          bookmark: "",
          inspectedOperationId: null,
          commitId,
          changeId,
          comparisonParentId: null,
          comparisonGroupId: null,
          path: null,
          directory: "",
          agentSessionId: selected.session.session_id,
          reviewThreadId: null,
        }) ?? "#")
      : null,
  );
</script>

{#if selected && agent}
  <div class="store-agent-context">
    <header class="store-collaboration__detail-header">
      <div class="store-collaboration__detail-title-row">
        <span class="store-collaboration__detail-identity">
          <CommunityMemberAvatar
            member={agent}
            size="small"
            showPresence
            decorative
          />
          <span class="store-collaboration__detail-title">{agent.name}</span>
        </span>
        <StoreIdentityCopy
          value={selected.session.session_id}
          label="agent session ID"
        />
      </div>
      <div class="store-collaboration__detail-meta">
        <CommunityChip
          tone={selected.session.state === "active" ? "success" : "neutral"}
        >
          {selected.session.state}
        </CommunityChip>
        <StoreIdentityCopy
          value={selected.snapshot.agent_root}
          label="agent root"
        />
        {#if changeId || commitId}
          <CommunityProjectsRevisionGraphIds
            commitId={commitId ?? ""}
            {changeId}
            placement="inline"
          />
        {/if}
        {#if changeHref}
          <a class="store-collaboration__context-link" href={changeHref}>
            Open related change <ExternalLinkIcon aria-hidden="true" />
          </a>
        {/if}
      </div>
      {#if selected.activity_truncated || selected.code_truncated}
        <p class="store-collaboration__coverage" role="status">
          This bounded view omits older {selected.activity_truncated
            ? "activity"
            : "code associations"}.
        </p>
      {/if}
    </header>
    {#if selected.activity.length}
      <StoreAgentTranscript
        activity={selected.activity}
        {agent}
        member={{
          ...state.member,
          kind: state.member.kind === "agent" ? "agent" : undefined,
        }}
        patches={state.detail?.patches}
      />
    {:else if state.agentTranscript.length}
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
          onLoadMore={() => void controller.actions.loadMore("transcript")}
        >
          {#snippet row(chunk)}
            <div class="store-collaboration__message" role="listitem">
              <CommunityMessage
                message={transcriptMessage(chunk, selected.session)}
                author={agent}
                members={[agent]}
                avatarShape="rounded"
              />
            </div>
          {/snippet}
        </CommunityProjectsVirtualRows>
      </div>
    {:else}
      <Empty.Root class="store-collaboration__empty">
        <Empty.Header>
          <Empty.Media variant="icon"
            ><BotIcon aria-hidden="true" /></Empty.Media
          >
          <Empty.Title>No transcript activity</Empty.Title>
          <Empty.Description
            >This session has no selected transcript records.</Empty.Description
          >
        </Empty.Header>
      </Empty.Root>
    {/if}
  </div>
{:else}
  <Empty.Root class="store-collaboration__empty">
    <Empty.Header>
      <Empty.Media variant="icon"><BotIcon aria-hidden="true" /></Empty.Media>
      <Empty.Title>Select an agent session</Empty.Title>
      <Empty.Description
        >Choose a session to inspect its pinned, sanitized activity.</Empty.Description
      >
    </Empty.Header>
  </Empty.Root>
{/if}
