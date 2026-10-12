<script lang="ts">
  // Community agent gallery; local execution data and actions replace relay plumbing.
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import SessionsIcon from "@lucide/svelte/icons/messages-square";
  import MoreIcon from "@lucide/svelte/icons/ellipsis-vertical";
  import BotIcon from "@lucide/svelte/icons/bot";
  import ShieldIcon from "@lucide/svelte/icons/shield-user";
  import StoreContentEmpty from "../store-application/StoreContentEmpty.svelte";
  import CommunitySkeleton from "../community-skeleton/CommunitySkeleton.svelte";
  import CommunityAgentProcessingControl from "../community-agent-processing-control/CommunityAgentProcessingControl.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import type {
    AgentSetup,
    AgentStatus,
  } from "../../controller/agent-execution-types.js";
  import { agentAvailability, agentMember } from "./model.js";
  import "./agents.css";
  let {
    value,
    unknown = false,
    available = true,
    management = true,
    busy = false,
    onAdd,
    onEdit,
    onOpen,
    onStart,
    onStop,
    onSessions,
    onChat,
    onDelete,
    onRefresh,
    onManage,
  }: {
    value: AgentStatus | null;
    unknown?: boolean;
    available?: boolean;
    management?: boolean;
    busy?: boolean;
    onAdd(): void;
    onEdit(agent: AgentSetup): void;
    onOpen(agent: AgentSetup): void;
    onStart(id: string): void;
    onStop?(session: string): void;
    onSessions(id?: string): void;
    onChat?(id: string): void;
    onDelete(agent: AgentSetup): void;
    onRefresh(): void;
    onManage?: () => void;
  } = $props();
</script>

{#snippet sessionsCard()}
  <button
    class="agents__add-card agents__sessions-card"
    type="button"
    onclick={() => onSessions()}
    ><span class="agents__sessions-icon"
      ><SessionsIcon aria-hidden="true" /></span
    ><span>View sessions</span></button
  >
{/snippet}

<section class="agents" aria-label="Your agents">
  <header>
    <div>
      <h1>Agents</h1>
      <p>
        {management
          ? "Your agents on this device."
          : "Agents available on this server."}
      </p>
    </div>
    <div class="agents__header-actions">
      {#if onManage}<Button variant="outline" onclick={onManage}
          ><ShieldIcon aria-hidden="true" />Manage server access</Button
        >{/if}
      <Tooltip.Provider>
        <Tooltip.Root>
          <Tooltip.Trigger
            >{#snippet child({ props })}
              <Button
                {...props}
                variant="ghost"
                size="icon-sm"
                aria-label="Refresh agents"
                onclick={onRefresh}
                disabled={busy || !available}
              >
                <RefreshIcon aria-hidden="true" />
              </Button>
            {/snippet}</Tooltip.Trigger
          >
          <Tooltip.Content role="tooltip">Refresh agents</Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
    </div>
  </header>
  {#if !available}<div class="agents__grid">{@render sessionsCard()}</div>
    <StoreContentEmpty
      Icon={BotIcon}
      title="Agent execution unavailable"
      description={management
        ? "Local agent management requires an enabled local execution service on this device."
        : "Remote execution requires enabled server agents and a sharing grant for your principal."}
    />
  {:else if !value && !unknown}<div class="agents__grid">
      {@render sessionsCard()}{#if management}{@render addCard()}{/if}<CommunitySkeleton
        layout="gallery"
        galleryVariant="agents"
        inlineGallery
        label="Loading your agents"
      />
    </div>
  {:else}
    {#if unknown}<p role="status">
        Agent availability is unknown. Saved agents remain visible while
        reconnecting.
      </p>{/if}
    <div class="agents__grid">
      {@render sessionsCard()}
      {#if management}{@render addCard()}{/if}
      {#each value?.setups ?? [] as agent (agent.id)}
        {@const status = agentAvailability(
          agent.id,
          value?.sessions ?? [],
          unknown,
          value,
        )}
        {@const live = [
          "Starting",
          "Idle",
          "Queued",
          "Working",
          "Needs attention",
          "Stopping",
          "Online",
        ].includes(status.label)}
        <div class="agents__card" data-agent-id={agent.id}>
          <button
            class="agents__card-open"
            aria-label={`View ${agent.name}`}
            onclick={() => onOpen(agent)}
          ></button>
          <div class="agents__avatar" title={status.label}>
            <div class="agents__avatar-content">
              <CommunityMemberAvatar
                member={agentMember(agent)}
                size="xlarge"
                shape="rounded"
                showPresence
                presence={status.presence}
              />
              {#if management && (status.label === "Offline" || (live && onStop && !unknown))}<div
                  class="agents__processing"
                >
                  <CommunityAgentProcessingControl
                    label={live ? "Stop" : "Start"}
                    accessibleLabel={`${live ? "Stop" : "Start"} ${agent.name}`}
                    disabled={busy ||
                      unknown ||
                      value?.runtime !== "ready" ||
                      (!live &&
                        !value?.runners.find((r) => r.id === agent.runner)
                          ?.available)}
                    onChange={() =>
                      live ? onStop?.(agent.id) : onStart(agent.id)}
                  />
                </div>{/if}
            </div>
          </div>
          <div class="agents__details">
            <strong>{agent.name}</strong>
            <span class="agents__about"
              >{agent.preferences.model || "Harness default model"}</span
            >
          </div>
          <DropdownMenu.Root
            ><DropdownMenu.Trigger
              >{#snippet child({ props })}<Button
                  {...props}
                  class="agents__card-menu"
                  variant="ghost"
                  size="icon-sm"
                  aria-label={`Actions for ${agent.name}`}
                  ><MoreIcon aria-hidden="true" /></Button
                >{/snippet}</DropdownMenu.Trigger
            >
            <DropdownMenu.Content class="agents__card-actions" align="end"
              ><DropdownMenu.Group>
                {#if management}<DropdownMenu.Item
                    disabled={busy ||
                      unknown ||
                      value?.runtime !== "ready" ||
                      !value?.runners.find((r) => r.id === agent.runner)
                        ?.available}
                    onSelect={() => onStart(agent.id)}
                    >Start agent</DropdownMenu.Item
                  >{/if}
                {#if onChat}<DropdownMenu.Item
                    onSelect={() => onChat?.(agent.id)}
                    >New chat</DropdownMenu.Item
                  >{/if}
                <DropdownMenu.Item onSelect={() => onSessions(agent.id)}
                  >View chats</DropdownMenu.Item
                >
                {#if management}<DropdownMenu.Item
                    onSelect={() => onEdit(agent)}>Edit agent</DropdownMenu.Item
                  >
                  <DropdownMenu.Item
                    variant="destructive"
                    disabled={busy || unknown}
                    onSelect={() => onDelete(agent)}
                    >Delete agent</DropdownMenu.Item
                  >{/if}
              </DropdownMenu.Group></DropdownMenu.Content
            >
          </DropdownMenu.Root>
        </div>
      {/each}
    </div>
    {#if management && value && !value.setups.length && !unknown}<p>
        No agents yet. Add an agent to save its instructions and runtime
        preferences.
      </p>{/if}
  {/if}
</section>

{#snippet addCard()}<button
    class="agents__add-card"
    type="button"
    aria-label="Add agent"
    disabled={busy || !value || unknown}
    onclick={onAdd}
    ><PlusIcon aria-hidden="true" /><span>Add agent</span></button
  >{/snippet}
