<script lang="ts">
  import { onDestroy } from "svelte";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import Check from "@lucide/svelte/icons/check";
  import Copy from "@lucide/svelte/icons/copy";
  import TriangleAlert from "@lucide/svelte/icons/triangle-alert";

  let {
    changeId = "",
    changeDisplay = "",
    commitId,
    commitDisplay = "",
    commitLabel = "commit ID",
    placement = "overlay",
  }: {
    changeId?: string | undefined;
    changeDisplay?: string | undefined;
    commitId: string;
    commitDisplay?: string | undefined;
    commitLabel?: string | undefined;
    placement?: "overlay" | "inline" | undefined;
  } = $props();

  let copied = $state("");
  let failed = $state("");
  let resetTimer: ReturnType<typeof setTimeout> | undefined;

  onDestroy(() => {
    if (resetTimer) clearTimeout(resetTimer);
  });

  function shorten(value: string): string {
    if (value.length <= 13) return value;
    return `${value.slice(0, 8)}…${value.slice(-4)}`;
  }

  function displayValue(
    value: string,
    explicit: string,
    length: number,
  ): string {
    if (explicit) return explicit;
    if (!value) return "";
    if (value.length <= length) return value;
    return value.slice(0, length);
  }

  async function copy(label: string, value: string): Promise<void> {
    copied = "";
    failed = "";
    try {
      const clipboard = globalThis.navigator?.clipboard;
      if (!clipboard?.writeText) throw new Error("Clipboard is unavailable");
      await clipboard.writeText(value);
      copied = label;
    } catch {
      failed = label;
    }
    if (resetTimer) clearTimeout(resetTimer);
    resetTimer = setTimeout(() => {
      copied = "";
      failed = "";
    }, 1_500);
  }
</script>

{#snippet copyIcon(state: string)}
  {#if state === "copied"}
    <Check aria-hidden="true" />
  {:else if state === "failed"}
    <TriangleAlert aria-hidden="true" />
  {:else}
    <Copy aria-hidden="true" />
  {/if}
{/snippet}

{#snippet idButton(label: string, value: string, tone: "change" | "commit")}
  {@const state =
    copied === label ? "copied" : failed === label ? "failed" : ""}
  <button
    type="button"
    class="community-revision-graph-ids__button"
    class:community-revision-graph-ids__button--change={tone === "change"}
    aria-label="Copy full {label}: {value}"
    title="Copy full {label}"
    data-copied={state === "copied" || undefined}
    data-copy-failed={state === "failed" || undefined}
    onclick={(event) => {
      event.stopPropagation();
      void copy(label, value);
    }}
  >
    <code>{shorten(value)}</code>
    {@render copyIcon(state)}
  </button>
{/snippet}

{#snippet splitButton(
  label: string,
  value: string,
  display: string,
  side: "left" | "right",
)}
  {@const state =
    copied === label ? "copied" : failed === label ? "failed" : ""}
  <Tooltip.Root>
    <Tooltip.Trigger>
      {#snippet child({ props })}
        <button
          {...props}
          type="button"
          class="community-revision-graph-ids__split-button"
          class:community-revision-graph-ids__split-button--left={side ===
            "left"}
          class:community-revision-graph-ids__split-button--right={side ===
            "right"}
          aria-label="Copy full {label}: {value}"
          data-copied={state === "copied" || undefined}
          data-copy-failed={state === "failed" || undefined}
          onclick={(event) => {
            event.stopPropagation();
            void copy(label, value);
          }}
        >
          {#if side === "left"}
            {@render copyIcon(state)}
            <code>{display}</code>
          {:else}
            <code>{display}</code>
            {@render copyIcon(state)}
          {/if}
        </button>
      {/snippet}
    </Tooltip.Trigger>
    <Tooltip.Content side="top" sideOffset={6}>{value}</Tooltip.Content>
  </Tooltip.Root>
{/snippet}

<div class="community-revision-graph-ids" data-placement={placement}>
  {#if changeId && commitLabel === "commit ID"}
    <div
      class="community-revision-graph-ids__split"
      role="group"
      aria-label="Native revision identifiers"
    >
      <Tooltip.Provider delayDuration={0}>
        {@render splitButton(
          commitLabel,
          commitId,
          displayValue(commitId, commitDisplay, 8),
          "left",
        )}
        {@render splitButton(
          "change ID",
          changeId,
          displayValue(changeId, changeDisplay, 12),
          "right",
        )}
      </Tooltip.Provider>
    </div>
  {:else if changeId}
    {@render idButton("change ID", changeId, "change")}
    {@render idButton(commitLabel, commitId, "commit")}
  {:else}
    {@render idButton(commitLabel, commitId, "commit")}
  {/if}
</div>

<style>
  .community-revision-graph-ids {
    position: absolute;
    z-index: 3;
    top: calc(var(--community-revision-graph-row-height, 76px) / 2);
    right: 0.5rem;
    display: flex;
    transform: translateY(-50%);
    flex-direction: column;
    align-items: flex-end;
    gap: 0.2rem;
  }

  .community-revision-graph-ids[data-placement="inline"] {
    position: static;
    z-index: auto;
    top: auto;
    right: auto;
    transform: none;
    flex-direction: row;
    flex-wrap: wrap;
    align-items: center;
  }

  .community-revision-graph-ids__button {
    display: inline-flex;
    min-width: 0;
    max-width: 100%;
    height: 1.65rem;
    align-items: center;
    gap: 0.35rem;
    padding: 0 0.4rem;
    border: 1px solid transparent;
    border-radius: 0.4rem;
    color: inherit;
    background: color-mix(in srgb, currentcolor 7%, transparent);
    font-weight: 500;
    cursor: pointer;
  }

  .community-revision-graph-ids__button--change {
    color: var(--primary);
  }

  .community-revision-graph-ids__split {
    display: inline-flex;
    min-width: 0;
    max-width: 100%;
    overflow: hidden;
    border: 1px solid transparent;
    border-radius: 0.4rem;
    color: inherit;
    background: color-mix(in srgb, currentcolor 7%, transparent);
  }

  .community-revision-graph-ids__split-button {
    display: inline-flex;
    min-width: 0;
    height: 1.65rem;
    align-items: center;
    gap: 0.35rem;
    padding: 0 0.4rem;
    border: 0;
    color: inherit;
    background: transparent;
    font: inherit;
    font-weight: 500;
    cursor: pointer;
  }

  .community-revision-graph-ids__split-button--right {
    color: var(--primary);
    border-left: 1px solid color-mix(in srgb, currentcolor 16%, transparent);
  }

  .community-revision-graph-ids__button code {
    min-width: 0;
    overflow: hidden;
    font-family: var(--font-mono, monospace);
    font-size: var(--community-text-secondary);
    font-weight: 500;
    line-height: 1;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .community-revision-graph-ids__button :global(svg) {
    width: 0.75rem;
    height: 0.75rem;
    flex: 0 0 auto;
  }

  .community-revision-graph-ids__split-button code {
    min-width: 0;
    overflow: hidden;
    font-family: var(--font-mono, monospace);
    font-size: var(--community-text-secondary);
    font-weight: 500;
    line-height: 1;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .community-revision-graph-ids__split-button :global(svg) {
    width: 0.75rem;
    height: 0.75rem;
    flex: 0 0 auto;
  }

  @media (hover: hover) {
    .community-revision-graph-ids__button:hover {
      border-color: color-mix(in srgb, currentcolor 20%, transparent);
      background: color-mix(in srgb, currentcolor 12%, transparent);
      box-shadow: 0 1px 2px color-mix(in srgb, currentcolor 14%, transparent);
    }

    .community-revision-graph-ids__split:has(
      .community-revision-graph-ids__split-button:hover
    ) {
      border-color: color-mix(in srgb, currentcolor 20%, transparent);
      background: color-mix(in srgb, currentcolor 12%, transparent);
      box-shadow: 0 1px 2px color-mix(in srgb, currentcolor 14%, transparent);
    }

    .community-revision-graph-ids__split-button:hover {
      background: color-mix(in srgb, currentcolor 8%, transparent);
    }
  }

  .community-revision-graph-ids__button:focus-visible {
    border-color: color-mix(in srgb, currentcolor 28%, transparent);
    background: color-mix(in srgb, currentcolor 12%, transparent);
    outline: none;
  }

  .community-revision-graph-ids__split-button:focus-visible {
    background: color-mix(in srgb, currentcolor 10%, transparent);
    outline: none;
  }

  .community-revision-graph-ids__split:has(
    .community-revision-graph-ids__split-button:focus-visible
  ) {
    border-color: color-mix(in srgb, currentcolor 28%, transparent);
    background: color-mix(in srgb, currentcolor 12%, transparent);
  }
</style>
