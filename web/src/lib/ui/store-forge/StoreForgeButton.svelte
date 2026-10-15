<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import CommunityChip from "../community-chip/CommunityChip.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceStack } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    bookmark,
    stack,
  }: {
    controller: StackWorkspaceController;
    bookmark?: string;
    stack?: WorkspaceStack;
  } = $props();
  const name = $derived(bookmark ?? stack?.name ?? "");
  const mode = $derived(stack ? "stack" : "bookmark");
  const conflicts = $derived(Boolean(stack?.conflicts));
  $effect(() =>
    mode === "bookmark" ? controller.forge?.watch(name) : undefined,
  );
  const forge = $derived(controller.forge);
</script>

{#if forge}
  <Button
    variant="ghost"
    size="sm"
    disabled={controller.locked ||
      forge.busy ||
      Boolean(forge.pending) ||
      conflicts}
    aria-label={mode === "stack"
      ? `Submit stack ${name}`
      : `Pull request for ${name}`}
    onclick={(event) => {
      event.stopPropagation();
      if (mode === "bookmark" && forge.pr(name)) forge.select(name);
      else void forge.preview(name);
    }}
  >
    {#if mode === "stack"}Submit stack{:else if forge.pr(name)}#{forge.pr(name)!
        .number} · {forge.pr(name)!.state}{:else}Create PR{/if}
  </Button>
  {#if mode === "bookmark" && forge.pr(name)}<CommunityChip variant="muted"
      >{forge.checkSummary(name)}</CommunityChip
    >{/if}
{/if}
