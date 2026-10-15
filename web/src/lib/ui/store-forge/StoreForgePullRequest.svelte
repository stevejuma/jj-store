<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import StoreForgeDescription from "./StoreForgeDescription.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller }: { controller: ForgeController } = $props();
  const blocked = $derived(controller.busy || Boolean(controller.pending));
  import { Input } from "@lapismd/design-core/shadcn/input";
  let method = $state("squash");
  let editing = $state(false);
  const methods = $derived(controller.data?.capabilities.merge_methods ?? []);
  const methodLabel = (value: string) =>
    value === "merge" ? "Merge" : value === "squash" ? "Squash" : "Rebase";
  let mergeConfirm = $state(false);
  $effect(() => {
    controller.selected?.number;
    mergeConfirm = false;
    editing = untrack(() => controller.draftDirty);
  });
  $effect(() => {
    if (!methods.includes(method)) method = methods[0] ?? "";
  });
</script>

{#if controller.selected && controller.data}
  {@const pr = controller.selected}
  <section aria-label={`PR ${pr.number}`}>
    <h3>
      <a href={pr.url} target="_blank" rel="noreferrer">#{pr.number}</a> · {pr.state}{pr.draft
        ? " · Draft"
        : ""}
    </h3>
    <p>{pr.head} → {pr.base}</p>
    <label
      >Title<Input
        bind:value={controller.title}
        oninput={() => (controller.draftDirty = true)}
      /></label
    >
    <StoreForgeDescription
      bind:value={
        () => controller.body,
        (value) => {
          controller.body = value;
          controller.draftDirty = true;
        }
      }
      bind:editing
      disabled={blocked || !controller.data.capabilities.edit}
    />
    <Button
      disabled={blocked ||
        !controller.draftDirty ||
        !controller.data.capabilities.edit}
      onclick={async () => {
        await controller.edit();
        if (!controller.draftDirty) editing = false;
      }}>Save PR description</Button
    >
    <div class="forge-panel__toolbar">
      <Button
        variant="outline"
        disabled={blocked || !controller.data.capabilities.checks}
        onclick={() => {
          void controller.act({ kind: "checks" });
        }}>Checks and runs</Button
      >
      {#if controller.data.capabilities.draft}<Button
          variant="outline"
          disabled={blocked || pr.state !== "open"}
          onclick={() => controller.act({ kind: "draft", draft: !pr.draft })}
          >{pr.draft ? "Ready for review" : "Convert to draft"}</Button
        >{/if}
    </div>
    <Button
      variant="outline"
      disabled={blocked}
      onclick={() => controller.previewRestack(pr.head)}>Restack…</Button
    >
    {#if pr.state === "open"}
      <Menu.Root>
        <Menu.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="outline"
              disabled={blocked ||
                !controller.data?.capabilities.merge ||
                !methods.length}
              aria-label={`Merge method: ${method ? methodLabel(method) : "Unavailable"}`}
            >
              {method ? methodLabel(method) : "Merge unavailable"}
              <ChevronDown data-icon="inline-end" aria-hidden="true" />
            </Button>
          {/snippet}
        </Menu.Trigger>
        <Menu.Content align="start" preventScroll={false}>
          <Menu.Group>
            <Menu.Label>Merge method</Menu.Label>
            <Menu.RadioGroup bind:value={method}>
              {#each methods as option}
                <Menu.RadioItem value={option}
                  >{methodLabel(option)}</Menu.RadioItem
                >
              {/each}
            </Menu.RadioGroup>
          </Menu.Group>
        </Menu.Content>
      </Menu.Root>
      <Button
        disabled={blocked ||
          !controller.data.capabilities.merge ||
          !methods.includes(method) ||
          pr.draft ||
          pr.mergeable !== true}
        onclick={() => (mergeConfirm = true)}
        >{pr.native_stack ? "Merge through this PR…" : "Merge next…"}</Button
      >
      {#if mergeConfirm}<div role="group" aria-label="Confirm merge">
          <p>
            {pr.native_stack
              ? "Merge this PR and its lower stack dependencies using forge protection rules?"
              : `Merge PR #${pr.number} into ${pr.base}?`}
          </p>
          <Button
            disabled={blocked}
            onclick={() => {
              mergeConfirm = false;
              void controller.act({
                kind: "merge",
                method,
                expected_sha: pr.sha,
              });
            }}>Confirm merge</Button
          ><Button variant="outline" onclick={() => (mergeConfirm = false)}
            >Cancel</Button
          >
        </div>{/if}
    {/if}
    {#if controller.merge}<p role="status">
        Merge: {controller.merge.state}
      </p>
      {#if controller.merge.receipt}<Button
          variant="outline"
          disabled={blocked}
          onclick={() =>
            controller.act({
              kind: "merge_status",
              receipt: controller.merge!.receipt!,
            })}>Check merge result</Button
        >{/if}{/if}
  </section>
{/if}
