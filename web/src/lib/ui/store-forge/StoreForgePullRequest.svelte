<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller }: { controller: ForgeController } = $props();
  const blocked = $derived(controller.busy || Boolean(controller.pending));
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  let method = $state("squash");
  let mergeConfirm = $state(false);
  $effect(() => {
    controller.selected?.number;
    mergeConfirm = false;
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
    <label
      >Description<Textarea
        rows={12}
        bind:value={controller.body}
        oninput={() => (controller.draftDirty = true)}
      /></label
    >
    <Button
      disabled={blocked ||
        !controller.draftDirty ||
        !controller.data.capabilities.edit}
      onclick={() => controller.edit()}>Save PR description</Button
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
      <label
        >Merge method<select bind:value={method}>
          {#each controller.data.capabilities.merge_methods as option}<option
              value={option}
              >{option === "merge"
                ? "Merge"
                : option === "squash"
                  ? "Squash"
                  : "Rebase"}</option
            >{/each}
        </select></label
      >
      <Button
        disabled={blocked ||
          !controller.data.capabilities.merge ||
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
