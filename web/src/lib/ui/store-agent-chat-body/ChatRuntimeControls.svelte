<script lang="ts">
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import BotIcon from "@lucide/svelte/icons/bot";
  import LightbulbIcon from "@lucide/svelte/icons/lightbulb";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import type { CommunityMemberModel } from "../community-workspace/model.js";
  type Choice = { value: string; label: string };
  let {
    agents,
    agent,
    models,
    model,
    thinkingOptions,
    thinking,
    disabled = false,
    onAgentChange,
    onModelChange,
    onThinkingChange,
  }: {
    agents: readonly CommunityMemberModel[];
    agent: CommunityMemberModel;
    models: readonly Choice[];
    model: string;
    thinkingOptions: readonly Choice[];
    thinking: string;
    disabled?: boolean;
    onAgentChange: (value: string) => void;
    onModelChange: (value: string) => void;
    onThinkingChange: (value: string) => void;
  } = $props();
  const controls = $derived([
    {
      label: "Agent",
      value: agent.id,
      options: agents.map((member) => ({
        value: member.id,
        label: member.name,
      })),
      onChange: onAgentChange,
    },
    {
      label: "Thinking",
      value: thinking,
      options: thinkingOptions,
      onChange: onThinkingChange,
    },
    { label: "Model", value: model, options: models, onChange: onModelChange },
  ]);
</script>

<div
  class="chat-runtime-controls"
  role="group"
  aria-label="Next message runtime"
>
  {#each controls as control (control.label)}
    {@const selected =
      control.options.find((option) => option.value === control.value)?.label ??
      control.value}
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        {#snippet child({ props })}
          <Button
            {...props}
            variant="ghost"
            size={control.label === "Agent" ? "icon-sm" : "sm"}
            {disabled}
            aria-label={`${control.label}: ${selected}`}
            title={`${control.label}: ${selected}`}
          >
            {#if control.label === "Agent"}<BotIcon aria-hidden="true" />
            {:else if control.label === "Thinking"}<LightbulbIcon
                data-icon="inline-start"
                aria-hidden="true"
              />
            {:else}<CommunityMemberAvatar
                member={agent}
                size="inline"
                decorative
              />{/if}
            {#if control.label !== "Agent"}<span class="chat-runtime-label"
                >{selected}</span
              >{/if}
          </Button>
        {/snippet}
      </DropdownMenu.Trigger>
      <DropdownMenu.Content
        align="end"
        side="top"
        preventScroll={false}
        collisionPadding={8}
        style={`width: min(${control.label === "Thinking" ? "14rem" : "18rem"}, calc(100vw - 2rem)); min-width: 0;`}
      >
        <DropdownMenu.Group>
          <DropdownMenu.Label>{control.label}</DropdownMenu.Label>
          <DropdownMenu.RadioGroup
            value={control.value}
            onValueChange={control.onChange}
            aria-label={control.label}
          >
            {#each control.options as option (option.value)}
              <DropdownMenu.RadioItem value={option.value}>
                {#if control.label === "Agent"}
                  {@const member = agents.find(
                    (member) => member.id === option.value,
                  )!}
                  <CommunityMemberAvatar {member} size="inline" decorative />
                {/if}
                {option.label}
              </DropdownMenu.RadioItem>
            {/each}
          </DropdownMenu.RadioGroup>
        </DropdownMenu.Group>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  {/each}
</div>

<style>
  .chat-runtime-controls {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    min-width: 0;
  }
  @container chat-footer (max-width: 24rem) {
    .chat-runtime-label {
      display: none;
    }
  }
</style>
