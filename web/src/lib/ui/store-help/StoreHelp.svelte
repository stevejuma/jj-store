<script lang="ts">
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  const desktop = isMacDesktopWindow();
  import { Button } from "@lapismd/design-core/shadcn/button";
  import BookOpenIcon from "@lucide/svelte/icons/book-open";
  import XIcon from "@lucide/svelte/icons/x";
  import StoreHelpSkeleton from "./StoreHelpSkeleton.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
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
  let failure = $state<string | null>(null);
  let loading = $state(false);
  let source = $state(helpSession.path);
  let opener: HTMLElement | null = null;
  let timeout: ReturnType<typeof setTimeout> | undefined;
  function arm() {
    clearTimeout(timeout);
    loading = true;
    timeout = setTimeout(() => {
      fail("Documentation did not finish loading. Please retry.");
    }, 15000);
  }
  function fail(message: string) {
    clearTimeout(timeout);
    loading = false;
    failure = message;
  }
  function loaded() {
    // Error responses cannot send the ready protocol. Surface their actual
    // plain-text error immediately rather than waiting for the readiness timer.
    try {
      const document = frame?.contentDocument;
      if (!document || document.URL === "about:blank") return;
      if (!document.querySelector('[data-md-component="container"]'))
        fail(
          document.body.textContent?.trim().slice(0, 500) ||
            "Documentation could not be loaded.",
        );
    } catch {
      fail("Documentation could not be loaded from this application.");
    }
  }
  function open(event: MouseEvent) {
    opener =
      event.currentTarget instanceof HTMLElement ? event.currentTarget : null;
    mounted = true;
    opened = true;
    dialog?.showModal();
    if (loading || (!ready && !failure)) arm();
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
    } else if (event.data.type === "jj-store-help-loading") {
      arm();
    } else if (event.data.type === "jj-store-help-error") {
      fail(event.data.message);
    } else {
      ready = true;
      loading = false;
      failure = null;
      clearTimeout(timeout);
      if (helpSession.path === source && helpSession.scrollY) {
        frame?.contentWindow?.scrollTo(0, helpSession.scrollY);
        helpSession.scrollY = 0;
      }
    }
  }
  function focusBoundary(event: KeyboardEvent) {
    if (event.key !== "Tab" || !event.shiftKey || !ready || loading) return;
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
    arm();
    if (ready) {
      frame?.contentWindow?.postMessage(
        { type: "jj-store-help-retry" },
        window.location.origin,
      );
    } else {
      frame?.contentWindow?.location.replace(source);
    }
  }
  $effect(() => () => clearTimeout(timeout));
</script>

<svelte:window onmessage={message} />
<StoreNotice
  noticeKey="help-load"
  scope="help"
  title="Help could not be loaded"
  message={failure}
  pending={loading}
  action={{ id: "retry-help", label: "Retry", run: retry }}
/>
<div
  class="store-help-entry"
  data-sidebar={sidebar || undefined}
  data-compact={compact || undefined}
>
  <Button
    variant="ghost"
    size={compact ? "icon-sm" : "default"}
    class="store-help-trigger"
    data-sidebar={sidebar || undefined}
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
      class="store-help-close"
      aria-label="Close Help"
      onclick={close}><XIcon aria-hidden="true" /></Button
    >
  </header>
  <div class="store-help__body">
    {#if loading && !ready}<div class="store-help__loading">
        <StoreHelpSkeleton />
      </div>{/if}
    {#if mounted}<iframe
        bind:this={frame}
        src={source}
        title="jj-store documentation"
        class:ready
        onload={loaded}
        onerror={() => fail("Documentation could not be loaded. Please retry.")}
        referrerpolicy="no-referrer"
      ></iframe>{/if}
  </div>
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
  :global(.store-help-trigger[data-sidebar]:is(:hover, :focus-visible)),
  :global(.store-help-close:is(:hover, :focus-visible)) {
    color: var(--foreground);
    background: color-mix(in srgb, var(--foreground) 7%, transparent);
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
    background: var(--store-theme-gradient, var(--background));
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
    border-bottom: 1px solid
      color-mix(in srgb, var(--foreground) 11%, transparent);
    flex-shrink: 0;
    background: var(--store-theme-chrome, var(--sidebar));
  }
  header.desktop {
    padding-inline-start: 80px;
    min-height: 40px;
  }
  h1 {
    font-size: var(--community-text-body, 0.875rem);
    margin: 0;
  }
  .store-help__body {
    position: relative;
    display: flex;
    flex: 1;
    min-height: 0;
  }
  .store-help__loading {
    position: absolute;
    inset: 0;
    z-index: 1;
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
