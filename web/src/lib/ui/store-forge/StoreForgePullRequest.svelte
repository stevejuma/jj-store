<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreForgePullRequestActions from "./StoreForgePullRequestActions.svelte";
  import StoreForgeChecks from "./StoreForgeChecks.svelte";
  import StoreForgeDescription from "./StoreForgeDescription.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let {
    controller,
    editing = $bindable(false),
  }: {
    controller: ForgeController;
    editing?: boolean;
  } = $props();
  const blocked = $derived(
    controller.busy ||
      Boolean(controller.pending) ||
      (controller.accounts.managed && !controller.ready),
  );
  import { Input } from "@lapismd/design-core/shadcn/input";
  const selection = $derived(controller.selected?.number);
  $effect(() => {
    selection;
    editing = false;
  });
</script>

{#if controller.selected && controller.data}
  {@const pr = controller.selected}
  <section class="forge-pr" aria-label={`PR ${pr.number}`}>
    <div class="forge-pr__content">
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
      {#if controller.merge}<p role="status">
          Merge: {controller.merge.state}
        </p>{/if}
    </div>
    <StoreForgeChecks {controller} />
    <StoreForgePullRequestActions {controller} footer />
  </section>
{/if}
