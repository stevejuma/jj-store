<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ArrowLeft from "@lucide/svelte/icons/arrow-left";
  import History from "@lucide/svelte/icons/history";
  import FolderGit from "@lucide/svelte/icons/folder-git-2";
  import GitCommit from "@lucide/svelte/icons/git-commit-horizontal";
  import Bot from "@lucide/svelte/icons/bot";
  import CommunityMemberHoverCard from "../community-member-hover-card/CommunityMemberHoverCard.svelte";
  import { workMember } from "../store-work/work-presentation.js";
  import { useProfileMembers } from "../../profiles/context.svelte.js";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
  import WorkflowRunHeader from "./WorkflowRunHeader.svelte";
  import WorkflowRunSections from "./WorkflowRunSections.svelte";
  import CommunityProjectsRevisionGraphIds from "../community-projects/CommunityProjectsRevisionGraphIds.svelte";
  import StoreIdentityCopy from "../store-collaboration/StoreIdentityCopy.svelte";
  import type { WorkflowPage } from "../../controller/workflow-types.js";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  let {
    page,
    work,
    selection,
    writable,
    loading,
    repositoryName,
    onChange,
    onClose,
    onRefresh,
    onMore,
  }: {
    page: WorkflowPage;
    work: WorkflowController;
    selection: WorkflowSelection;
    writable: boolean;
    loading: boolean;
    repositoryName(id: string): string;
    onChange(fields: WorkflowSelection): void;
    onClose(): void;
    onRefresh(): void;
    onMore(): void;
  } = $props();
  const run = $derived(page.runs[0]),
    definition = $derived(page.workflows[0]);
  const authors = useProfileMembers(() =>
    run ? [workMember(run.requester)] : [],
  );
  const author = $derived(authors.values[0]);
  const attempt = $derived(
    selection.attempt
      ? run?.attempts.find((row) => row.id === selection.attempt)
      : run?.attempts.at(-1),
  );
  const terminal = $derived(
    !!attempt && ["completed", "failed", "cancelled"].includes(attempt.status),
  );
  async function command(kind: "cancel" | "retry") {
    if (!run || !attempt) return;
    const saved = await work.submit(
      kind === "cancel"
        ? { type: "cancel", run: run.id }
        : { type: "retry", run: run.id, expected: attempt.id },
    );
    if (saved) {
      onChange({ at: undefined, attempt: undefined });
      onRefresh();
    }
  }
</script>

{#if run && attempt}
  <article
    class="community-projects__task-detail workflow-run-detail"
    aria-label="Run details"
  >
    <WorkflowRunHeader
      name={definition?.definition.name ?? "Workflow run"}
      id={run.id}
      created={Number(run.created_at)}
      pinned={selection.at !== undefined}
      {loading}
      action={writable && !selection.attempt && attempt.status !== "unknown"
        ? terminal
          ? "retry"
          : "cancel"
        : undefined}
      disabled={work.state.writing ||
        !!work.state.pending ||
        attempt.status === "cancel_requested"}
      {onClose}
      {onRefresh}
      onPin={() =>
        onChange(
          selection.at !== undefined
            ? { at: undefined, attempt: undefined }
            : { at: String(page.position) },
        )}
      onCommand={() => command(terminal ? "retry" : "cancel")}
    />
    <div class="community-projects__task-body">
      <dl class="community-projects__task-meta">
        <div>
          <dt><Bot aria-hidden="true" />Author</dt>
          <dd>
            <CommunityMemberHoverCard
              member={author ?? workMember(run.requester)}
              trigger="mention"
              triggerLabel={author?.name ?? workMember(run.requester).name}
            />
          </dd>
        </div>
        <div>
          <dt><History aria-hidden="true" />Status</dt>
          <dd><WorkflowRunStatus status={attempt.status} label /></dd>
        </div>
        <div>
          <dt><FolderGit aria-hidden="true" />Repository</dt>
          <dd title={run.source.repository}>
            {repositoryName(run.source.repository)}
          </dd>
        </div>
        <div>
          <dt><Bot aria-hidden="true" />Host</dt>
          <dd>{attempt.host ?? "Waiting for an eligible host"}</dd>
        </div>
        <div>
          <dt><History aria-hidden="true" />Definition</dt>
          <dd><code>{run.version}</code></dd>
        </div>
        {#if run.source.conversation}
          <div>
            <dt><History aria-hidden="true" />Thread</dt>
            <dd><code>{run.source.conversation.thread}</code></dd>
          </div>
          <div>
            <dt><History aria-hidden="true" />Revision</dt>
            <dd><code>{run.source.conversation.revision.id}</code></dd>
          </div>
          <div>
            <dt><History aria-hidden="true" />Publication</dt>
            <dd>{run.source.conversation.position}</dd>
          </div>
        {:else}
          <div>
            <dt><History aria-hidden="true" />Operation</dt>
            <dd>
              <StoreIdentityCopy
                value={run.source.operation ?? ""}
                label="operation ID"
              />
            </dd>
          </div>
          <div>
            <dt><GitCommit aria-hidden="true" />Commit</dt>
            <dd>
              <CommunityProjectsRevisionGraphIds
                commitId={run.source.commit ?? ""}
                placement="inline"
              />
            </dd>
          </div>
          {#if run.source.bookmark}<div>
              <dt><FolderGit aria-hidden="true" />Bookmark</dt>
              <dd>{run.source.bookmark}</dd>
            </div>{/if}
        {/if}
      </dl>
      <WorkflowRunSections
        {run}
        {page}
        {selection}
        {loading}
        {onChange}
        {onMore}
      />
    </div>
  </article>
{:else}<Button variant="ghost" onclick={onClose}
    ><ArrowLeft aria-hidden="true" />Back to history</Button
  ><StoreWorkEmpty
    kind="activity"
    title="Run unavailable at this publication"
    description="Return to history or follow a current run selection."
  />{/if}
