<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import X from "@lucide/svelte/icons/x";
  import CommunityEmojiPicker from "../community-emoji-picker/CommunityEmojiPicker.svelte";
  import { createReactionLibrary } from "../../controller/reaction-library.svelte.js";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import type { StoreDataSource } from "../../controller/native-data-source.js";
  let {
    id,
    source,
    value,
    disabled = false,
    onChange,
  }: {
    id: string;
    source?: StoreDataSource;
    value?: string;
    disabled?: boolean;
    onChange(value: string | undefined): void;
  } = $props();
  const { context } = ownNotifications();
  const notice = $derived(`workflow-emojis-${id}`);
  const library = createReactionLibrary({
    source: () => source!.threads!,
    principal: () => "",
    generation: () => 0,
    valid: (_, reader) => reader === source?.threads,
  });
  const custom = $derived(
    value?.startsWith("custom:")
      ? library.definitions.find((d) => d.emoji_id === value!.slice(7))
      : undefined,
  );
  const label = $derived(
    !value
      ? "Any reaction"
      : value === "+"
        ? "Like"
        : value === "-"
          ? "Dislike"
          : custom
            ? `:${custom.shortcode}:`
            : value,
  );
  async function load() {
    const reader = source;
    if (!reader?.threads) return;
    try {
      await library.load();
      if (reader === source) context.service.resolve(context.scope, notice);
    } catch {
      if (reader === source)
        context.service.publish({
          scope: context.scope,
          key: notice,
          title: "Custom emoji could not be loaded",
          message:
            "Standard emoji remain available. Your reaction filter is retained.",
          severity: "warning",
          lifetime: "condition",
          action: { id: "retry-emoji", label: "Retry", run: () => void load() },
        });
    }
  }
  $effect(() => {
    const reader = source;
    untrack(() => {
      library.clear();
      if (reader) void load();
    });
    return () => {
      library.clear();
      context.service.resolve(context.scope, notice);
    };
  });
</script>

<div class="workflow-reaction-picker">
  <CommunityEmojiPicker
    contentClass="workflow-reaction-picker__menu"
    collisionPadding={8}
    label="Choose reaction"
    selection={value === "+"
      ? "👍"
      : value === "-"
        ? "👎"
        : custom
          ? ""
          : value?.startsWith("custom:")
            ? ""
            : value}
    {disabled}
    communityEmojis={library.emojis}
    presets={["+", "-", "✅", "❤️"]}
    onSelect={(emoji, custom) => {
      if (!custom) onChange(emoji);
      else {
        const definition = library.definitions.find(
          (d) =>
            d.shortcode === custom.shortcode && d.actor === custom.ownerPubkey,
        );
        if (definition) onChange(`custom:${definition.emoji_id}`);
      }
    }}
  />
  {#if !value || value === "+" || value === "-" || value.startsWith("custom:")}<span
      >{label}</span
    >{/if}
  {#if value}<Button
      variant="ghost"
      size="icon-sm"
      {disabled}
      aria-label="Match any reaction"
      title="Match any reaction"
      onclick={() => onChange(undefined)}><X aria-hidden="true" /></Button
    >{/if}
</div>

<style>
  .workflow-reaction-picker {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    min-width: 0;
    flex: 1;
  }
  .workflow-reaction-picker span {
    font-size: 0.8125rem;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .workflow-reaction-picker :global(button) {
    flex: none;
    border-radius: 999px;
    background: var(--background);
  }
  .workflow-reaction-picker :global(button:hover) {
    background: var(--muted);
  }
  :global(.workflow-reaction-picker__menu[data-ui-part="popover-content"]) {
    display: flex;
    flex-direction: column;
    max-height: min(
      calc(100dvh - 1rem),
      var(--bits-popover-content-available-height, 100dvh)
    );
  }
  :global(.workflow-reaction-picker__menu .community-emoji-picker__presets),
  :global(.workflow-reaction-picker__menu .community-emoji-picker__community) {
    flex: none;
  }
  :global(
    .workflow-reaction-picker__menu[data-ui-part="popover-content"]
      .community-emoji-picker__host
  ) {
    min-height: 0;
    flex: 1;
    overflow: hidden;
  }
  :global(
    .workflow-reaction-picker__menu[data-ui-part="popover-content"]
      .community-emoji-picker__host
      em-emoji-picker
  ) {
    height: min(
      27rem,
      calc(var(--bits-popover-content-available-height, 100dvh) - 6rem)
    );
  }
</style>
