<script lang="ts">
  import "./navigation.css";
  import { onDestroy } from "svelte";
  import type { ChatBodyItem } from "./model.js";
  import type { ChatBodyComments } from "./comments.svelte.js";
  import { navigationTurns } from "./navigation-turns.js";
  import ChatTurnCard from "./ChatTurnCard.svelte";
  import ChatEndHint from "./ChatEndHint.svelte";
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
  let previewEnd = $state(false),
    atEnd = $state(false);
  let container = $state<HTMLElement>(),
    availableHeight = $state(500);
  const turns = $derived(navigationTurns(items, comments.threads));
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
    root?.querySelector<HTMLElement>(
      id === "pending-comments"
        ? "[data-chat-pending]"
        : `[data-chat-item="${CSS.escape(id)}"]`,
    );
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
      atEnd = node.scrollHeight - node.clientHeight - node.scrollTop <= 8;
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
            previewEnd
              ? ".chat-end-marker"
              : `[data-turn-id="${CSS.escape(preview?.id ?? "")}"]`,
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
    const keydown = (event: KeyboardEvent) => {
      if ((event.metaKey || event.ctrlKey) && event.key === "ArrowDown") {
        event.preventDefault();
        jumpToEnd();
      }
    };
    node.addEventListener("wheel", wheel, { passive: false });
    node.addEventListener("keydown", keydown);
    return () => {
      node.removeEventListener("wheel", wheel);
      node.removeEventListener("keydown", keydown);
    };
  });
  function navigate(id: string, first: string) {
    const turn = turns.find((entry) => entry.id === id);
    if (turn?.commentsOnly) {
      const entry = turn.comments[0];
      const thread = comments.threads.find(
        (entryThread) => entryThread.id === entry?.threadId,
      );
      if (thread && entry) void comments.goToMessage(thread, entry.comment);
      active = id;
      close();
      return;
    }
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
  function jumpToEnd() {
    if (!root) return;
    const draft = root.querySelector<HTMLElement>(
      "[data-chat-next-message] [role='textbox']",
    );
    (draft ?? target(items.at(-1)?.id ?? ""))?.focus({ preventScroll: true });
    close();
    root.scrollTo({
      top: root.scrollHeight,
      behavior: matchMedia("(prefers-reduced-motion: reduce)").matches
        ? "instant"
        : "smooth",
    });
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
        aria-label={turn.pending
          ? "Preview pending comments"
          : `Preview turn ${index + 1}`}
        aria-current={!atEnd && active === turn.id ? "location" : undefined}
        onpointerenter={() => {
          previewId = turn.id;
          previewEnd = false;
        }}
        onfocus={() => {
          previewId = turn.id;
          previewEnd = false;
        }}
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
    <button
      class="chat-turn-marker chat-end-marker"
      type="button"
      aria-label="Jump to end"
      aria-keyshortcuts="Meta+ArrowDown Control+ArrowDown"
      aria-current={atEnd ? "location" : undefined}
      onpointerenter={() => (previewEnd = true)}
      onfocus={() => (previewEnd = true)}
      onclick={jumpToEnd}><span aria-hidden="true"></span></button
    >
    {#if open && previewEnd}
      <ChatEndHint />
    {:else if open && preview}
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
