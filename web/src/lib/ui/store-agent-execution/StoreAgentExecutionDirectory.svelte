<script lang="ts">
  import { onMount } from "svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import { agentMember } from "../community-agents/model.js";
  import { repositoryExecutionSessions } from "./repository-sessions.js";
  import type { Model } from "../../api/client.js";
  import "../store-collaboration/store-collaboration.css";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentLaunchContext } from "../../controller/agent-execution-types.js";
  let {
    manager,
    context = {},
    recorded = [],
    query = "",
  }: {
    manager?: AgentExecution;
    context?: AgentLaunchContext;
    recorded?: readonly Model<"AgentSessionSummary">[];
    query?: string;
  } = $props();
  const sessions = $derived(
    context.repository
      ? repositoryExecutionSessions(
          manager?.value,
          context.repository,
          recorded,
          query,
        )
      : [],
  );
  onMount(() => {
    void manager?.ensure();
  });
</script>

{#if manager?.available() && sessions.length}<div
    role="list"
    aria-label="Interactive agent sessions"
    class="store-agent-session-list"
  >
    {#each sessions as session (session.id)}
      {@const title = session.conversation_title ?? "New conversation"}
      {@const lastActivity = Number(session.last_activity_at_millis ?? 0)}
      {@const member = {
        ...agentMember({
          id: session.id,
          name: session.title,
          avatar: session.avatar,
        }),
        acpAgentId: session.setup_id ?? undefined,
      }}
      <div
        role="listitem"
        class="store-agent-session-item"
        data-session-id={session.id}
      >
        <div class="store-collaboration__row" data-kind="agent" data-collection>
          <button
            type="button"
            class="store-collaboration__row-select"
            aria-label={`Open ${title} ${session.state} interactive agent session`}
            onclick={() => manager.openSession(session.id)}
          ></button>
          <span class="store-collaboration__row-leading"
            ><CommunityMemberAvatar
              {member}
              size="small"
              showPresence
              decorative
            /></span
          >
          <span class="store-collaboration__row-main">
            <span class="store-collaboration__row-title-row"
              ><span class="store-collaboration__row-title">{title}</span></span
            >
            <span class="store-collaboration__row-meta">
              {#if lastActivity > 0 && lastActivity <= 8_640_000_000_000_000 && Number.isSafeInteger(lastActivity)}<CommunityRelativeTime
                  timestamp={lastActivity / 1000}
                />{/if}
              <span>{session.runner} · {session.state}</span>
            </span>
          </span>
        </div>
      </div>
    {/each}
  </div>{/if}
