<script lang="ts">
  import type { Snippet } from "svelte";
  import { Composer, type ComposerStatus } from "@lapismd/design-core/ai/chat";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import TriangleAlertIcon from "@lucide/svelte/icons/triangle-alert";
  import ChatNextMessage from "../store-agent-chat-body/ChatNextMessage.svelte";
  import type { CommunityMemberModel } from "../../community/workspace-model.js";
  let {
    presentation,
    member,
    hideAvatar,
    dockActions,
    value,
    disabled,
    disabledReason,
    ariaLabel,
    status,
    hasContext,
    input: editorInput,
    drawer,
    actions,
    sendButton,
    onSubmit,
  }: {
    presentation: "document" | "classic";
    member: CommunityMemberModel;
    hideAvatar: boolean;
    dockActions: boolean;
    value: string;
    disabled: boolean;
    disabledReason: string;
    ariaLabel: string;
    status?: ComposerStatus;
    hasContext: boolean;
    input: Snippet;
    drawer: Snippet;
    actions: Snippet;
    sendButton: Snippet;
    onSubmit: () => void;
  } = $props();
</script>

{#if presentation === "document"}
  <div class="community-document-composer">
    {#if status}<Alert.Root
        variant={status.type === "error" ? "destructive" : "default"}
        role={status.type === "error" ? "alert" : "status"}
      >
        <TriangleAlertIcon aria-hidden="true" />
        <Alert.Description>{status.message ?? status.type}</Alert.Description>
      </Alert.Root>{/if}
    {@render drawer()}
    <ChatNextMessage
      {member}
      {hideAvatar}
      {disabled}
      input={editorInput}
      {onSubmit}
    />
    {#if !dockActions}<div class="community-document-composer__actions">
        {@render actions()}{@render sendButton()}
      </div>{/if}
  </div>
{:else}
  <Composer
    class="community-chat-composer__surface"
    {value}
    density="spacious"
    elevation="none"
    {disabled}
    interactiveDrawerWhenDisabled={Boolean(status || hasContext)}
    {status}
    statusPosition="top"
    aria-label={ariaLabel}
    title={disabled ? disabledReason : undefined}
    {onSubmit}
    {drawer}
    {sendButton}
    footerActions={actions}
  >
    {#snippet input()}<div class="community-chat-composer__editor-wrap">
        <ScrollArea
          type="auto"
          orientation="vertical"
          class="community-chat-composer__editor-scroll-area"
        >
          {@render editorInput()}
        </ScrollArea>
      </div>{/snippet}
  </Composer>
{/if}

<style>
  .community-document-composer {
    display: grid;
    gap: 0.5rem;
  }
  .community-document-composer__actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-inline-start: 2rem;
  }
  .community-document-composer :global(.community-composer-status-actions) {
    position: static;
    justify-self: end;
  }
  .community-document-composer :global(.community-chat-composer__mira.mira) {
    --mira-font-size: 1rem;
    --mira-line-height: 1.75;
    min-height: 1.75rem;
  }
  .community-document-composer
    :global(
      .community-chat-composer__mira
        :is(
          .mira__body,
          .mira__pane,
          .mira__editor-scroll,
          .mira-code-editor,
          .cm-editor
        )
    ) {
    min-height: 1.75rem;
  }
  .community-document-composer
    :global(.community-chat-composer__mira .cm-content) {
    min-height: max(16rem, 45vh);
  }
  .community-document-composer
    :global(.community-chat-composer__mira .cm-cursor) {
    border-inline-start-color: var(--primary);
    transform: scaleY(1.2);
    transform-origin: center;
  }
</style>
