<script lang="ts">
  import * as Card from "@lapismd/design-core/shadcn/card";
  import { Checkbox } from "@lapismd/design-core/shadcn/checkbox";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ArrowLeftRight from "@lucide/svelte/icons/arrow-left-right";
  import CommunityCommitDiff from "../community-commit-diff/CommunityCommitDiff.svelte";
  import type { WorkspaceDragSession } from "../../controller/stack-workspace/drag-session.svelte.js";
  import type {
    WorkspaceHunk,
    WorkspaceSelection,
  } from "../../controller/stack-workspace/types.js";
  let {
    hunk,
    drag,
    path,
    origin,
    selection,
    checked,
    disabled,
    selectable,
    viewMode,
    wrap,
    onChecked,
    onAssign,
  }: {
    hunk: WorkspaceHunk;
    drag: WorkspaceDragSession;
    path: string;
    origin: string;
    selection: WorkspaceSelection | null;
    checked: boolean;
    disabled: boolean;
    selectable: boolean;
    viewMode: "unified" | "split";
    wrap: boolean;
    onChecked: (checked: boolean) => void;
    onAssign: (selection: WorkspaceSelection) => void;
  } = $props();
  const id = $props.id();
  const header = $derived(
    hunk.patch.split(/\r?\n/).find((line) => line.startsWith("@@ ")) ??
      hunk.label,
  );
</script>

<Card.Root
  class="stack-workspace__hunk-card"
  role="group"
  aria-label={`Draggable hunk ${hunk.label}`}
  draggable={Boolean(selection) && !disabled}
  ondragstart={(event) => {
    if (!selection || disabled) {
      event.preventDefault();
      return;
    }
    drag.start(event, { kind: "selection", selection });
  }}
>
  <Card.Header class="stack-workspace__hunk-header">
    {#if selectable}
      <Checkbox
        {id}
        {checked}
        aria-label={hunk.label}
        disabled={disabled || !selection}
        onCheckedChange={(next) => onChecked(next === true)}
      />
      <label for={id} title={hunk.label}><code>{header}</code></label>
    {:else}<Card.Title title={hunk.label}><code>{header}</code></Card.Title
      >{/if}
    {#if selection}<Button
        variant="ghost"
        size="icon-sm"
        {disabled}
        aria-label={`Assign hunk ${hunk.label}`}
        title="Assign hunk"
        onclick={() => selection && onAssign(selection)}
        ><ArrowLeftRight aria-hidden="true" /></Button
      >{/if}
  </Card.Header>
  <Card.Content class="stack-workspace__hunk-content">
    <CommunityCommitDiff
      single
      label={`${origin} · ${path} · ${hunk.label}`}
      scrollStrategy="styled"
      files={[
        {
          path,
          patch: hunk.patch.startsWith("diff --git ")
            ? hunk.patch
            : `diff --git a/${path} b/${path}\n--- a/${path}\n+++ b/${path}\n${hunk.patch}`,
        },
      ]}
      {viewMode}
      {wrap}
    />
  </Card.Content>
</Card.Root>
