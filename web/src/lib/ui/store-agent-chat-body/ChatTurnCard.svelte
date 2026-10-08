<script lang="ts">
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import type { ChatNavigationTurn } from "./navigation-turns.js";
  let {
    turn: preview,
    index: previewIndex,
    bookmarked,
    maxHeight,
    onNavigate,
    onBookmark,
  }: {
    turn: ChatNavigationTurn;
    index: number;
    bookmarked: boolean;
    maxHeight: number;
    onNavigate: () => void;
    onBookmark: () => void;
  } = $props();
  function dateLabel(time: number) {
    const days = Math.floor(Math.max(0, Date.now() - time) / 86400000);
    return `${new Intl.RelativeTimeFormat(undefined, { numeric: "auto" }).format(-days, "day")} (${new Intl.DateTimeFormat(undefined, { dateStyle: "medium", timeStyle: "short" }).format(time)})`;
  }
</script>

<section
  class="chat-turn-popover"
  role="group"
  aria-label="Turn summary"
  style:max-height={`${maxHeight}px`}
>
  <button
    class="chat-turn-content"
    type="button"
    aria-label={`Scroll to turn ${previewIndex + 1}`}
    onclick={onNavigate}
  >
    <span class="chat-turn-prompt">{preview.prompt}</span>
    <span class="chat-turn-summary-heading">Turn Summary</span>
    <span class="chat-turn-summary">{preview.summary}</span>
  </button>
  <div class="chat-turn-metadata">
    <div class="chat-turn-author">
      <CommunityMemberAvatar
        member={{
          id: `turn:${preview.author}`,
          name: preview.author,
          handle: "",
          initials: preview.author.slice(0, 2).toUpperCase(),
        }}
        size="inline"
        decorative
      />{preview.author}
    </div>
    {#if preview.agentName || preview.modelLabel}<div>
        ⌘ <span>{preview.modelLabel ?? preview.agentName}</span>
      </div>{/if}
    {#if preview.additions || preview.deletions}<div
        aria-label={`${preview.additions} captured lines added, ${preview.deletions} captured lines removed`}
      >
        ± <span class="chat-turn-additions"
          >+{preview.additions.toLocaleString()}</span
        ><span class="chat-turn-deletions"
          >−{preview.deletions.toLocaleString()}</span
        >
      </div>{/if}
    {#if preview.createdAt !== undefined}<div>
        ◷ <time datetime={new Date(preview.createdAt).toISOString()}
          >{dateLabel(preview.createdAt)}</time
        >
      </div>{/if}
  </div>
  <footer>
    <button
      type="button"
      aria-label={`Bookmark turn ${previewIndex + 1}`}
      aria-pressed={bookmarked}
      onclick={onBookmark}
      >{bookmarked ? "★" : "☆"} ⌘ / Ctrl + click to bookmark</button
    >
    <span>⌥ / Alt + scroll by marker</span>
  </footer>
</section>

<style>
  .chat-turn-popover {
    position: absolute;
    right: calc(100% - 2px);
    top: 50%;
    transform: translateY(-50%);
    width: min(22rem, 75vw);
    overflow: auto;
    padding: 0.8rem;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--muted);
    color: var(--foreground);
    box-shadow: 0 6px 22px #0002;
    font-size: 0.85rem;
  }
  .chat-turn-content {
    display: grid;
    gap: 0.65rem;
    border: 0;
    padding: 0;
    width: 100%;
    background: transparent;
    color: inherit;
    font: inherit;
    text-align: left;
    cursor: pointer;
  }
  .chat-turn-prompt {
    display: -webkit-box;
    -webkit-line-clamp: 2;
    line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
    line-height: 1.5;
  }
  .chat-turn-summary-heading {
    border-top: 1px solid var(--border);
    padding-top: 0.65rem;
    font-size: 0.75rem;
    color: var(--muted-foreground);
  }
  .chat-turn-summary {
    line-height: 1.6;
    overflow-wrap: anywhere;
  }
  .chat-turn-content:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: 2px;
  }
  .chat-turn-metadata {
    display: grid;
    gap: 0.65rem;
    margin-top: 0.8rem;
    padding-top: 0.75rem;
    border-top: 1px solid var(--border);
    color: var(--muted-foreground);
  }
  .chat-turn-metadata > div {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    line-height: 1.5;
  }
  .chat-turn-additions {
    color: #15803d;
  }
  .chat-turn-deletions {
    color: #b91c1c;
  }
  :global(.dark) .chat-turn-deletions {
    color: #fca5a5;
  }
  :global(.dark) .chat-turn-additions {
    color: #86efac;
  }
  footer {
    display: grid;
    gap: 0.5rem;
    margin-top: 0.75rem;
    padding-top: 0.65rem;
    border-top: 1px solid var(--border);
    color: var(--muted-foreground);
    font-size: 0.75rem;
  }
  footer button {
    border: 0;
    border-radius: var(--radius);
    padding: 0.25rem 0;
    background: transparent;
    color: inherit;
    text-align: left;
    cursor: pointer;
    font: inherit;
  }
  footer button:focus-visible {
    outline: 2px solid var(--ring);
    outline-offset: 2px;
  }
</style>
