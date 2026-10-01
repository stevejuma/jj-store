<script lang="ts">
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import BotIcon from "@lucide/svelte/icons/bot";
  import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
  import { CommunityMessage } from "@lapismd/lapis-community/components/community-message";
  import { isLocalAssociation } from "../../controller/repository-read-types.js";
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
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  import StoreMessageActions from "./StoreMessageActions.svelte";

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
  const localAssociation = $derived(
    association && isLocalAssociation(association) ? association : null,
  );
  const sourceRepository = $derived(
    localAssociation
      ? localAssociation.source_repositories.length === 1
        ? localAssociation.source_repositories[0]
        : null
      : state.repository.id,
  );
  const sourceOperation = $derived(
    localAssociation
      ? (localAssociation.evidence.find(
          (e) => e.repository === sourceRepository,
        )?.operation ?? null)
      : state.route.snapshotOperationId,
  );
  const changeId = $derived(association?.changes[0]);
  const commitId = $derived(
    association?.after_commits[0] ?? association?.before_commits[0],
  );
  const changeHref = $derived(
    selected && changeId && commitId && sourceRepository && sourceOperation
      ? (storeRouteToHash({
          ...state.route,
          page: "log",
          repository: sourceRepository ?? state.repository.id,
          snapshotOperationId: sourceOperation,
          bookmark: "",
          inspectedOperationId: null,
          commitId,
          changeId,
          comparisonParentId: null,
          comparisonGroupId: null,
          path: null,
          directory: "",
          agentSessionId:
            sourceRepository === state.repository.id
              ? selected.session.session_id
              : null,
          reviewThreadId: null,
        }) ?? "#")
      : null,
  );
</script>

{#if selected && agent}
  <div class="store-agent-context">
    <header class="store-collaboration__detail-header">
      {#if localAssociation}<p>
          Source repositories: {localAssociation.source_repositories.join(", ")}
        </p>{/if}
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
        <StoreRecordMenu
          label="Agent details actions"
          items={[
            { label: "Copy session ID", value: selected.session.session_id },
            { label: "Copy snapshot ID", value: selected.session.snapshot_id },
            { label: "Copy agent root", value: selected.snapshot.agent_root },
            { label: "Copy repository ID", value: selected.snapshot.repo_id },
            {
              label: "Copy epoch ID",
              value:
                "epoch" in selected.snapshot
                  ? selected.snapshot.epoch
                  : selected.snapshot.metadata_commit,
            },
          ]}
        />
      </div>
      <div class="store-collaboration__detail-meta">
        <CommunityChip
          tone={selected.session.state === "active" ? "success" : "neutral"}
        >
          {selected.session.state}
        </CommunityChip>

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
        {controller}
        activity={selected.activity}
        sessionId={selected.session.session_id}
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
                >{#snippet supplement()}<StoreMessageActions
                    authorName={agent.name}
                    items={[
                      { label: "Copy message", value: chunk.content },
                      {
                        label: "Copy session ID",
                        value: selected.session.session_id,
                      },
                      {
                        label: "Copy transcript ID",
                        value: selected.transcripts[0]?.transcript_id,
                      },
                      { label: "Copy sequence", value: String(chunk.sequence) },
                    ]}
                  />{/snippet}</CommunityMessage
              >
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
