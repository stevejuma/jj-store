<script lang="ts">
  import { onDestroy } from "svelte";
  import * as Tooltip from "@lapismd/design-core/shadcn/tooltip";
  import Check from "@lucide/svelte/icons/check";
  import Copy from "@lucide/svelte/icons/copy";
  import TriangleAlert from "@lucide/svelte/icons/triangle-alert";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import { abbreviateRevisionId, revisionLeadingChars } from "./revision-id.js";
  import "./revision-graph-ids.css";

  let {
    changeId = "",
    commitId = "",
    commitLabel = "commit ID",
    compactLeadingChars = 6,
    expandedLeadingChars = 10,
    placement = "overlay",
  }: {
    changeId?: string | undefined;
    commitId?: string | undefined;
    commitLabel?: string | undefined;
    /** Leading characters retained before ... and the final two characters. */
    compactLeadingChars?: number;
    /** Leading characters shown by the hovered or keyboard-focused half. */
    expandedLeadingChars?: number;
    placement?: "overlay" | "inline" | undefined;
  } = $props();

  type Side = "commit" | "change";
  type Feedback = "" | "copied" | "failed";
  let feedback = $state<Record<Side, Feedback>>({ commit: "", change: "" });
  const timers: Partial<Record<Side, ReturnType<typeof setTimeout>>> = {};
  const requests = { commit: 0, change: 0 };
  let destroyed = false;
  const compact = $derived(revisionLeadingChars(compactLeadingChars, 6));
  const expanded = $derived(
    Math.max(compact, revisionLeadingChars(expandedLeadingChars, 10)),
  );
  const capacity = $derived(Math.max(compact * 2, expanded + 2));

  function reset() {
    for (const side of ["commit", "change"] as const) {
      clearTimeout(timers[side]);
      requests[side]++;
      feedback[side] = "";
    }
  }
  $effect(() => {
    // Reused file/snapshot surfaces must not show feedback for the previous ID.
    void commitId;
    void changeId;
    reset();
  });
  onDestroy(() => {
    destroyed = true;
    reset();
  });

  async function copy(side: Side, value: string): Promise<void> {
    const request = ++requests[side];
    clearTimeout(timers[side]);
    feedback[side] = "";
    let result: Feedback;
    try {
      const clipboard = globalThis.navigator?.clipboard;
      if (!clipboard?.writeText) throw new Error("Clipboard is unavailable");
      await clipboard.writeText(value);
      result = "copied";
    } catch {
      result = "failed";
    }
    if (destroyed || requests[side] !== request) return;
    feedback[side] = result;
    timers[side] = setTimeout(() => {
      feedback[side] = "";
    }, 1_500);
  }
</script>

{#snippet splitButton(label: string, value: string, side: Side, chip: string)}
  <Tooltip.Root>
    <Tooltip.Trigger>
      {#snippet child({ props })}
        <button
          {...props}
          type="button"
          class="community-revision-graph-ids__split-button"
          data-side={side}
          aria-label="Copy {label} {value}"
          data-copied={feedback[side] === "copied" || undefined}
          data-copy-failed={feedback[side] === "failed" || undefined}
          onclick={(event) => {
            event.stopPropagation();
            void copy(side, value);
          }}
        >
          <CommunityChip size="inline" shape="square" variant="muted" ariaHidden
            >{chip}</CommunityChip
          >
          {#if feedback[side]}
            <code>{feedback[side] === "copied" ? "Copied!" : "Retry"}</code>
          {:else}
            <code class="id-compact"
              >{abbreviateRevisionId(value, compact)}</code
            >
            <code class="id-expanded"
              >{abbreviateRevisionId(value, expanded)}</code
            >
            <code class="id-collapsed">{abbreviateRevisionId(value, 2)}</code>
          {/if}
        </button>
      {/snippet}
    </Tooltip.Trigger>
    <Tooltip.Content side="top" sideOffset={6}>{value}</Tooltip.Content>
  </Tooltip.Root>
{/snippet}

{#if commitId || changeId}
  <div class="community-revision-graph-ids" data-placement={placement}>
    {#if commitLabel === "commit ID"}
      <div
        class="community-revision-graph-ids__split"
        data-single={!commitId || !changeId || undefined}
        style:--id-capacity={capacity + 10}
        style:--id-expanded={expanded + 5}
        role="group"
        aria-label="Native revision identifiers"
      >
        <Tooltip.Provider delayDuration={0}>
          {#if commitId}{@render splitButton(
              "Git commit SHA",
              commitId,
              "commit",
              "GIT",
            )}{/if}
          {#if changeId}{@render splitButton(
              "JJ Change ID",
              changeId,
              "change",
              "JJ",
            )}{/if}
        </Tooltip.Provider>
      </div>
    {:else if commitId}
      <button
        type="button"
        class="community-revision-graph-ids__button"
        aria-label="Copy full {commitLabel}: {commitId}"
        title="Copy full {commitLabel}"
        data-copied={feedback.commit === "copied" || undefined}
        data-copy-failed={feedback.commit === "failed" || undefined}
        onclick={(event) => {
          event.stopPropagation();
          void copy("commit", commitId);
        }}
      >
        <code
          >{commitId.length <= 13
            ? commitId
            : `${commitId.slice(0, 8)}…${commitId.slice(-4)}`}</code
        >
        {#if feedback.commit === "copied"}<Check aria-hidden="true" />
        {:else if feedback.commit === "failed"}<TriangleAlert
            aria-hidden="true"
          />
        {:else}<Copy aria-hidden="true" />{/if}
      </button>
    {/if}
    <span class="sr-only" role="status" aria-atomic="true">
      {#each ["commit", "change"] as const as side}
        {#if feedback[side]}
          {feedback[side] === "copied" ? "Copied" : "Could not copy"}
          {side === "change"
            ? "JJ Change ID"
            : commitLabel === "commit ID"
              ? "Git commit SHA"
              : commitLabel}.
        {/if}
      {/each}
    </span>
  </div>
{/if}
