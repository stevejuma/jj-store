<script lang="ts">
  import type { Model } from "../../api/client.js";
  import type { AgentAssociation } from "../../controller/repository-read-types.js";
  import {
    agentMember,
    agentSessionTiming,
  } from "../../collaboration/presentation.js";
  import MessagesSquareIcon from "@lucide/svelte/icons/messages-square";
  import WrenchIcon from "@lucide/svelte/icons/wrench";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import StoreRecordMenu from "./StoreRecordMenu.svelte";
  import "./store-collaboration.css";
  let {
    session,
    selectedId = null,
    association,
    collection = false,
    onSelect,
  }: {
    session: Model<"AgentSessionSummary">;
    selectedId?: string | null;
    association?: AgentAssociation | null;
    collection?: boolean;
    onSelect: (id: string) => void;
  } = $props();
  const agent = $derived(agentMember(session)),
    timing = $derived(agentSessionTiming(session));
</script>

<div role="listitem" class="store-agent-session-item">
  <div
    class="store-collaboration__row"
    data-kind="agent"
    data-collection={collection ? "" : undefined}
    data-selected={selectedId === session.session_id ? "true" : undefined}
  >
    <button
      type="button"
      class="store-collaboration__row-select"
      aria-label={`Open ${agent.name} ${session.state} agent session`}
      aria-pressed={selectedId === session.session_id}
      onclick={() => onSelect(session.session_id)}
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
      <span class="store-collaboration__row-title">{agent.name}</span>
      <span class="store-collaboration__row-meta"
        ><CommunityChip
          size="compact"
          tone={session.state === "active" ? "success" : "neutral"}
          >{session.state}</CommunityChip
        >{#if timing}<span class="store-collaboration__agent-start"
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
            ><WrenchIcon aria-hidden="true" />{session.tool_count}</span
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

<style>
  .store-agent-session-item {
    padding-block: 2px;
  }
</style>
