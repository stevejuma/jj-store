<script lang="ts">
  import { onDestroy } from "svelte";
  import type { ChatBodyItem } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import { navigationTurns } from "./navigation-turns.js";
  import ChatTurnCard from "./ChatTurnCard.svelte";
  let {
    items,
    root,
    comments,
  }: {
    items: readonly ChatBodyItem[];
    root?: HTMLElement;
    comments: ChatBodyComments;
  } = $props();
  let active = $state(""),
    previewId = $state("");
  let hovered = $state(false),
    focused = $state(false);
  let container = $state<HTMLElement>(),
    availableHeight = $state(500);
  const turns = $derived(navigationTurns(items));
  const preview = $derived(
    turns.find((turn) => turn.id === previewId) ??
      turns.find((turn) => turn.id === active) ??
      turns[0],
  );
  const previewIndex = $derived(
    turns.findIndex((turn) => turn.id === preview?.id),
  );
  const open = $derived(hovered || focused);
  const target = (id: string) =>
    root?.querySelector<HTMLElement>(`[data-chat-item="${CSS.escape(id)}"]`);
  let pointerExit: ReturnType<typeof setTimeout> | undefined;
  function enter() {
    clearTimeout(pointerExit);
    hovered = true;
  }
  function leave() {
    clearTimeout(pointerExit);
    pointerExit = setTimeout(() => (hovered = false), 150);
  }
  onDestroy(() => clearTimeout(pointerExit));
  function close() {
    clearTimeout(pointerExit);
    hovered = false;
    focused = false;
  }
  function bookmark(id: string) {
    comments.bookmarkedTurns[id] = !comments.bookmarkedTurns[id];
  }
  $effect(() => {
    const node = root,
      sections = turns;
    if (!node) return;
    let frame = 0;
    const update = () => {
      availableHeight = node.clientHeight;
      const top = node.getBoundingClientRect().top;
      let id = sections[0]?.id ?? "";
      for (const turn of sections) {
        if (
          (target(turn.firstItemId)?.getBoundingClientRect().top ?? Infinity) -
            top <=
          80
        )
          id = turn.id;
        else break;
      }
      active = id;
    };
    const schedule = () => {
      cancelAnimationFrame(frame);
      frame = requestAnimationFrame(update);
    };
    const observer = new ResizeObserver(schedule);
    observer.observe(node);
    const content = node.querySelector("ol");
    if (content) observer.observe(content);
    node.addEventListener("scroll", schedule, { passive: true });
    schedule();
    return () => {
      cancelAnimationFrame(frame);
      observer.disconnect();
      node.removeEventListener("scroll", schedule);
    };
  });
  $effect(() => {
    const outside = (event: PointerEvent) => {
      if (!container?.contains(event.target as Node)) close();
    };
    const escape = (event: KeyboardEvent) => {
      if (
        event.key === "Escape" &&
        open &&
        container?.contains(document.activeElement)
      ) {
        event.preventDefault();
        event.stopPropagation();
        close();
        container
          .querySelector<HTMLButtonElement>(
            `[data-turn-id="${CSS.escape(preview?.id ?? "")}"]`,
          )
          ?.focus();
        focused = false;
      }
    };
    document.addEventListener("pointerdown", outside);
    document.addEventListener("keydown", escape, true);
    return () => {
      document.removeEventListener("pointerdown", outside);
      document.removeEventListener("keydown", escape, true);
    };
  });
  $effect(() => {
    const node = container,
      entries = turns;
    if (!node) return;
    const wheel = (event: WheelEvent) => {
      if (!event.altKey || !event.deltaY) return;
      event.preventDefault();
      const index = entries.findIndex((turn) => turn.id === active);
      const next =
        entries[
          Math.max(
            0,
            Math.min(entries.length - 1, index + Math.sign(event.deltaY)),
          )
        ];
      if (next) navigate(next.id, next.firstItemId);
    };
    node.addEventListener("wheel", wheel, { passive: false });
    return () => node.removeEventListener("wheel", wheel);
  });
  function navigate(id: string, first: string) {
    const node = target(first);
    if (!node || !root) return;
    root.scrollTo({
      top:
        root.scrollTop +
        node.getBoundingClientRect().top -
        root.getBoundingClientRect().top -
        16,
      behavior: matchMedia("(prefers-reduced-motion: reduce)").matches
        ? "instant"
        : "smooth",
    });
    node.focus({ preventScroll: true });
    active = id;
    close();
  }
</script>

{#if turns.length}
  <div
    class="chat-navigation"
    bind:this={container}
    role="group"
    aria-label="Turn navigation"
    onpointerenter={enter}
    onpointerleave={leave}
    onfocusin={() => (focused = true)}
    onfocusout={(event) => {
      if (!container?.contains(event.relatedTarget as Node)) focused = false;
    }}
  >
    {#each turns as turn, index (turn.id)}
      <button
        class="chat-turn-marker"
        type="button"
        data-turn-id={turn.id}
        aria-label={`Preview turn ${index + 1}`}
        aria-current={active === turn.id ? "location" : undefined}
        onpointerenter={() => (previewId = turn.id)}
        onfocus={() => (previewId = turn.id)}
        onclick={(event) => {
          if (event.metaKey || event.ctrlKey) bookmark(turn.id);
          else navigate(turn.id, turn.firstItemId);
        }}
      >
        <span
          aria-hidden="true"
          data-bookmarked={comments.bookmarkedTurns[turn.id]}
        ></span>
      </button>
    {/each}
    {#if open && preview}
      <ChatTurnCard
        turn={preview}
        index={previewIndex}
        bookmarked={comments.bookmarkedTurns[preview.id] ?? false}
        maxHeight={Math.max(160, availableHeight - 16)}
        onNavigate={() => navigate(preview.id, preview.firstItemId)}
        onBookmark={() => bookmark(preview.id)}
      />
    {/if}
  </div>
{/if}

<style>
  .chat-navigation {
    position: absolute;
    right: 0.3rem;
    top: 50%;
    transform: translateY(-50%);
    z-index: 5;
  }
  .chat-turn-marker {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    height: 10px;
    width: 28px;
    padding: 0 6px;
    border: 0;
    border-radius: 3px;
    background: transparent;
    cursor: pointer;
  }
  .chat-turn-marker span {
    height: 2px;
    width: 16px;
    background: var(--muted-foreground);
    opacity: 0.5;
    border-radius: 2px;
  }
  .chat-turn-marker[aria-current] span {
    background: var(--foreground);
    opacity: 1;
  }
  .chat-turn-marker span[data-bookmarked="true"] {
    background: var(--primary);
    opacity: 1;
  }
  .chat-turn-marker:hover,
  .chat-turn-marker:focus-visible {
    outline: 2px solid var(--ring);
  }
</style>
