<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Menu from "@lapismd/design-core/shadcn/dropdown-menu";
  import Settings from "@lucide/svelte/icons/settings";
  import Download from "@lucide/svelte/icons/download";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import WorkflowRunEvidence from "./WorkflowRunEvidence.svelte";
  import type { WorkflowEvidence } from "../../controller/workflow-types.js";
  let {
    seconds = $bindable(false),
    timestamps = $bindable(false),
    fullscreen = $bindable(false),
    downloading,
    onDownload,
    outputs,
    hasMore,
    loading,
    onMore,
  }: {
    seconds?: boolean;
    timestamps?: boolean;
    fullscreen?: boolean;
    downloading: boolean;
    onDownload(): void;
    outputs: readonly WorkflowEvidence[];
    hasMore: boolean;
    loading: boolean;
    onMore(): void;
  } = $props();
</script>

<Menu.Root>
  <Menu.Trigger
    >{#snippet child({ props })}<Button
        {...props}
        variant="ghost"
        size="icon-sm"
        aria-label="Log settings"><Settings aria-hidden="true" /></Button
      >{/snippet}</Menu.Trigger
  >
  <Menu.Content align="end" class="workflow-console-settings">
    <Menu.Group>
      <Menu.CheckboxItem bind:checked={seconds} closeOnSelect={false}
        >Show seconds</Menu.CheckboxItem
      >
      <Menu.CheckboxItem bind:checked={timestamps} closeOnSelect={false}
        >Show timestamps</Menu.CheckboxItem
      >
      <Menu.CheckboxItem bind:checked={fullscreen}
        >Show full screen</Menu.CheckboxItem
      >
      <Menu.Separator />
      <Menu.Item disabled={downloading} onSelect={onDownload}
        ><Download aria-hidden="true" />{downloading
          ? "Downloading logs…"
          : "Download logs"}</Menu.Item
      >
      {#if outputs.length}<Menu.Separator /><WorkflowRunEvidence
          rows={outputs}
        />{/if}
      {#if hasMore}<Menu.Item disabled={loading} onSelect={onMore}
          ><Refresh aria-hidden="true" />Load more evidence</Menu.Item
        >{/if}
    </Menu.Group>
  </Menu.Content>
</Menu.Root>
