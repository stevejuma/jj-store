<script lang="ts">
  import CommunityMemberHoverCard from "../community-member-hover-card/CommunityMemberHoverCard.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import { workMember } from "../store-work/work-presentation.js";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
  import type {
    WorkflowVersion,
    WorkflowRun,
  } from "../../controller/workflow-types.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  let {
    rows,
    repositoryName,
    onChange,
  }: {
    rows: readonly {
      home: string;
      workflow: WorkflowVersion;
      run: WorkflowRun;
      position?: number | bigint;
    }[];
    repositoryName(id: string): string;
    onChange(fields: WorkflowSelection): void;
  } = $props();
</script>

<div
  class="community-projects__task-list"
  role="list"
  aria-label="Workflow runs"
>
  {#each rows as { home, workflow, run, position } (`${home}:${run.id}`)}{@const current =
      run.attempts.at(-1)!}
    <div role="listitem">
      <article class="community-projects__task-row workflow-history-row">
        <button
          class="community-projects__task-row-target"
          aria-label={`Open run ${workflow.definition.name} ${run.id.slice(0, 8)}`}
          onclick={() =>
            onChange({
              home,
              run: run.id,
              at:
                ["completed", "failed", "cancelled", "skipped"].includes(
                  current.status,
                ) && position !== undefined
                  ? String(position)
                  : undefined,
              attempt: undefined,
            })}
        ></button>
        <WorkflowRunStatus status={current.status} /><strong
          title={`${run.id} · ${repositoryName(home)}`}
          >{workflow.definition.name} <span>#{run.id.slice(0, 8)}</span></strong
        >
        <span class="community-projects__task-repository"
          >{repositoryName(run.source.repository)}</span
        >
        <span class="community-projects__task-row-assignees"
          ><CommunityMemberHoverCard
            member={workMember(run.requester)}
            avatarSize="xsmall"
          /></span
        >
        <span class="workflow-history-state"
          >{current.status.replaceAll("_", " ")}</span
        >
        <CommunityRelativeTime timestamp={Number(run.created_at) / 1000} />
      </article>
    </div>
  {/each}
</div>
