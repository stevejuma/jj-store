<script lang="ts">
  import StoreSettingsIcon from "./StoreSettingsIcon.svelte";
  import { settingsRowIcons } from "./settings-row-icons.js";
  import { onDestroy, type Component, type Snippet } from "svelte";
  import {
    settingsPresentation,
    type SettingsPresentation,
  } from "./settings-presentation.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Copy from "@lucide/svelte/icons/copy";
  import Check from "@lucide/svelte/icons/circle-check";
  import AlertCircle from "@lucide/svelte/icons/circle-alert";
  import ArrowUpRight from "@lucide/svelte/icons/arrow-up-right";
  let {
    label,
    copyLabel = label,
    copyable = false,
    value,
    children,
    display,
    emptyLabel = "Unavailable",
    description,
    stacked = false,
    standalone = false,
    tone,
    live,
    onOpen,
    openLabel = `Open ${label}`,
    openDisabled = false,
    presentation,
    Icon,
  }: {
    label: string;
    copyLabel?: string;
    copyable?: boolean;
    value?: string | number | bigint | null | undefined;
    children?: Snippet;
    display?: string;
    emptyLabel?: string;
    description?: string;
    stacked?: boolean;
    standalone?: boolean;
    tone?: "ready" | "warning" | "error";
    live?: "status" | "alert";
    onOpen?: () => void;
    openLabel?: string;
    openDisabled?: boolean;
    presentation?: SettingsPresentation;
    Icon?: Component;
  } = $props();
  const inheritedPresentation = settingsPresentation();
  const compact = $derived(
    (presentation ?? inheritedPresentation.value) !== "default",
  );
  const RowIcon = $derived(Icon ?? settingsRowIcons[label]);
  const text = $derived(value == null ? "" : String(value));
  const canCopy = $derived(copyable || !!onOpen);
  let feedback = $state<"" | "copied" | "failed">("");
  let feedbackVersion = $state(0);
  let timer: ReturnType<typeof setTimeout> | undefined;
  let attempt = 0;
  $effect(() => {
    text;
    canCopy;
    attempt += 1;
    feedback = "";
    clearTimeout(timer);
  });
  onDestroy(() => {
    attempt += 1;
    clearTimeout(timer);
  });
  async function copy() {
    const current = ++attempt,
      captured = text;
    clearTimeout(timer);
    try {
      await navigator.clipboard.writeText(captured);
      if (current !== attempt || captured !== text) return;
      feedback = "copied";
      feedbackVersion += 1;
    } catch {
      if (current !== attempt || captured !== text) return;
      feedback = "failed";
    }
    timer = setTimeout(() => {
      feedback = "";
    }, 1500);
  }
</script>

<svelte:element
  this={standalone ? "dl" : "div"}
  class="store-readonly-row"
  class:store-readonly-row--stacked={stacked}
  class:store-readonly-row--compact={compact}
  class:store-readonly-row--copyable={canCopy}
>
  <dt>
    {#if RowIcon && compact}<span class="store-readonly-row__icon"
        ><StoreSettingsIcon Icon={RowIcon} {tone} /></span
      >{/if}
    <span
      >{label}{#if description}<p>{description}</p>{/if}</span
    >
  </dt>
  <dd>
    <span
      class="store-readonly-row__value"
      data-tone={tone}
      role={live}
      title={canCopy && text ? text : undefined}
      >{#if children}{@render children()}{:else}{text
          ? (display ?? text)
          : emptyLabel}{/if}</span
    >
    {#if text && canCopy}
      <span class="store-readonly-row__actions">
        {#if onOpen}<Button
            type="button"
            variant="ghost"
            size={compact ? "icon-xs" : "icon-sm"}
            class="store-readonly-row__open"
            aria-label={openLabel}
            title={openLabel}
            disabled={openDisabled}
            onclick={onOpen}><ArrowUpRight aria-hidden="true" /></Button
          >{/if}
        <Button
          type="button"
          variant="ghost"
          size={compact ? "icon-xs" : "icon-sm"}
          class="store-readonly-row__copy"
          aria-label={`Copy full ${copyLabel}: ${text}`}
          title={compact ? `Copy ${copyLabel}: ${text}` : `Copy ${copyLabel}`}
          data-copied={feedback === "copied" || undefined}
          data-copy-failed={feedback === "failed" || undefined}
          onclick={() => void copy()}
        >
          {#key feedbackVersion}{#if feedback === "copied"}<Check
                aria-hidden="true"
              />{:else if feedback === "failed"}<AlertCircle
                aria-hidden="true"
              />{:else}<Copy aria-hidden="true" />{/if}{/key}
        </Button>
      </span>
    {/if}
    <span class="sr-only" role="status"
      >{feedback === "copied"
        ? `${copyLabel} copied`
        : feedback === "failed"
          ? `Could not copy ${copyLabel}. Try again.`
          : ""}</span
    >
  </dd>
</svelte:element>

<style>
  .store-readonly-row {
    position: relative;
    display: grid;
    grid-template-columns: minmax(10rem, 0.8fr) minmax(0, 1.5fr);
    align-items: center;
    gap: 1rem;
    min-width: 0;
    margin: 0;
    padding: 0.85rem 1rem;
  }
  .store-readonly-row--compact {
    padding: 0.65rem 1rem;
  }
  .store-readonly-row--compact:not(.store-readonly-row--copyable)
    .store-readonly-row__value {
    overflow: visible;
    text-overflow: clip;
    white-space: normal;
    overflow-wrap: anywhere;
  }
  .store-readonly-row--compact dt {
    display: flex;
    align-items: center;
    gap: 0.6rem;
  }
  .store-readonly-row__icon {
    display: inline-flex;
    flex: none;
    color: var(--muted-foreground);
  }

  dt {
    min-width: 0;
    font-size: var(--community-text-secondary);
    font-weight: 550;
    overflow-wrap: anywhere;
  }
  dt p {
    margin: 0.35rem 0 0;
    color: var(--muted-foreground);
    font-weight: 400;
  }
  dd {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    margin: 0;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
  .store-readonly-row__value {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .store-readonly-row__actions {
    display: flex;
    flex: none;
    align-items: center;
    margin-inline-start: auto;
  }
  .store-readonly-row :global(.store-readonly-row__copy) {
    position: static;
    /* A translated button would capture its stretched hit area while pressed. */
    translate: none;
  }
  .store-readonly-row :global(.store-readonly-row__copy::after) {
    content: "";
    position: absolute;
    inset: 0;
  }
  .store-readonly-row :global(.store-readonly-row__copy:focus-visible::after) {
    outline: 2px solid var(--ring);
    outline-offset: -2px;
  }
  .store-readonly-row :global(.store-readonly-row__open) {
    position: relative;
    z-index: 1;
  }
  .store-readonly-row:has(:global(.store-readonly-row__copy:hover)) {
    background: var(--muted);
  }
  @media (prefers-reduced-motion: no-preference) {
    .store-readonly-row
      :global(.store-readonly-row__copy[data-copied="true"] svg) {
      animation: copy-confirm 160ms ease-out;
    }
  }
  @keyframes copy-confirm {
    from {
      transform: scale(0.75);
    }
    to {
      transform: scale(1);
    }
  }
  [data-tone="ready"] {
    color: var(--success-foreground, var(--foreground));
  }
  [data-tone="warning"] {
    color: var(--warning-foreground, var(--foreground));
  }
  [data-tone="error"] {
    color: var(--destructive);
  }
  .store-readonly-row--stacked {
    grid-template-columns: minmax(0, 1fr) auto;
    column-gap: 0.5rem;
    row-gap: 0;
    min-height: 4rem;
    padding: 0.55rem 0.5rem;
  }
  .store-readonly-row--stacked dt {
    font-family: var(--font-sans);
    font-size: var(--text-sm);
    line-height: var(--text-sm--line-height);
    font-weight: 400;
  }
  .store-readonly-row--stacked dd {
    display: contents;
  }
  .store-readonly-row--stacked .store-readonly-row__value {
    grid-column: 1;
    grid-row: 2;
  }
  .store-readonly-row--stacked .store-readonly-row__actions {
    grid-column: 2;
    grid-row: 1 / 3;
  }
  @media (max-width: 40rem) {
    .store-readonly-row:not(.store-readonly-row--stacked) {
      grid-template-columns: minmax(0, 1fr);
      gap: 0.3rem;
    }
  }
</style>
