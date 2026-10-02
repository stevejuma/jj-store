<script lang="ts">
  // @community-copy CommunityProjects.svelte selectedTask detail (Apache-2.0), native metadata.
  import type { Snippet } from "svelte";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";
  import CommunityRelativeTime from "../community-relative-time/CommunityRelativeTime.svelte";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import ChevronDownIcon from "@lucide/svelte/icons/chevron-down";
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import GitBranchIcon from "@lucide/svelte/icons/git-branch";
  import ListTodoIcon from "@lucide/svelte/icons/list-todo";
  import RefreshIcon from "@lucide/svelte/icons/refresh-cw";
  import FolderGit2Icon from "@lucide/svelte/icons/folder-git-2";
  import PencilIcon from "@lucide/svelte/icons/pencil";
  import ListPlusIcon from "@lucide/svelte/icons/list-plus";
  import type { StoreRepositorySummary } from "../../controller/contracts.js";
  import type { WorkController } from "../../controller/work-types.js";
  import StoreWorkComments from "./StoreWorkComments.svelte";
  import StoreWorkTaskStateIcon from "./StoreWorkTaskStateIcon.svelte";
  import { taskStateLabel } from "./work-presentation.js";
  import "./community-work-tasks.css";
  import "./task-detail-layout.css";
  let {
    work,
    repositories,
    writable,
    principal,
    busy = false,
    onEdit,
    onSubtask,
    onClose,
    onSelectTask,
    onOpenRepository,
    onRefresh,
    subtasks,
    viewportRef = $bindable<HTMLElement | null>(null),
  }: {
    work: WorkController;
    repositories: readonly StoreRepositorySummary[];
    writable: boolean;
    principal?: string | undefined;
    busy?: boolean;
    onEdit: () => void;
    onSubtask?: () => void;
    onClose: () => void;
    onSelectTask: (id: string) => void;
    onOpenRepository?: (id: string) => void | Promise<void>;
    onRefresh?: () => void | Promise<void>;
    subtasks?: Snippet;
    viewportRef?: HTMLElement | null;
  } = $props();
  const task = $derived(work.state.task);
  const disabled = $derived(
    busy || work.state.loading || work.state.writing || !!work.state.pending,
  );
  let descriptionOpen = $state(true);
  const target = $derived(repositories.find((r) => r.id === task?.repository));
</script>

{#if task}<article
    class="community-projects__task-detail"
    aria-label="Task details"
  >
    <header class="community-projects__task-detail-header">
      <Button
        type="button"
        variant="ghost"
        size="icon-sm"
        class="community-projects__detail-back"
        aria-label="Back to tasks"
        title="Back to tasks"
        onclick={onClose}><ArrowLeftIcon aria-hidden="true" /></Button
      >
      <div>
        <h2>
          {task.title} <span title={task.id}>#{task.id.slice(0, 8)}</span>
        </h2>
        <p>
          Task created <CommunityRelativeTime
            timestamp={Number(task.actor.created_at_millis) / 1000}
          />
        </p>
      </div>
      {#if onRefresh}<Button
          variant="ghost"
          size="icon-sm"
          aria-label="Refresh projects"
          title="Refresh projects"
          disabled={busy || work.state.loading}
          onclick={onRefresh}><RefreshIcon aria-hidden="true" /></Button
        >{/if}
    </header>
    {#key task.id}<StoreWorkComments {work} {writable} {principal} {busy}>
        {#snippet children(activity, composer)}
          <ScrollArea
            class="community-projects__task-scroll"
            type="hover"
            role="region"
            aria-label="Task content"
            bind:viewportRef
          >
            <div class="community-projects__task-body">
              <dl class="community-projects__task-meta">
                <div>
                  <dt><StoreWorkTaskStateIcon state={task.state} />Status</dt>
                  <dd>{taskStateLabel(task.state)}</dd>
                </div>
                <div>
                  <dt><GitBranchIcon aria-hidden="true" />Repository</dt>
                  <dd>
                    {#if task.repository && target && onOpenRepository}<Button
                        variant="secondary"
                        size="sm"
                        class="community-projects__task-target"
                        onclick={() => onOpenRepository?.(task.repository!)}
                        ><FolderGit2Icon
                          data-icon="inline-start"
                          aria-hidden="true"
                        />Open target repository</Button
                      ><span>{target.name}</span>
                    {:else if task.repository}<span title={task.repository}
                        >Separate access required for the target repository.</span
                      >
                    {:else}Project only{/if}
                  </dd>
                </div>
                {#if task.parent}<div>
                    <dt><ListTodoIcon aria-hidden="true" />Parent</dt>
                    <dd>
                      <Button
                        variant="ghost"
                        size="sm"
                        onclick={() => onSelectTask(task.parent!)}
                        >Parent: {work.state.tasks.find(
                          (t) => t.id === task.parent,
                        )?.title ?? task.parent.slice(0, 8)}</Button
                      >
                    </dd>
                  </div>{/if}
                {#if writable}<div>
                    <dt><ListTodoIcon aria-hidden="true" />Actions</dt>
                    <dd class="community-projects__task-assignees">
                      <Button
                        variant="secondary"
                        size="sm"
                        {disabled}
                        onclick={onEdit}
                        ><PencilIcon
                          data-icon="inline-start"
                          aria-hidden="true"
                        />Edit task</Button
                      >{#if onSubtask}<Button
                          variant="secondary"
                          size="sm"
                          {disabled}
                          onclick={onSubtask}
                          ><ListPlusIcon
                            data-icon="inline-start"
                            aria-hidden="true"
                          />Add subtask</Button
                        >{/if}
                    </dd>
                  </div>{/if}
              </dl>
              {#if task.body}<Collapsible.Root
                  class="community-projects__task-description"
                  bind:open={descriptionOpen}
                >
                  <Collapsible.Trigger
                    class="community-projects__task-section-trigger"
                    ><span>Description</span
                    >{#if descriptionOpen}<ChevronDownIcon
                        class="community-projects__task-section-chevron"
                        aria-hidden="true"
                      />{:else}<ChevronRightIcon
                        class="community-projects__task-section-chevron"
                        aria-hidden="true"
                      />{/if}</Collapsible.Trigger
                  >
                  <Collapsible.Content
                    class="community-projects__task-description-content"
                    ><CommunityMarkdownContent
                      fullWidth
                      content={task.body}
                    /></Collapsible.Content
                  >
                </Collapsible.Root>{/if}
              {@render subtasks?.()}
              {@render activity()}
            </div>
          </ScrollArea>
          {#if writable}<footer
              class="community-projects__task-footer"
              aria-label="Task comment composer"
            >
              {@render composer()}
            </footer>{/if}
        {/snippet}
      </StoreWorkComments>{/key}
  </article>{/if}
