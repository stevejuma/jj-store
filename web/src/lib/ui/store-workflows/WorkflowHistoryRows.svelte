<script lang="ts">
  import { onMount } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Badge } from "@lapismd/design-core/shadcn/badge";
  import * as DropdownMenu from "@lapismd/design-core/shadcn/dropdown-menu";
  import Calendar from "@lucide/svelte/icons/calendar-days";
  import Timer from "@lucide/svelte/icons/timer";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Ellipsis from "@lucide/svelte/icons/ellipsis";
  import File from "@lucide/svelte/icons/file-text";
  import List from "@lucide/svelte/icons/list-tree";
  import CommunityMemberHoverCard from "../community-member-hover-card/CommunityMemberHoverCard.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import { useProfileMembers } from "../../profiles/context.svelte.js";
  import { workMember } from "../store-work/work-presentation.js";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
  import {
    runDuration,
    workflowRunSelection,
    type WorkflowHistoryRow,
  } from "./workflow-history-presentation.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  let {
    rows,
    repositoryName,
    onChange,
    onWorkflow,
  }: {
    rows: readonly WorkflowHistoryRow[];
    repositoryName(id: string): string;
    onChange(fields: WorkflowSelection): void;
    onWorkflow(row: WorkflowHistoryRow): void;
  } = $props();
  const authors = useProfileMembers(() =>
    [...new Set(rows.map((row) => row.run.requester))].map(workMember),
  );
  let now = $state(Date.now());
  onMount(() => {
    const timer = setInterval(() => {
      now = Date.now();
    }, 1000);
    return () => clearInterval(timer);
  });
</script>

<div role="list" aria-label="Workflow runs" class="workflow-history-rows">
  {#each rows as row (`${row.home}:${row.run.id}`)}
    {@const { workflow, run } = row}
    {@const author =
      authors.values.find((member) => member.id === run.requester) ??
      workMember(run.requester)}
    <div role="listitem">
      <article class="workflow-history-row">
        <button
          class="workflow-history-row__target"
          aria-label={`Open run ${workflow.definition.name} ${run.id.slice(0, 8)}`}
          onclick={() => onChange(workflowRunSelection(row))}
        ></button>
        <WorkflowRunStatus status={run.attempts.at(-1)?.status ?? "unknown"} />
        <CommunityMemberHoverCard
          member={author}
          avatarSize="small"
          trigger="avatar"
        />
        <div class="workflow-history-row__summary">
          <strong>{workflow.definition.name}</strong>
          <div class="workflow-history-row__context">
            <span title={`${run.id} · ${repositoryName(row.home)}`}
              >#{run.id.slice(0, 8)}:</span
            >
            <span
              >{run.event === null ? "Manually run" : "Event-triggered"}</span
            >
          </div>
        </div>
        <div
          class="workflow-history-row__source"
          title={repositoryName(run.source.repository)}
        >
          {#if run.source.bookmark}<Badge
              variant="secondary"
              class="workflow-history-row__bookmark"
              ><GitBranch aria-hidden="true" />{run.source.bookmark}</Badge
            >{:else}<span>{repositoryName(run.source.repository)}</span>{/if}
        </div>
        <div class="workflow-history-row__time">
          <span
            ><Calendar aria-hidden="true" /><CommunityRelativeTime
              timestamp={Number(run.created_at) / 1000}
            /></span
          >
          <span
            ><Timer aria-hidden="true" /><span aria-label="Run duration"
              >{runDuration(run, now)}</span
            ></span
          >
        </div>
        <DropdownMenu.Root>
          <DropdownMenu.Trigger
            >{#snippet child({ props })}<Button
                {...props}
                variant="ghost"
                size="icon-sm"
                class="workflow-history-row__menu"
                aria-label={`Actions for run ${run.id.slice(0, 8)}`}
                ><Ellipsis aria-hidden="true" /></Button
              >{/snippet}</DropdownMenu.Trigger
          >
          <DropdownMenu.Content align="end" class="workflow-history-row-menu">
            <DropdownMenu.Group>
              <DropdownMenu.Item
                onSelect={() => onChange(workflowRunSelection(row))}
                ><List aria-hidden="true" />View run</DropdownMenu.Item
              >
              {#if run.version === workflow.version}<DropdownMenu.Item
                  onSelect={() => onWorkflow(row)}
                  ><File aria-hidden="true" />View workflow definition</DropdownMenu.Item
                >{/if}
            </DropdownMenu.Group>
          </DropdownMenu.Content>
        </DropdownMenu.Root>
      </article>
    </div>
  {/each}
</div>
