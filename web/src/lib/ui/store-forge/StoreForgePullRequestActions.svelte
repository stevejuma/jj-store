<script lang="ts">
  import { forgeMenuLayout } from "./menu-layout.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import CircleX from "@lucide/svelte/icons/circle-x";
  import CircleHelp from "@lucide/svelte/icons/circle-help";
  import "./forge-merge.css";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import FilePenLine from "@lucide/svelte/icons/file-pen-line";
  import CircleCheck from "@lucide/svelte/icons/circle-check";
  import Layers from "@lucide/svelte/icons/layers";
  import RefreshCw from "@lucide/svelte/icons/refresh-cw";
  import Combine from "@lucide/svelte/icons/combine";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import GitMerge from "@lucide/svelte/icons/git-merge";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  let {
    controller,
    branch,
    footer = false,
  }: {
    controller: ForgeController;
    branch?: string;
    footer?: boolean;
  } = $props();
  const method = $derived(controller.mergeMethod);
  let mergeConfirm = $state(false);
  const fullMethodLabel = (value: string) =>
    value === "merge"
      ? "Create a merge commit"
      : value === "squash"
        ? "Squash and merge"
        : "Rebase and merge";
  const selected = $derived(controller.selected);
  const pr = $derived(
    branch
      ? selected?.head === branch
        ? selected
        : controller.pr(branch)
      : selected,
  );
  const merge = $derived(pr ? controller.merges[String(pr.number)] : undefined);
  const blocked = $derived(
    controller.busy ||
      Boolean(controller.pending) ||
      (controller.accounts.managed && !controller.ready),
  );
  const methods = $derived(controller.data?.capabilities.merge_methods ?? []);
  const methodLabel = (value: string) =>
    value === "merge" ? "Merge" : value === "squash" ? "Squash" : "Rebase";
  const mergeDisabled = $derived(
    blocked ||
      (merge &&
        ["pending", "accepted", "enqueued", "queued", "in_progress"].includes(
          merge.state,
        )) ||
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
    if (!methods.includes(method)) controller.mergeMethod = methods[0] ?? "";
  });
  function select() {
    if (pr && selected?.number !== pr.number)
      controller.select(pr.head, branch ? "inline" : "column");
  }
</script>

{#snippet methodOptions()}
  <Menu.Group>
    <Menu.Label>Merge method</Menu.Label>
    <Menu.RadioGroup
      value={method}
      onValueChange={(value) => (controller.mergeMethod = value)}
    >
      {#each methods as option}
        <Menu.RadioItem
          value={option}
          disabled={blocked || !controller.data?.capabilities.merge}
        >
          {#if option === "merge"}<GitMerge
              aria-hidden="true"
            />{:else if option === "squash"}<Combine
              aria-hidden="true"
            />{:else}<GitBranch aria-hidden="true" />{/if}
          {footer ? fullMethodLabel(option) : methodLabel(option)}
        </Menu.RadioItem>
      {/each}
    </Menu.RadioGroup>
  </Menu.Group>
{/snippet}
{#if pr}
  {#if footer}
    <footer
      class="forge-merge"
      role="group"
      aria-label="Pull request merge actions"
    >
      <div class="forge-merge__status">
        {#if pr.mergeable === true}<CircleCheck
            aria-hidden="true"
          />{:else if pr.mergeable === false}<CircleX
            aria-hidden="true"
          />{:else}<CircleHelp aria-hidden="true" />{/if}
        <div>
          <strong
            >{pr.state === "merged"
              ? "Pull request merged"
              : pr.state !== "open"
                ? "Pull request closed"
                : pr.draft
                  ? "Draft pull request"
                  : pr.mergeable === true
                    ? "No conflicts with base branch"
                    : pr.mergeable === false
                      ? "Conflicts with base branch"
                      : "Mergeability needs refresh"}</strong
          >
          <span
            >{pr.draft
              ? "Mark ready for review before merging."
              : "Forge protection and stack order are checked before merging."}</span
          >
        </div>
      </div>
      {#if pr.state === "open"}
        <div class="forge-merge__bar">
          <div class="forge-merge__buttons">
            <Button
              variant="outline"
              size="sm"
              disabled={mergeDisabled}
              onclick={() => {
                select();
                mergeConfirm = true;
              }}
              ><GitMerge data-icon="inline-start" />{!method
                ? "Merge unavailable"
                : method === "merge"
                  ? "Merge pull request"
                  : fullMethodLabel(method)}</Button
            >
            <Menu.Root
              ><Menu.Trigger
                >{#snippet child({ props })}<Button
                    {...props}
                    variant="outline"
                    size="icon-sm"
                    disabled={blocked ||
                      !methods.length ||
                      !controller.data?.capabilities.merge}
                    aria-label="Choose merge method"
                    ><ChevronDown aria-hidden="true" /></Button
                  >{/snippet}</Menu.Trigger
              >
              <Menu.Content
                align="start"
                preventScroll={false}
                style={forgeMenuLayout}>{@render methodOptions()}</Menu.Content
              >
            </Menu.Root>
          </div>
          {#if merge?.receipt}<Button
              variant="ghost"
              size="sm"
              disabled={blocked}
              onclick={() =>
                controller.act({
                  kind: "merge_status",
                  receipt: merge.receipt!,
                })}
              ><RefreshCw data-icon="inline-start" />Check merge result</Button
            >{/if}
        </div>
      {/if}
    </footer>
  {:else}
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
            <Ellipsis aria-hidden="true" />
          </Button>
        {/snippet}
      </Menu.Trigger>
      <Menu.Content align="end" preventScroll={false} style={forgeMenuLayout}>
        <Menu.Group>
          <Menu.Label>Pull request actions</Menu.Label>
          <Menu.Item disabled={blocked} onSelect={() => controller.refresh()}
            ><RefreshCw aria-hidden="true" />Refresh pull request</Menu.Item
          >
          {#if controller.data?.capabilities.draft}
            <Menu.Item
              disabled={blocked || pr.state !== "open"}
              onSelect={() => {
                select();
                void controller.act({ kind: "draft", draft: !pr.draft });
              }}
              >{#if pr.draft}<CircleCheck
                  aria-hidden="true"
                />{:else}<FilePenLine aria-hidden="true" />{/if}{pr.draft
                ? "Ready for review"
                : "Convert to draft"}</Menu.Item
            >
          {/if}
          <Menu.Item
            disabled={blocked}
            onSelect={() => {
              select();
              void controller.previewRestack(pr.head);
            }}><Layers aria-hidden="true" />Restack…</Menu.Item
          >
          <Menu.Item
            disabled={mergeDisabled}
            onSelect={() => {
              select();
              mergeConfirm = true;
            }}
            ><GitMerge aria-hidden="true" />{pr.native_stack
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
                }}><RefreshCw aria-hidden="true" />Check merge result</Menu.Item
              >{/if}
          {/if}
        </Menu.Group>
        {#if pr.state === "open"}
          <Menu.Separator />
          {@render methodOptions()}
        {/if}
      </Menu.Content>
    </Menu.Root>
  {/if}
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
