<script lang="ts">
  import "./composer.css";
  import {
    composerSubmission,
    composerFeedback,
    composerSubmissionError,
    editLastComposerMessage,
  } from "./composer-submission.js";
  import { createComposerAgentAudience } from "./composer-agent-audience.svelte.js";
  import CommunityComposerStatusActions from "./CommunityComposerStatusActions.svelte";
  import Mira from "@lapismd/mira";
  import type { MiraHandle, MiraMarkdownActionId } from "@lapismd/mira";
  import CommunityComposerSendActions from "./CommunityComposerSendActions.svelte";
  import CommunityComposerSurface from "./CommunityComposerSurface.svelte";
  import { createComposerSelection } from "./composer-selection.js";
  import { useDocumentComposerActions } from "./composer-document-actions.svelte.js";
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
    presentation = "classic",
    member = { id: "user", name: "You", handle: "", initials: "YO" },
    hideAvatar = false,
    dockActions = false,
    documentActions = $bindable(),
    restoreDraft = $bindable(),
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
    allowEmpty = false,
    enterInsertsNewline = false,
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
  const selection = createComposerSelection((v) => (view = v));
  let error = $state("");
  const catalogReader = useCommunityComposerSlashCatalog();
  const catalog = $derived(
    mergeComposerSlashCatalog(
      rooms,
      catalogReader?.catalog() ?? emptyComposerSlashSession(),
    ),
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
  const audience = createComposerAgentAudience({
    scope: () => audienceKey,
    initial: () => initialAgentIds,
    members: () => members,
    value: () => value,
    setValue: (next) => (value = next),
  });
  const issue = $derived(error || audience.error);
  const status = $derived(composerFeedback(issue, composerStatus));
  useDocumentComposerActions({
    restore: () => audience.restore,
    getDraft: () => restoreDraft,
    setDraft: (next) => (restoreDraft = next),
    active: () => dockActions && presentation === "document",
    actions: () => actions,
    get: () => documentActions,
    set: (next) => (documentActions = next),
  });
  let active = true;
  onDestroy(() => {
    active = false;
    autocomplete.clear();
  });
  function selectAutocomplete(index: number) {
    if (!view || !query || !items[index]) return false;
    const selected = applyComposerAutocompleteSelection({
      item: items[index]!,
      query,
      view,
    });
    if (!selected?.keepOpen) autocomplete.clear();
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
    if (blocked || !onSubmit || (!allowEmpty && !value.trim())) return;
    submitting = true;
    error = "";
    const submitted = value;
    try {
      await onSubmit(composerSubmission(submitted, context));
      if (!active) return;
      if (clearOnSubmit && value === submitted)
        value = audience.restore(submitted);
      autocomplete.clear();
    } catch (cause) {
      if (!active) return;
      error = composerSubmissionError(cause);
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
    ...selection,
    onAutocompleteQuery: (q) => {
      void autocomplete.update(q);
    },
    onAutocompleteMove: (direction) => {
      selectedIndex = (selectedIndex + direction + items.length) % items.length;
    },
    onAutocompleteAccept: () => selectAutocomplete(selectedIndex),
    onAutocompleteDismiss: autocomplete.clear,
    submitOnEnter: () =>
      presentation !== "document" &&
      !enterInsertsNewline &&
      mode === "chat" &&
      showSubmitButton,
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

{#snippet actions()}
  <CommunityComposerToolbar
    bind:formattingOpen
    disabled={blocked}
    mentionMember={audience.mentionButtonMember}
    onDismissMention={audience.dismissMember}
    {communityEmojis}
    onAction={applyAction}
    onInsert={insert}
    onEmoji={(emoji, custom) => insert(communityEmojiToken(emoji, custom))}
  />
  {#if additionalActions}{@render additionalActions()}{/if}
{/snippet}

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
      disabled={blocked}
      autoMentionDisabled={audience.busy}
      showAutoMention={query.kind === "member" && audience.available}
      autoMentionEnabled={audience.enabled}
      lockedAgentPubkeys={audience.lockedMemberIds}
      onAutoMentionChange={(enabled) => audience.toggle(enabled)}
      onToggleAlwaysAddress={audience.available
        ? audience.toggleMember
        : undefined}
      onSelect={selectAutocomplete}
    />{/if}
  <CommunityComposerSurface
    {actions}
    {dockActions}
    {presentation}
    {member}
    {hideAvatar}
    {value}
    disabled={blocked}
    hasContext={!!context}
    {status}
    {ariaLabel}
    {disabledReason}
    onSubmit={() => void submit()}
  >
    {#snippet drawer()}
      {#if context?.kind === "reply" || context?.kind === "transcript-reply"}<CommunityReplyContext
          target={context.target}
          onCancel={onCancelContext}
        />
      {:else if context?.kind === "edit"}<CommunityComposerEditContext
          label={context.label}
          onCancel={onCancelContext}
        />{/if}
      {#if status && (issue || onRetryComposerStatus || onDismissComposerStatus)}<CommunityComposerStatusActions
          error={issue}
          onRetry={onRetryComposerStatus}
          onDismiss={onDismissComposerStatus}
          onRetryError={error ? submit : audience.retry}
          onClearError={() => (error ? (error = "") : audience.clearError())}
        />{/if}
    {/snippet}
    {#snippet input()}
      <Mira
        bind:this={mira}
        bind:value
        mode="live-preview"
        extensions={[extension]}
        readonly={blocked}
        placeholder={presentation === "document" ? "" : placeholder}
        toolbar={false}
        blockControls={false}
        outline={false}
        htmlPolicy="safe"
        class="community-chat-composer__mira"
      />
    {/snippet}
    {#snippet sendButton()}<CommunityComposerSendActions
        show={showSubmitButton}
        disabled={blocked || !onSubmit || (!allowEmpty && !value.trim())}
        {busy}
        {onCancel}
        label={submitLabel}
        editing={context?.kind === "edit"}
        onSend={submit}
      />{/snippet}
  </CommunityComposerSurface>
</div>
