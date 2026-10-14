<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ListFilter from "@lucide/svelte/icons/list-filter";
  import User from "@lucide/svelte/icons/user";
  import Zap from "@lucide/svelte/icons/zap";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import X from "@lucide/svelte/icons/x";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import CommunityMemberAvatar from "../community-member-avatar/CommunityMemberAvatar.svelte";
  import { useProfileMembers } from "../../profiles/context.svelte.js";
  import { workMember } from "../store-work/work-presentation.js";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
  import {
    workflowStates,
    type WorkflowSelection,
  } from "../../controller/workflows-route-codec.js";
  import type { WorkflowHistoryRow } from "./workflow-history-presentation.js";
  let {
    selection,
    rows,
    onChange,
  }: {
    selection: WorkflowSelection;
    rows: readonly WorkflowHistoryRow[];
    onChange(fields: WorkflowSelection): void;
  } = $props();
  const profiles = useProfileMembers(() =>
    [
      ...new Set([
        ...rows.map((row) => row.run.requester),
        ...(selection.author ? [selection.author] : []),
      ]),
    ].map(workMember),
  );
  const bookmarks = $derived(
    [
      ...new Set([
        ...rows.flatMap((row) =>
          row.run.source.bookmark ? [row.run.source.bookmark] : [],
        ),
        ...(selection.bookmark ? [selection.bookmark] : []),
      ]),
    ].sort(),
  );
  const filtered = $derived(
    !!(
      selection.trigger ||
      selection.state ||
      selection.bookmark ||
      selection.author
    ),
  );
</script>

<div class="workflow-history-filters" aria-label="History filters">
  <StoreRepositoryPicker
    label="Run trigger filter"
    repositories={[
      { id: "manual", name: "Manual", description: "Manually requested runs" },
      {
        id: "event",
        name: "Repository event",
        description: "Runs requested by recorded events",
      },
    ]}
    value={selection.trigger ?? ""}
    controlled
    allowEmpty
    variant="ghost"
    emptyLabel="Trigger"
    searchLabel="Find a trigger"
    groupLabel="Triggers"
    unavailableLabel="Selected trigger"
    onChange={(trigger) =>
      onChange({
        trigger: (trigger as WorkflowSelection["trigger"]) || undefined,
      })}
  >
    {#snippet triggerIcon()}<Zap aria-hidden="true" />{/snippet}
    {#snippet itemIcon()}<Zap aria-hidden="true" />{/snippet}
  </StoreRepositoryPicker>
  <StoreRepositoryPicker
    label="Run status filter"
    repositories={workflowStates.map((id) => ({
      id,
      name: id.replaceAll("_", " "),
      description: "Run status",
    }))}
    value={selection.state ?? ""}
    controlled
    allowEmpty
    variant="ghost"
    emptyLabel="Status"
    searchLabel="Find a status"
    groupLabel="Statuses"
    unavailableLabel="Selected status"
    keepItemIcon
    onChange={(state) =>
      onChange({ state: (state as WorkflowSelection["state"]) || undefined })}
  >
    {#snippet triggerIcon()}{#if selection.state}<WorkflowRunStatus
          status={selection.state}
        />{:else}<ListFilter aria-hidden="true" />{/if}{/snippet}
    {#snippet itemIcon(id)}<WorkflowRunStatus status={id} />{/snippet}
  </StoreRepositoryPicker>
  <StoreRepositoryPicker
    label="Run bookmark filter"
    repositories={bookmarks.map((id) => ({
      id,
      name: id,
      description: "Recorded source bookmark",
    }))}
    value={selection.bookmark ?? ""}
    controlled
    allowEmpty
    variant="ghost"
    emptyLabel="Bookmark"
    searchLabel="Find a bookmark"
    groupLabel="Bookmarks"
    emptyResult="No bookmarks in the loaded runs."
    unavailableLabel="Selected bookmark"
    onChange={(bookmark) => onChange({ bookmark: bookmark || undefined })}
  >
    {#snippet triggerIcon()}<GitBranch aria-hidden="true" />{/snippet}
    {#snippet itemIcon()}<GitBranch aria-hidden="true" />{/snippet}
  </StoreRepositoryPicker>
  <StoreRepositoryPicker
    label="Run author filter"
    repositories={profiles.values.map((member) => ({
      id: member.id,
      name: member.name,
      description: member.id,
    }))}
    value={selection.author ?? ""}
    controlled
    allowEmpty
    variant="ghost"
    emptyLabel="Author"
    searchLabel="Find an author"
    groupLabel="Authors"
    emptyResult="No authors in the loaded runs."
    unavailableLabel="Selected author"
    keepItemIcon
    onChange={(author) => onChange({ author: author || undefined })}
  >
    {#snippet triggerIcon()}{#if selection.author}<CommunityMemberAvatar
          member={profiles.values.find(
            (member) => member.id === selection.author,
          ) ?? workMember(selection.author)}
          size="xsmall"
          decorative
        />{:else}<User aria-hidden="true" />{/if}{/snippet}
    {#snippet itemIcon(id)}<CommunityMemberAvatar
        member={profiles.values.find((member) => member.id === id) ??
          workMember(id)}
        size="xsmall"
        decorative
      />{/snippet}
  </StoreRepositoryPicker>
  {#if filtered}<Button
      variant="ghost"
      size="icon-sm"
      aria-label="Clear history filters"
      title="Clear history filters"
      onclick={() =>
        onChange({
          trigger: undefined,
          state: undefined,
          bookmark: undefined,
          author: undefined,
        })}><X aria-hidden="true" /></Button
    >{/if}
</div>
