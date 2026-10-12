<script lang="ts">
  import type { Snippet } from "svelte";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import BotIcon from "@lucide/svelte/icons/bot";
  import ExternalLinkIcon from "@lucide/svelte/icons/external-link";
  import { isLocalAssociation } from "../../controller/repository-read-types.js";
  import type { StoreController } from "../../controller/contracts.js";
  import { agentMember } from "../../collaboration/presentation.js";
  import { storeRouteToHash } from "../../controller/store-route-codec.js";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import StoreAgentTranscript from "./StoreAgentTranscript.svelte";
  import StoreConversationMetadata from "../store-agent-execution/StoreConversationMetadata.svelte";
  import { metadataKey } from "../../controller/conversation-metadata.svelte.js";
  import StoreAgentSessionMenu from "./StoreAgentSessionMenu.svelte";
  import { managedRecordedAgent } from "../../collaboration/managed-agent.js";

  let {
    controller,
    showHeader = true,
    afterItems,
  }: {
    controller: StoreController;
    showHeader?: boolean;
    afterItems?: Snippet;
  } = $props();
  const state = $derived(controller.state);
  const selected = $derived(state.agentSession);
  const agent = $derived(
    selected
      ? agentMember(
          selected.session,
          managedRecordedAgent(
            state.repository.id,
            selected.session,
            controller.execution?.value?.sessions,
          ),
        )
      : undefined,
  );
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

{#each selected && agent ? [{ detail: selected, author: agent }] : [] as { detail, author } (detail.session.session_id)}
  <div class="store-agent-context" data-header={showHeader}>
    {#if showHeader}<header class="store-collaboration__detail-header">
        {#if localAssociation}<p>
            Source repositories: {localAssociation.source_repositories.join(
              ", ",
            )}
          </p>{/if}
        <div class="store-collaboration__detail-title-row">
          <span class="store-collaboration__detail-identity">
            <CommunityMemberAvatar
              member={author}
              size="small"
              showPresence
              decorative
            />
            <span class="store-collaboration__detail-title"
              >{detail.session.conversation_title ?? author.name}</span
            >
          </span>
          <StoreAgentSessionMenu {controller} />
        </div>
        <div class="store-collaboration__detail-meta">
          <CommunityChip
            tone={detail.session.state === "active" ? "success" : "neutral"}
          >
            {detail.session.state}
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
        {#if detail.activity_truncated || detail.code_truncated}
          <p class="store-collaboration__coverage" role="status">
            This bounded view omits older {detail.activity_truncated
              ? "activity"
              : "code associations"}.
          </p>
        {/if}
      </header>{/if}
    {#if detail.activity.length}
      {#if showHeader && controller.execution && detail && controller.execution.metadata.details[metadataKey( { session: detail.session.session_id, repository: state.repository.id } )]}
        <StoreConversationMetadata
          manager={controller.execution}
          query={{
            session: detail.session.session_id,
            repository: state.repository.id,
          }}
        />
      {/if}
      <StoreAgentTranscript
        {controller}
        {afterItems}
        activity={detail.activity}
        sessionId={detail.session.session_id}
        integration={detail.session.integration}
        agent={author}
        member={{
          ...state.member,
          kind: state.member.kind === "agent" ? "agent" : undefined,
        }}
        patches={state.detail?.patches}
      />
    {:else if state.agentTranscript.length}
      <StoreAgentTranscript
        {controller}
        {afterItems}
        activity={[]}
        legacy={state.agentTranscript}
        sessionId={detail.session.session_id}
        integration={detail.session.integration}
        agent={author}
        member={state.member}
      />
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
      {@render afterItems?.()}
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
{/each}

<style>
  .store-agent-context[data-header="false"] {
    min-height: 0;
  }
</style>
