<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import ExternalLink from "@lucide/svelte/icons/external-link";
  import StoreForgeCheckIcon from "./StoreForgeCheckIcon.svelte";
  import StoreForgeIcon from "./StoreForgeIcon.svelte";
  import type { ForgeController } from "../../controller/forge/controller.svelte.js";
  import type { ForgeCheck } from "../../controller/forge/types.js";
  let {
    controller,
    check,
  }: { controller: ForgeController; check: ForgeCheck } = $props();
  const blocked = $derived(
    controller.busy ||
      Boolean(controller.pending) ||
      (controller.accounts.managed && !controller.ready),
  );
  const capabilities = $derived(controller.data!.capabilities);
  const label = $derived(
    check.state === "running"
      ? "In progress"
      : check.state === "manual"
        ? "Action required"
        : check.state,
  );
</script>

<li class="forge-checks__row">
  <StoreForgeCheckIcon state={check.state} />
  <StoreForgeIcon provider={controller.data?.connection?.provider} />
  <div class="forge-checks__name">
    <strong>{check.name}</strong><span
      >{label}{check.sha !== controller.selected?.sha
        ? ` · Revision ${check.sha.slice(0, 8)}`
        : ""}</span
    >
  </div>
  <div class="forge-checks__actions">
    {#if check.log_id}
      <Button
        variant="ghost"
        size="xs"
        disabled={blocked || !capabilities.logs}
        onclick={() =>
          controller.act({
            kind: "logs",
            run: check.run_id ?? controller.activeRun,
            job: check.log_id!,
          })}>View logs</Button
      >
    {:else if check.run_id}
      <Button
        variant="ghost"
        size="xs"
        disabled={blocked}
        onclick={() => controller.act({ kind: "jobs", run: check.run_id! })}
        >Jobs</Button
      >
    {/if}
    {#if check.url}<a
        class="forge-checks__link"
        href={check.url}
        target="_blank"
        rel="noreferrer"
        aria-label={`Open ${check.name} on forge`}
        ><ExternalLink size={14} aria-hidden="true" />Details</a
      >{/if}
    {#if check.run_id && !check.log_id && (capabilities.rerun || capabilities.cancel)}
      <Menu.Root>
        <Menu.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-sm"
              disabled={blocked}
              aria-label={`Actions for ${check.name}`}
              ><Ellipsis aria-hidden="true" /></Button
            >{/snippet}</Menu.Trigger
        >
        <Menu.Content align="end" preventScroll={false}
          ><Menu.Group>
            <Menu.Label>Run actions</Menu.Label>
            {#if capabilities.rerun}
              {#if capabilities.rerun_all}<Menu.Item
                  onSelect={() =>
                    controller.act({
                      kind: "rerun",
                      run: check.run_id!,
                      failed_only: false,
                    })}>Rerun all</Menu.Item
                >{/if}
              <Menu.Item
                onSelect={() =>
                  controller.act({
                    kind: "rerun",
                    run: check.run_id!,
                    failed_only: true,
                  })}>Rerun failed</Menu.Item
              >
            {/if}
            {#if capabilities.cancel && ["queued", "running"].includes(check.state)}<Menu.Item
                onSelect={() =>
                  controller.act({ kind: "cancel", run: check.run_id! })}
                >Cancel run</Menu.Item
              >{/if}
          </Menu.Group></Menu.Content
        >
      </Menu.Root>
    {/if}
  </div>
</li>
