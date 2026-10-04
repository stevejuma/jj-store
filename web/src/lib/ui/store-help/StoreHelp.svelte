<script lang="ts">
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  const desktop = isMacDesktopWindow();
  import { Button } from "@lapismd/design-core/shadcn/button";
  import BookOpenIcon from "@lucide/svelte/icons/book-open";
  import XIcon from "@lucide/svelte/icons/x";
  import { helpSession, admittedHelpMessage } from "./help-session.js";
  let {
    compact = false,
    sidebar = false,
  }: { compact?: boolean; sidebar?: boolean } = $props();
  let dialog = $state<HTMLDialogElement>();
  let frame = $state<HTMLIFrameElement>();
  let opened = $state(false);
  let mounted = $state(false);
  let ready = $state(false);
  let failed = $state(false);
  let source = $state(helpSession.path);
  let opener: HTMLElement | null = null;
  let timeout: ReturnType<typeof setTimeout> | undefined;
  function arm() {
    clearTimeout(timeout);
    failed = false;
    timeout = setTimeout(() => {
      if (!ready) failed = true;
    }, 15000);
  }
  function open(event: MouseEvent) {
    opener =
      event.currentTarget instanceof HTMLElement ? event.currentTarget : null;
    mounted = true;
    opened = true;
    dialog?.showModal();
    arm();
  }
  function close() {
    if (frame?.contentWindow) {
      try {
        const location = frame.contentWindow.location;
        if (
          location.origin === window.location.origin &&
          location.pathname.startsWith("/help/")
        ) {
          helpSession.path =
            location.pathname + location.search + location.hash;
          helpSession.scrollY = frame.contentWindow.scrollY;
        }
      } catch {
        /* An external navigation must not become a retained Help URL. */
      }
    }
    clearTimeout(timeout);
    opened = false;
    dialog?.close();
    opener?.focus();
  }
  function message(event: MessageEvent) {
    if (
      !admittedHelpMessage(
        event,
        frame?.contentWindow ?? null,
        window.location.origin,
      )
    )
      return;
    if (event.data.type === "jj-store-help-focus-close") {
      if (opened)
        dialog
          ?.querySelector<HTMLButtonElement>('[aria-label="Close Help"]')
          ?.focus();
      return;
    }
    if (event.data.type === "jj-store-help-close") {
      if (opened) close();
    } else {
      ready = true;
      failed = false;
      clearTimeout(timeout);
      if (helpSession.path === source && helpSession.scrollY) {
        frame?.contentWindow?.scrollTo(0, helpSession.scrollY);
        helpSession.scrollY = 0;
      }
    }
  }
  function focusBoundary(event: KeyboardEvent) {
    if (event.key !== "Tab" || !event.shiftKey || !ready || failed) return;
    const closeButton = dialog?.querySelector('[aria-label="Close Help"]');
    if (event.target !== closeButton) return;
    const document = frame?.contentDocument;
    const items = document
      ? [
          ...document.querySelectorAll<HTMLElement>(
            "a[href], button, input, select, textarea, [tabindex]",
          ),
        ].filter(
          (element) =>
            element.tabIndex >= 0 &&
            !element.hasAttribute("disabled") &&
            element.getClientRects().length > 0 &&
            getComputedStyle(element).visibility !== "hidden",
        )
      : [];
    event.preventDefault();
    items.at(-1)?.focus();
  }
  function retry() {
    ready = false;
    source = "/help/";
    frame?.contentWindow?.location.replace(source);
    arm();
  }
  $effect(() => () => clearTimeout(timeout));
</script>

<svelte:window onmessage={message} />
<div
  class="store-help-entry"
  data-sidebar={sidebar || undefined}
  data-compact={compact || undefined}
>
  <Button
    variant="ghost"
    size={compact ? "icon-sm" : "default"}
    class="store-help-trigger"
    data-compact={compact || undefined}
    aria-label="Help"
    title={compact ? "Help" : undefined}
    onclick={open}
  >
    <BookOpenIcon aria-hidden="true" />{#if !compact}<span>Help</span>{/if}
  </Button>
</div>
<dialog
  bind:this={dialog}
  class="store-help"
  aria-labelledby="store-help-title"
  onkeydown={focusBoundary}
  oncancel={(event) => {
    event.preventDefault();
    close();
  }}
>
  <header class:desktop data-tauri-drag-region={desktop ? "deep" : undefined}>
    <h1 id="store-help-title">Help</h1>
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label="Close Help"
      onclick={close}><XIcon aria-hidden="true" /></Button
    >
  </header>
  {#if !ready && !failed}<p role="status">Loading documentation…</p>{/if}
  {#if failed}<div role="alert">
      <p>Documentation could not be loaded from this application build.</p>
      <Button variant="outline" onclick={retry}>Retry</Button>
    </div>{/if}
  {#if mounted}<iframe
      bind:this={frame}
      src={source}
      title="jj-store documentation"
      class:ready
      referrerpolicy="no-referrer"
    ></iframe>{/if}
</dialog>

<style>
  .store-help-entry[data-sidebar] {
    padding-inline: 0.75rem;
  }
  .store-help-entry[data-compact] {
    display: flex;
    justify-content: center;
    padding-inline: 0.5rem;
  }
  :global(.store-help-trigger) {
    width: 100%;
    justify-content: flex-start;
  }
  :global(.store-help-trigger[data-compact]) {
    width: 2rem;
    justify-content: center;
  }
  .store-help {
    position: fixed;
    inset: 0;
    margin: 0;
    padding: 0;
    width: 100%;
    max-width: none;
    height: 100dvh;
    max-height: none;
    border: 0;
    color: var(--foreground);
    background: var(--background);
  }
  .store-help[open] {
    display: flex;
    flex-direction: column;
  }
  .store-help::backdrop {
    background: var(--background);
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.4rem 1rem;
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }
  header.desktop {
    padding-inline-start: 80px;
    min-height: 40px;
  }
  h1 {
    font-size: 1rem;
    margin: 0;
  }
  p,
  [role="alert"] {
    padding: 1rem;
  }
  iframe {
    border: 0;
    width: 100%;
    min-height: 0;
    flex: 1;
    visibility: hidden;
  }
  iframe.ready {
    visibility: visible;
  }
</style>
