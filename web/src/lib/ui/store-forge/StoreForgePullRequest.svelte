<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreForgeChecks from "./StoreForgeChecks.svelte";
  import StoreForgeDescription from "./StoreForgeDescription.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let {
    controller,
    editing = $bindable(false),
    showChecks = true,
  }: {
    controller: ForgeController;
    editing?: boolean;
    showChecks?: boolean;
  } = $props();
  const blocked = $derived(controller.busy || Boolean(controller.pending));
  import { Input } from "@lapismd/design-core/shadcn/input";
  const selection = $derived(controller.selected?.number);
  $effect(() => {
    selection;
    editing = false;
  });
</script>

{#if controller.selected && controller.data}
  {@const pr = controller.selected}
  <section aria-label={`PR ${pr.number}`}>
    {#if editing}
      <label
        >Title<Input
          bind:value={controller.title}
          oninput={() => (controller.draftDirty = true)}
          disabled={blocked || !controller.data.capabilities.edit}
        /></label
      >
    {/if}
    <p class="forge-pr__branches">
      <code>{pr.head}</code><span aria-label="into">→</span><code
        >{pr.base}</code
      >
    </p>
    <StoreForgeDescription
      bind:value={
        () => controller.body,
        (value) => {
          controller.body = value;
          controller.draftDirty = true;
        }
      }
      bind:editing
      showEditButton={false}
      disabled={blocked || !controller.data.capabilities.edit}
    />
    {#if controller.draftDirty || editing}
      <Button
        disabled={blocked ||
          !controller.draftDirty ||
          !controller.data.capabilities.edit}
        onclick={async () => {
          await controller.edit();
          if (!controller.draftDirty) editing = false;
        }}>Save changes</Button
      >
    {/if}
    {#if showChecks}<StoreForgeChecks {controller} />{/if}
    {#if controller.merge}<p role="status">
        Merge: {controller.merge.state}
      </p>{/if}
  </section>
{/if}
