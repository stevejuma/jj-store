<script lang="ts">
  import { Spinner } from "@lapismd/design-core/shadcn/spinner";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import StoreForgeButton from "./StoreForgeButton.svelte";
  import StoreForgeCheckIcon from "./StoreForgeCheckIcon.svelte";
  import {
    forgeBranchPushStatus,
    forgeBranchCheckStatus,
  } from "../../controller/forge/branch-status.js";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  let {
    controller,
    branch,
    localHead,
  }: {
    controller: StackWorkspaceController;
    branch: string;
    localHead?: string;
  } = $props();
  const forge = $derived(controller.forge);
  const pr = $derived(forge?.pr(branch));
  const push = $derived(
    forgeBranchPushStatus(
      localHead,
      pr,
      forge?.stale || Boolean(forge?.pending),
    ),
  );
  const checks = $derived(
    forgeBranchCheckStatus(
      pr,
      forge?.ci[branch],
      forge?.data?.capabilities.checks ?? false,
      forge?.stale,
    ),
  );
</script>

{#if forge?.data?.connection}
  <div
    class="forge-branch-status"
    role="group"
    aria-label={`Branch status for ${branch}`}
  >
    <span
      class="forge-branch-status__push"
      title="Compared with the last observed pull request head.">{push}</span
    >
    {#if pr}
      <span class="forge-branch-status__separator" aria-hidden="true">·</span>
      <StoreForgeButton {controller} bookmark={branch} compact />
      <span
        role="status"
        aria-label={checks.description}
        title={`Checks for PR #${pr.number} · ${pr.sha.slice(0, 12)}`}
      >
        <CommunityChip
          class="forge-branch-status__check"
          variant="muted"
          size="compact"
          tone={checks.state === "passed"
            ? "success"
            : checks.state === "failed"
              ? "danger"
              : "neutral"}
        >
          {#snippet leading()}
            {#if checks.state === "running"}<Spinner
                aria-hidden="true"
                role="presentation"
              />
            {:else}<StoreForgeCheckIcon state={checks.state} />{/if}
          {/snippet}
          {checks.label}
        </CommunityChip>
      </span>
    {/if}
  </div>
{/if}

<style>
  .forge-branch-status {
    display: flex;
    min-width: 0;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.35rem;
    font-size: var(--community-text-secondary);
    line-height: 1.25;
  }
  .forge-branch-status__push,
  .forge-branch-status__separator {
    color: var(--muted-foreground);
  }
  .forge-branch-status :global(.forge-branch-status__check) {
    height: calc(var(--spacing) * 6);
    min-height: calc(var(--spacing) * 6);
    border-radius: min(var(--radius-md), 8px);
    padding-inline: calc(var(--spacing) * 2);
    font-size: var(--text-xs);
    font-weight: var(--font-weight-medium);
  }
  .forge-branch-status
    :global(.forge-branch-status__check[data-chip-tone="neutral"]) {
    background-color: color-mix(
      in srgb,
      var(--foreground) 16%,
      var(--background)
    );
    border-color: color-mix(in srgb, var(--foreground) 30%, var(--background));
  }
</style>
