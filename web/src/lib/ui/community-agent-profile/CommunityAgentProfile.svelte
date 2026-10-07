<script lang="ts">
  import { Reasoning } from "@lapismd/design-core/ai/experimental";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import MessageIcon from "@lucide/svelte/icons/message-square";
  import ActivityIcon from "@lucide/svelte/icons/wrench";
  import FingerprintIcon from "@lucide/svelte/icons/fingerprint";
  import UserIcon from "@lucide/svelte/icons/user-round";
  import CpuIcon from "@lucide/svelte/icons/cpu";
  import CopyIcon from "@lucide/svelte/icons/copy";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import PlayIcon from "@lucide/svelte/icons/play";
  import StopIcon from "@lucide/svelte/icons/circle-stop";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import CommunityProfileActions from "../community-profile-actions/CommunityProfileActions.svelte";
  import CommunityAgentRuntimeFields from "./CommunityAgentRuntimeFields.svelte";
  import { agentMember, agentAvailability } from "../community-agents/model.js";
  import type {
    AgentSetup,
    AgentStatus,
  } from "../../controller/agent-execution-types.js";
  import "./agent-profile.css";
  let {
    agent,
    status,
    unknown = false,
    pending = false,
    onEdit,
    onStart,
    onActivity,
    onSessions,
    onChat,
    onStop,
    onCopy,
  }: {
    agent: AgentSetup;
    status: AgentStatus | null;
    unknown?: boolean;
    pending?: boolean;
    onEdit: () => void;
    onStart: () => void;
    onActivity: () => void;
    onSessions: () => void;
    onChat?: () => void;
    onStop: (session: string) => void;
    onCopy: () => void;
  } = $props();
  let tab = $state("info");
  const presence = $derived(
    agentAvailability(agent.id, status?.sessions ?? [], unknown, status),
  );
  const connected = $derived(
    (status?.sessions ?? []).filter(
      (s) => s.setup_id === agent.id && s.connected,
    ),
  );
  const harness = $derived(status?.runners.find((r) => r.id === agent.runner));
</script>

<article
  tabindex="-1"
  class="agent-profile"
  aria-label={`${agent.name} agent profile`}
>
  <header>
    <CommunityMemberAvatar
      member={agentMember(agent)}
      size="large"
      shape="rounded"
      showPresence
      presence={presence.presence}
    />
    <h2>
      <Button
        type="button"
        variant="ghost"
        class="agent-profile__name-edit"
        aria-label={`Edit ${agent.name}`}
        disabled={pending || unknown}
        onclick={onEdit}
      >
        <span>{agent.name}</span><span
          class="agent-profile__name-edit-icon"
          aria-hidden="true"><PencilIcon /></span
        >
      </Button>
    </h2>
    <div class="agent-profile__signals">
      {#if presence.label === "Working"}<Reasoning
          label="Working"
          streaming
        />{:else}<CommunityChip
          tone={presence.label === "Online" ? "success" : "neutral"}
          >{presence.label}</CommunityChip
        >{/if}
    </div>
  </header>
  <CommunityProfileActions label="Agent profile actions" compact>
    {#if ["Starting", "Idle", "Online", "Working", "Queued", "Needs attention", "Stopping"].includes(presence.label)}<Button
        disabled={pending || unknown}
        onclick={() => onStop(agent.id)}
        ><StopIcon aria-hidden="true" />Stop agent</Button
      >
    {:else}<Button
        disabled={pending || unknown || !harness?.available}
        onclick={onStart}><PlayIcon aria-hidden="true" />Start agent</Button
      >{/if}
    {#if onChat}<Button variant="secondary" disabled={pending} onclick={onChat}
        >New chat</Button
      >{/if}
    <Button variant="secondary" disabled={pending} onclick={onSessions}
      ><MessageIcon aria-hidden="true" />View chats</Button
    >
    {#if connected.length > 1}<Button variant="secondary" onclick={onSessions}
        >Manage running sessions</Button
      >{/if}
  </CommunityProfileActions>
  <Tabs.Root bind:value={tab}>
    <Tabs.List class="agent-profile__tabs"
      ><Tabs.Trigger value="info">Info</Tabs.Trigger><Tabs.Trigger
        value="runtime">Runtime</Tabs.Trigger
      ></Tabs.List
    >
    <Tabs.Content value="info">
      <div class="agent-profile__info">
        <Button
          class="agent-profile__activity-link"
          variant="outline"
          aria-label="Activity log"
          onclick={onActivity}
        >
          <ActivityIcon aria-hidden="true" /><span>Activity log</span><span
            class="agent-profile__view">View</span
          ><ChevronRightIcon aria-hidden="true" />
        </Button>
        <ul class="agent-profile__info-list" aria-label="Agent information">
          {#if agent.description}<li class="agent-profile__info-row">
              <MessageIcon aria-hidden="true" />
              <div class="agent-profile__info-value">
                <strong>About</strong>
                <p>{agent.description}</p>
              </div>
            </li>{/if}
          <li>
            <Button
              class="agent-profile__info-row agent-profile__info-link"
              variant="ghost"
              aria-label={`Copy ${agent.name}'s agent ID`}
              onclick={onCopy}
            >
              <FingerprintIcon aria-hidden="true" /><span
                class="agent-profile__info-value"
                ><strong>Agent ID</strong><span class="agent-profile__key"
                  >{agent.id}</span
                ></span
              ><CopyIcon aria-hidden="true" />
            </Button>
          </li>
          <li class="agent-profile__info-row">
            <UserIcon aria-hidden="true" /><span
              class="agent-profile__info-value"
              ><strong>Managed by</strong><span>This installation</span></span
            >
          </li>
          <li class="agent-profile__info-row">
            <CpuIcon aria-hidden="true" />
            <div class="agent-profile__info-value">
              <strong>Agent harness</strong><span
                >{harness?.label ?? agent.runner} · {harness?.available
                  ? "Installed"
                  : "Unavailable"}</span
              >
            </div>
          </li>
          {#if agent.instructions}<li class="agent-profile__info-row">
              <MessageIcon aria-hidden="true" />
              <div class="agent-profile__info-value">
                <strong>Instructions</strong>
                <p>{agent.instructions}</p>
              </div>
            </li>{/if}
        </ul>
      </div>
    </Tabs.Content>
    <Tabs.Content value="runtime"
      ><div class="agent-profile__runtime">
        <CommunityAgentRuntimeFields
          harness={harness?.label ?? agent.runner}
          model={agent.preferences.model || "Harness default"}
          thinking={agent.preferences.effort || "Harness default"}
          mode={agent.preferences.mode || "Choose after connecting"}
          disabled={pending || unknown}
          {onEdit}
        />
        <p>
          These preferences apply to future launches. Existing sessions retain
          their captured configuration.
        </p>
      </div></Tabs.Content
    >
  </Tabs.Root>
</article>
