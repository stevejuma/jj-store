<script lang="ts">
  // CommunityProjects.svelte projectDirectoryCard (Apache-2.0).
  import CommunityMemberHoverCard from "../community-member-hover-card/CommunityMemberHoverCard.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import FolderKanbanIcon from "@lucide/svelte/icons/folder-kanban";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import type { Model } from "../../api/client.js";
  import { workMember } from "./work-presentation.js";
  import "./community-work-rows.css";
  type ProjectRow = Pick<
    Model<"ProjectV1">,
    "id" | "title" | "repositories" | "actor"
  > & {
    body?: string;
    excerpt?: string;
    task_counts?: Model<"WorkProjectSummary">["task_counts"];
  };
  let {
    projects,
    select,
    layout = "list",
  }: {
    layout?: "grid" | "list";
    projects: readonly ProjectRow[];
    select: (project: ProjectRow) => void;
  } = $props();
</script>

<div
  class="community-projects__grid community-projects__project-list"
  data-layout={layout}
  role="list"
  aria-label="Projects"
>
  {#each projects as project (project.id)}
    <div role="listitem">
      <article class="community-projects__card community-projects__project-row">
        <button
          type="button"
          class="community-projects__card-target"
          aria-label={`Open project ${project.title}`}
          onclick={() => select(project)}
        ></button>
        <span class="community-projects__card-icon"
          ><FolderKanbanIcon aria-hidden="true" /></span
        >
        <span class="community-projects__card-copy"
          ><span
            ><strong>{project.title}</strong><span
              class="community-projects__project-description"
              >{project.excerpt ??
                project.body ??
                "No project description"}</span
            ></span
          ></span
        >
        <span class="community-projects__card-meta">
          <span
            class="community-projects__row-count community-projects__project-repositories"
            aria-label={`${Object.values(project.repositories).filter(Boolean).length} repositories`}
            ><GitBranchIcon aria-hidden="true" />{Object.values(
              project.repositories,
            ).filter(Boolean).length}</span
          >
          {#if project.task_counts}<span
              class="community-projects__row-count community-projects__project-counts"
              >{project.task_counts.active} active · {project.task_counts.done} done</span
            >{/if}
          <CommunityMemberHoverCard
            member={workMember(project.actor.created_by)}
            avatarSize="xsmall"
          />
          <CommunityRelativeTime
            timestamp={Number(project.actor.updated_at_millis) / 1000}
          />
        </span>
      </article>
    </div>
  {/each}
</div>
