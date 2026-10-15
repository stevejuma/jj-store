<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import GitMerge from "@lucide/svelte/icons/git-merge";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let { controller, branch }: { controller: ForgeController; branch?: string } =
    $props();
  let method = $state("squash"),
    mergeConfirm = $state(false);
  const selected = $derived(controller.selected);
  const pr = $derived(
    branch
      ? selected?.head === branch
        ? selected
        : controller.pr(branch)
      : selected,
  );
  const merge = $derived(pr ? controller.merges[String(pr.number)] : undefined);
  const blocked = $derived(controller.busy || Boolean(controller.pending));
  const methods = $derived(controller.data?.capabilities.merge_methods ?? []);
  const methodLabel = (value: string) =>
    value === "merge" ? "Merge" : value === "squash" ? "Squash" : "Rebase";
  const mergeDisabled = $derived(
    blocked ||
      !controller.data?.capabilities.merge ||
      !methods.includes(method) ||
      pr?.draft ||
      pr?.state !== "open" ||
      pr?.mergeable !== true,
  );
  const identity = $derived(pr?.number);
  $effect(() => {
    identity;
    mergeConfirm = false;
  });
  $effect(() => {
    if (!methods.includes(method)) method = methods[0] ?? "";
  });
  function select() {
    if (pr && selected?.number !== pr.number)
      controller.select(pr.head, branch ? "inline" : "column");
  }
</script>

{#if pr}
  <Menu.Root>
    <Menu.Trigger>
      {#snippet child({ props })}
        <Button
          {...props}
          variant="ghost"
          size="icon-sm"
          aria-label={`PR actions: ${method ? methodLabel(method) : "Merge unavailable"}`}
          title="PR actions"
          disabled={blocked}
        >
          <GitMerge aria-hidden="true" />
        </Button>
      {/snippet}
    </Menu.Trigger>
    <Menu.Content align="end" preventScroll={false}>
      <Menu.Group>
        <Menu.Label>Pull request actions</Menu.Label>
        {#if controller.data?.capabilities.draft}
          <Menu.Item
            disabled={blocked || pr.state !== "open"}
            onSelect={() => {
              select();
              void controller.act({ kind: "draft", draft: !pr.draft });
            }}>{pr.draft ? "Ready for review" : "Convert to draft"}</Menu.Item
          >
        {/if}
        <Menu.Item
          disabled={blocked}
          onSelect={() => {
            select();
            void controller.previewRestack(pr.head);
          }}>Restack…</Menu.Item
        >
        <Menu.Item
          disabled={mergeDisabled}
          onSelect={() => {
            select();
            mergeConfirm = true;
          }}
          >{pr.native_stack
            ? "Merge through this PR…"
            : "Merge next…"}</Menu.Item
        >
        {#if merge}
          <Menu.Label>Merge: {merge.state}</Menu.Label>
          {#if merge.receipt}<Menu.Item
              disabled={blocked}
              onSelect={() => {
                select();
                void controller.act({
                  kind: "merge_status",
                  receipt: merge.receipt!,
                });
              }}>Check merge result</Menu.Item
            >{/if}
        {/if}
      </Menu.Group>
      {#if pr.state === "open"}
        <Menu.Separator />
        <Menu.Group>
          <Menu.Label>Merge method</Menu.Label>
          <Menu.RadioGroup bind:value={method}>
            {#each methods as option}
              <Menu.RadioItem
                value={option}
                disabled={blocked || !controller.data?.capabilities.merge}
                >{methodLabel(option)}</Menu.RadioItem
              >
            {/each}
          </Menu.RadioGroup>
        </Menu.Group>
      {/if}
    </Menu.Content>
  </Menu.Root>
  <Dialog.Root bind:open={mergeConfirm}>
    <Dialog.Content>
      <Dialog.Header>
        <Dialog.Title>Merge PR #{pr.number}?</Dialog.Title>
        <Dialog.Description
          >{pr.native_stack
            ? "Merge this PR and its lower stack dependencies using forge protection rules?"
            : `Merge PR #${pr.number} into ${pr.base}?`}</Dialog.Description
        >
      </Dialog.Header>
      <Dialog.Footer>
        <Button variant="outline" onclick={() => (mergeConfirm = false)}
          >Cancel</Button
        >
        <Button
          disabled={mergeDisabled}
          onclick={() => {
            mergeConfirm = false;
            void controller.act({
              kind: "merge",
              method,
              expected_sha: pr.sha,
            });
          }}>Confirm merge</Button
        >
      </Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{/if}
