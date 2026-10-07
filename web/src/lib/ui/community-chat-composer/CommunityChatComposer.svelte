<script lang="ts">
  import "@lapismd/mira/styles.css";
  import "./composer-0.css";
  import "./composer-1.css";
  import "../community-composer-status.css";
  import {
    composerSubmission,
    editLastComposerMessage,
  } from "./composer-submission.js";
  import { createComposerAgentAudience } from "./composer-agent-audience.svelte.js";
  import CommunityComposerStatusActions from "./CommunityComposerStatusActions.svelte";
  import CommunityComposerAgentAudience from "./CommunityComposerAgentAudience.svelte";
  import Mira from "@lapismd/mira";
  import type { MiraHandle, MiraMarkdownActionId } from "@lapismd/mira";
  import { Composer, type ComposerStatus } from "@lapismd/design-core/ai/chat";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import ArrowUpIcon from "@lucide/svelte/icons/arrow-up";
  import type { EditorView } from "@codemirror/view";
  import { onDestroy, untrack } from "svelte";
  import type { ComposerProps } from "./props.js";
  import { communityEmojiToken } from "./model.js";
  import {
    createCommunityComposerExtension,
    refreshCommunityComposerTokenDecorations,
  } from "./composer-extension.js";
  import type {
    CommunityComposerAutocompleteItem,
    CommunityComposerAutocompleteQuery,
  } from "./autocomplete.js";
  import {
    applyComposerAutocompleteSelection,
    insertComposerText,
  } from "./composer-autocomplete-apply.js";
  import { mergeComposerSlashCatalog } from "./composer-slash-catalog.js";
  import { createComposerAutocomplete } from "./composer-autocomplete.svelte.js";
  import { emptyComposerSlashSession } from "./composer-slash-session.js";
  import { useCommunityComposerSlashCatalog } from "./composer-slash-context.js";
  import CommunityComposerAutocompleteLayer from "./CommunityComposerAutocompleteLayer.svelte";
  import CommunityComposerEditContext from "./CommunityComposerEditContext.svelte";
  import CommunityReplyContext from "../community-reply-context/CommunityReplyContext.svelte";
  import CommunityComposerToolbar from "./CommunityComposerToolbar.svelte";

  let {
    value = $bindable(""),
    formattingOpen = $bindable(false),
    placeholder = "Write a message",
    members = [],
    audienceKey,
    initialAgentIds = [],
    rooms = [],
    communityEmojis = [],
    context,
    disabled = false,
    disabledReason = "Messaging is unavailable",
    composerStatus,
    busy = false,
    ariaLabel = "Message composer",
    submitLabel,
    mode = "chat",
    showSubmitButton = mode === "chat",
    clearOnSubmit = true,
    additionalActions,
    onSubmit,
    onCancel,
    onCancelContext,
    onEditLastMessage,
    onOpenMember,
    onMessageMember,
    onWaveMember,
    onRetryComposerStatus,
    onDismissComposerStatus,
    class: className = "",
    ref = $bindable(null),
  }: ComposerProps = $props();
  let mira = $state<MiraHandle>();
  let view = $state<EditorView>();
  let query = $state<CommunityComposerAutocompleteQuery>();
  let items = $state<CommunityComposerAutocompleteItem[]>([]);
  let selectedIndex = $state(0);
  let submitting = $state(false);
  let error = $state("");
  const catalogReader = useCommunityComposerSlashCatalog();
  const catalog = $derived(
    mergeComposerSlashCatalog(
      rooms,
      catalogReader?.catalog() ?? emptyComposerSlashSession(),
    ),
  );
  const status = $derived<ComposerStatus | undefined>(
    error ? { type: "error", message: error } : composerStatus,
  );
  const blocked = $derived(disabled || busy || submitting);
  const autocomplete = createComposerAutocomplete({
    props: () => ({ members, rooms, communityEmojis }),
    catalog: () => catalog,
    source: catalogReader,
    accept: (next, found) => {
      query = next;
      items = found;
      selectedIndex = 0;
    },
  });
  const clearAutocomplete = autocomplete.clear;
  const audience = createComposerAgentAudience({
    scope: () => audienceKey,
    initial: () => initialAgentIds,
    members: () => members,
    value: () => value,
    setValue: (next) => (value = next),
  });
  let active = true;
  onDestroy(() => {
    active = false;
    clearAutocomplete();
  });
  function selectAutocomplete(index: number) {
    if (!view || !query || !items[index]) return false;
    const selected = applyComposerAutocompleteSelection({
      item: items[index]!,
      query,
      view,
    });
    if (!selected?.keepOpen) clearAutocomplete();
    view.focus();
    return !!selected;
  }
  function insert(text: string) {
    if (!insertComposerText(view, text)) value += text;
  }
  function applyAction(action: MiraMarkdownActionId) {
    mira?.applyMarkdownAction(action);
    view?.focus();
  }
  async function submit() {
    if (blocked || !onSubmit || !value.trim()) return;
    submitting = true;
    error = "";
    const submitted = value;
    try {
      await onSubmit(composerSubmission(submitted, context));
      if (!active) return;
      if (clearOnSubmit && value === submitted)
        value = audience.restore(submitted);
      clearAutocomplete();
    } catch (cause) {
      if (!active) return;
      error =
        cause instanceof Error ? cause.message : "Message could not be sent";
    } finally {
      if (active) submitting = false;
    }
  }
  const extension = createCommunityComposerExtension({
    members: () => members,
    rooms: () => rooms,
    emojis: () => communityEmojis,
    preferredMemberIds: () => [],
    disabled: () => blocked,
    editing: () => context?.kind === "edit",
    autocompleteOpen: () => items.length > 0,
    onOpenMember: () => onOpenMember,
    onMessageMember: () => onMessageMember,
    onWaveMember: () => onWaveMember,
    onView: (v) => {
      view = v;
    },
    onSelection: () => {},
    onAutocompleteQuery: (q) => {
      void autocomplete.update(q);
    },
    onAutocompleteMove: (direction) => {
      selectedIndex = (selectedIndex + direction + items.length) % items.length;
    },
    onAutocompleteAccept: () => selectAutocomplete(selectedIndex),
    onAutocompleteDismiss: clearAutocomplete,
    submitOnEnter: () => mode === "chat" && showSubmitButton,
    onSubmit: () => void submit(),
    onEditLast: () => editLastComposerMessage(onEditLastMessage),
    onCancelEdit: () => onCancelContext?.(),
  });
  $effect(() => {
    members;
    rooms;
    communityEmojis;
    blocked;
    const v = view;
    untrack(() => {
      if (v) refreshCommunityComposerTokenDecorations(v);
    });
  });
</script>

<div
  bind:this={ref}
  class={`community-chat-composer ${className}`.trim()}
  data-ui-component="community-chat-composer"
  data-disabled={blocked ? "" : undefined}
  data-formatting-open={formattingOpen ? "" : undefined}
  role="group"
  aria-label={ariaLabel}
>
  {#if query && items.length}<CommunityComposerAutocompleteLayer
      {query}
      {items}
      {selectedIndex}
      onSelect={selectAutocomplete}
    />{/if}
  <Composer
    class="community-chat-composer__surface"
    {value}
    density="spacious"
    elevation="none"
    disabled={blocked}
    interactiveDrawerWhenDisabled={Boolean(status || context)}
    {status}
    statusPosition="top"
    aria-label={ariaLabel}
    title={disabled ? disabledReason : undefined}
    onSubmit={() => {
      if (showSubmitButton) void submit();
    }}
  >
    {#snippet drawer()}
      {#if audienceKey && members.some((m) => !!m.acpAgentId)}<CommunityComposerAgentAudience
          {audience}
          {members}
          disabled={blocked}
        />{/if}
      {#if context?.kind === "reply" || context?.kind === "transcript-reply"}<CommunityReplyContext
          target={context.target}
          onCancel={onCancelContext}
        />
      {:else if context?.kind === "edit"}<CommunityComposerEditContext
          label={context.label}
          onCancel={onCancelContext}
        />{/if}
      {#if status && (error || onRetryComposerStatus || onDismissComposerStatus)}<CommunityComposerStatusActions
          {error}
          onRetry={onRetryComposerStatus}
          onDismiss={onDismissComposerStatus}
          onRetryError={submit}
          onClearError={() => (error = "")}
        />{/if}
    {/snippet}
    {#snippet input()}<div class="community-chat-composer__editor-wrap">
        <ScrollArea
          type="auto"
          orientation="vertical"
          class="community-chat-composer__editor-scroll-area"
        >
          <Mira
            bind:this={mira}
            bind:value
            mode="live-preview"
            extensions={[extension]}
            readonly={blocked}
            {placeholder}
            toolbar={false}
            blockControls={false}
            outline={false}
            htmlPolicy="safe"
            class="community-chat-composer__mira"
          />
        </ScrollArea>
      </div>{/snippet}
    {#snippet footerActions()}
      <CommunityComposerToolbar
        bind:formattingOpen
        disabled={blocked}
        {communityEmojis}
        onAction={applyAction}
        onInsert={insert}
        onEmoji={(emoji, custom) => insert(communityEmojiToken(emoji, custom))}
      />
      {#if additionalActions}{@render additionalActions()}{/if}
    {/snippet}
    {#snippet sendButton()}{#if showSubmitButton}
        {#if onCancel}<Button
            type="button"
            variant="ghost"
            size="sm"
            disabled={busy}
            onclick={onCancel}>Cancel</Button
          >{/if}
        <Button
          type="button"
          size="icon-sm"
          class="community-chat-composer__send"
          data-ui-part="send-button"
          aria-label={submitLabel ??
            (context?.kind === "edit" ? "Save message" : "Send message")}
          disabled={blocked || !onSubmit || !value.trim()}
          onclick={submit}><ArrowUpIcon aria-hidden="true" /></Button
        >
      {/if}{/snippet}
  </Composer>
</div>
