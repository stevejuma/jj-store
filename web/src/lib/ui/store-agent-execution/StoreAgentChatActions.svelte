<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import MoreIcon from "@lucide/svelte/icons/ellipsis-vertical";
  import ActivityIcon from "@lucide/svelte/icons/activity";
  import StopIcon from "@lucide/svelte/icons/square";
  import PlayIcon from "@lucide/svelte/icons/play";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import MessagesIcon from "@lucide/svelte/icons/messages-square";
  import SettingsIcon from "@lucide/svelte/icons/settings-2";
  import RecoveryIcon from "@lucide/svelte/icons/circle-alert";
  import { requestRecoveryNotice } from "./request-recovery-notice.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import {
    object,
    type AgentSession,
  } from "../../controller/agent-execution-types.js";
  let {
    manager,
    session,
    agentId,
    onRuntime,
  }: {
    manager: AgentExecution;
    session?: AgentSession;
    agentId?: string;
    onRuntime: () => void;
  } = $props();
  const blocked = $derived(manager.busy || !!manager.pending);
  const recovery = $derived(requestRecoveryNotice(manager)?.action);
</script>

<DropdownMenu.Root>
  <DropdownMenu.Trigger>
    {#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        aria-label="Session options"
        title="Session options"><MoreIcon aria-hidden="true" /></Button
      >{/snippet}
  </DropdownMenu.Trigger>
  <DropdownMenu.Content
    align="end"
    style="width: min(18rem, calc(100vw - 2rem)); min-width: min(18rem, calc(100vw - 2rem));"
  >
    <DropdownMenu.Group>
      {#if recovery}<DropdownMenu.Item
          disabled={manager.busy}
          onSelect={recovery.run}
          ><RecoveryIcon
            aria-hidden="true"
          />{recovery.label}</DropdownMenu.Item
        >{/if}
      {#if session}<DropdownMenu.Item onSelect={onRuntime}
          ><SettingsIcon aria-hidden="true" />Conversation details</DropdownMenu.Item
        >{/if}
      <DropdownMenu.Item
        onSelect={() =>
          manager.viewActivity(session?.setup_id ?? agentId, session?.id)}
        ><ActivityIcon aria-hidden="true" />Activity log</DropdownMenu.Item
      >
      {#if session?.connected}<DropdownMenu.Item
          disabled={blocked}
          onSelect={() =>
            void manager.commandSession(session.id, { action: "end" })}
          ><StopIcon aria-hidden="true" />End session</DropdownMenu.Item
        >
      {:else if session && !["cleaned", "preparing", "opening"].includes(session.state)}
        {#if object(object(session.capabilities).initialize).loadSession === true}<DropdownMenu.Item
            disabled={blocked}
            onSelect={() =>
              void manager.commandSession(session.id, { action: "resume" })}
            ><PlayIcon aria-hidden="true" />Resume session</DropdownMenu.Item
          >{/if}
        <DropdownMenu.Item
          disabled={blocked}
          onSelect={() =>
            void manager.commandSession(session.id, {
              action: "continue_with_context",
            })}
          ><PlusIcon aria-hidden="true" />New session with saved context</DropdownMenu.Item
        >
      {:else if !session}<DropdownMenu.Item
          onSelect={() => manager.viewSessions(agentId)}
          ><MessagesIcon aria-hidden="true" />View sessions</DropdownMenu.Item
        >{/if}
    </DropdownMenu.Group>
  </DropdownMenu.Content>
</DropdownMenu.Root>
