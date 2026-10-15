<script lang="ts">
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreForgeIcon from "./StoreForgeIcon.svelte";
  import type { StackWorkspaceController } from "../../controller/stack-workspace/controller.svelte.js";
  import type { WorkspaceStack } from "../../controller/stack-workspace/types.js";
  let {
    controller,
    bookmark,
    stack,
    menuItem = false,
    compact = false,
  }: {
    controller: StackWorkspaceController;
    bookmark?: string;
    stack?: WorkspaceStack;
    menuItem?: boolean;
    compact?: boolean;
  } = $props();
  const name = $derived(bookmark ?? stack?.name ?? "");
  const mode = $derived(stack ? "stack" : "bookmark");
  const conflicts = $derived(Boolean(stack?.conflicts));
  $effect(() =>
    mode === "bookmark" ? controller.forge?.watch(name) : undefined,
  );
  const forge = $derived(controller.forge);
  const disabled = $derived(
    controller.locked || forge?.busy || Boolean(forge?.pending) || conflicts,
  );
  function open() {
    if (mode === "bookmark" && forge?.pr(name)) forge.select(name, "inline");
    else void forge?.preview(name);
  }
</script>

{#if forge}
  {#if menuItem}
    <Menu.Item {disabled} onSelect={open} aria-label="Submit stack">
      <StoreForgeIcon provider={forge.data?.connection?.provider} />Submit stack
    </Menu.Item>
  {:else}
    <Button
      variant="outline"
      size={compact ? "xs" : "sm"}
      class="store-forge-button"
      {disabled}
      aria-label={mode === "stack"
        ? `Submit stack ${name}`
        : `Pull request for ${name}`}
      onclick={(event) => {
        event.stopPropagation();
        open();
      }}
    >
      <StoreForgeIcon provider={forge.data?.connection?.provider} />
      {#if mode === "stack"}Submit stack{:else if compact && forge.pr(name)}PR #{forge.pr(
          name,
        )!.number}{:else if forge.pr(name)}#{forge.pr(name)!.number} · {forge.pr(
          name,
        )!.draft && forge.pr(name)!.state === "open"
          ? "Draft"
          : forge.pr(name)!.state}{:else}Create PR{/if}
    </Button>
  {/if}
{/if}

<style>
  :global(.store-forge-button[data-ui-component="button"]) {
    background-color: color-mix(
      in srgb,
      var(--foreground) 16%,
      var(--background)
    );
    border-color: color-mix(in srgb, var(--foreground) 30%, var(--background));
  }
  :global(
    .store-forge-button[data-ui-component="button"]:is(:hover, :focus-visible)
  ) {
    background-color: color-mix(
      in srgb,
      var(--foreground) 28%,
      var(--background)
    );
    border-color: color-mix(in srgb, var(--foreground) 42%, var(--background));
  }
</style>
