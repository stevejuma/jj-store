<script lang="ts">
  import { forgeMenuLayout } from "./menu-layout.js";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import ExternalLink from "@lucide/svelte/icons/external-link";
  import ClipboardCheck from "@lucide/svelte/icons/clipboard-check";
  import RefreshCwOff from "@lucide/svelte/icons/refresh-cw-off";
  import FileText from "@lucide/svelte/icons/file-text";
  import RotateCw from "@lucide/svelte/icons/rotate-cw";
  import CircleStop from "@lucide/svelte/icons/circle-stop";
  import StoreForgeCheckIcon from "./StoreForgeCheckIcon.svelte";
  import StoreForgeIcon from "./StoreForgeIcon.svelte";
  import { checkLabel } from "../../controller/forge/check-timing.js";
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
  const runActions = $derived(Boolean(check.run_id && !check.log_id));
</script>

<li class="forge-checks__row">
  <StoreForgeCheckIcon state={check.state} />
  <StoreForgeIcon provider={controller.data?.connection?.provider} />
  <div class="forge-checks__name">
    <strong title={check.name}>{check.name}</strong><span
      title={checkLabel(check)}
      >{checkLabel(check)}{check.sha !== controller.selected?.sha
        ? ` · Revision ${check.sha.slice(0, 8)}`
        : ""}</span
    >
  </div>
  <div class="forge-checks__actions">
    {#if check.log_id || check.run_id || check.url}
      <Menu.Root>
        <Menu.Trigger
          >{#snippet child({ props })}<Button
              {...props}
              variant="ghost"
              size="icon-xs"
              disabled={blocked}
              aria-label={`Actions for ${check.name}`}
              ><Ellipsis aria-hidden="true" /></Button
            >{/snippet}</Menu.Trigger
        >
        <Menu.Content align="end" preventScroll={false} style={forgeMenuLayout}>
          <Menu.Group>
            <Menu.Label>Check actions</Menu.Label>
            {#if check.log_id}
              <Menu.Item
                disabled={!capabilities.logs}
                onSelect={() =>
                  controller.act({
                    kind: "logs",
                    run: check.run_id ?? controller.activeRun,
                    job: check.log_id!,
                  })}><FileText aria-hidden="true" />View logs</Menu.Item
              >
            {:else if check.run_id}
              <Menu.Item
                onSelect={() =>
                  controller.act({ kind: "jobs", run: check.run_id! })}
                ><ClipboardCheck aria-hidden="true" />Jobs</Menu.Item
              >
            {/if}
            {#if check.url}<Menu.Item
                >{#snippet child({ props })}<a
                    {...props}
                    href={check.url}
                    target="_blank"
                    rel="noreferrer"
                    ><ExternalLink aria-hidden="true" />View details</a
                  >{/snippet}</Menu.Item
              >{/if}
            {#if runActions && capabilities.rerun}
              {#if capabilities.rerun_all}<Menu.Item
                  onSelect={() =>
                    controller.act({
                      kind: "rerun",
                      run: check.run_id!,
                      failed_only: false,
                    })}><RotateCw aria-hidden="true" />Rerun all</Menu.Item
                >{/if}
              <Menu.Item
                onSelect={() =>
                  controller.act({
                    kind: "rerun",
                    run: check.run_id!,
                    failed_only: true,
                  })}><RefreshCwOff aria-hidden="true" />Rerun failed</Menu.Item
              >
            {/if}
            {#if runActions && capabilities.cancel && ["queued", "running"].includes(check.state)}<Menu.Item
                onSelect={() =>
                  controller.act({ kind: "cancel", run: check.run_id! })}
                ><CircleStop aria-hidden="true" />Cancel run</Menu.Item
              >{/if}
          </Menu.Group>
        </Menu.Content>
      </Menu.Root>
    {/if}
  </div>
</li>
