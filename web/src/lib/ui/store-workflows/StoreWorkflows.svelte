<script lang="ts">
  import { untrack } from "svelte";
  import { provideProfiles } from "../../profiles/context.svelte.js";
  import { SvelteMap } from "svelte/reactivity";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import WorkflowSkeleton from "./WorkflowSkeleton.svelte";
  import Plus from "@lucide/svelte/icons/plus";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import { validateWorkflowHome } from "./workflow-home.js";
  import WorkflowFeedback from "./WorkflowFeedback.svelte";
  import WorkflowCard from "./WorkflowCard.svelte";
  import WorkflowEditor from "./WorkflowEditor.svelte";
  import WorkflowHistory from "./WorkflowHistory.svelte";
  import WorkflowRunDialog from "./WorkflowRunDialog.svelte";
  import {
    workflowAuthors,
    initialDefinition,
    initialDraft,
    type WorkflowDraft,
  } from "./workflow-presentation.js";
  import {
    createWorkflows,
    type WorkflowController,
  } from "../../controller/workflows.svelte.js";
  import { workId } from "../../controller/work-types.js";
  import type { StoreController } from "../../controller/contracts.js";
  import type {
    WorkflowDefinition,
    WorkflowVersion,
  } from "../../controller/workflow-types.js";
  import "./workflows.css";
  import "./workflow-editor.css";
  let { controller }: { controller: StoreController } = $props();
  provideProfiles(() => controller.directory);
  const notifications = ownNotifications();
  const homes = $derived(
    controller.state.repositories.filter(
      (r) => r.profile !== "git_sha1_jj045" && r.availability !== "unavailable",
    ),
  );
  const authors = $derived(workflowAuthors(controller));
  const sources = $derived(
    controller.state.repositories.filter((r) => r.profile !== "git_sha1_jj045"),
  );
  let preparing = $state(false);
  let home = $state(""),
    editor = $state(false),
    editorEpoch = $state(0),
    editing = $state<WorkflowVersion | null>(null),
    draft = $state<WorkflowDraft>(initialDraft(initialDefinition())),
    history = $state<WorkflowVersion | null>(null),
    running = $state<WorkflowVersion | null>(null);
  function canWrite(repository: string) {
    return (
      controller.state.accessMode === "local_trusted" ||
      ["write", "admin"].includes(
        homes.find((r) => r.id === repository)?.permission ?? "",
      )
    );
  }
  const writable = $derived(canWrite(home));
  const controllers = new SvelteMap<string, WorkflowController>(),
    drafts = new Map<string, WorkflowDraft>();
  const work = $derived(controllers.get(home));
  const rows = $derived(
    [...controllers.entries()].flatMap(([repository, owner]) =>
      (owner.state.page?.workflows ?? []).map((version) => ({
        repository,
        owner,
        version,
      })),
    ),
  );
  $effect(() => {
    if (!home && homes.length) home = homes[0]!.id;
  });
  $effect(() => {
    const selected = homes.map((r) => r.id);
    if (!controller.projects) return;
    const owners = untrack(() => {
      for (const [repository, owner] of controllers)
        if (!selected.includes(repository)) {
          owner.stop();
          controllers.delete(repository);
        }
      return selected.flatMap((repository) => {
        let owner = controllers.get(repository);
        if (!owner) {
          const transport =
            controller.projects!.transport(repository).workflows;
          if (!transport) return [];
          owner = createWorkflows(transport);
          controllers.set(repository, owner);
        }
        return [owner];
      });
    });
    const abort = new AbortController();
    const refresh = () => {
      if (!abort.signal.aborted)
        for (const owner of owners) void owner.refresh();
    };
    refresh();
    const timer = setInterval(refresh, 5000);
    window.addEventListener("focus", refresh);
    window.addEventListener("online", refresh);
    async function events(owner: WorkflowController) {
      while (!abort.signal.aborted && owner.transport.events) {
        try {
          await owner.transport.events(
            abort.signal,
            () => void owner.refresh(),
          );
        } catch {}
        if (!abort.signal.aborted)
          await new Promise((resolve) => setTimeout(resolve, 1000));
      }
    }
    for (const owner of owners) void events(owner);
    return () => {
      abort.abort();
      clearInterval(timer);
      window.removeEventListener("focus", refresh);
      window.removeEventListener("online", refresh);
      for (const owner of owners) owner.stop();
    };
  });
  function open(
    version: WorkflowVersion | null,
    duplicate = false,
    repository = home,
  ) {
    home = repository;
    editing = duplicate ? null : version;
    const key = editing?.id ?? "new";
    draft = structuredClone(
      $state.snapshot(
        drafts.get(`${home}:${key}`) ??
          initialDraft(
            version
              ? {
                  ...version.definition,
                  ...(duplicate
                    ? {
                        name: `${version.definition.name} copy`,
                        enabled: false,
                      }
                    : {}),
                }
              : initialDefinition(sources[0]?.id ?? ""),
          ),
      ),
    );
    editorEpoch++;
    editor = true;
  }
  async function save(definition: WorkflowDefinition) {
    if (preparing || work?.state.writing || work?.state.pending) return false;
    const owner = work,
      metadataHome = home,
      version = editing;
    preparing = true;
    try {
      if (!work || work.state.pending) return false;
      if (
        !(await validateWorkflowHome(
          controller,
          metadataHome,
          notifications.context,
        ))
      )
        return false;
      return (
        (await owner?.submit({
          type: "put",
          workflow: version?.id ?? workId(),
          expected: version?.version ?? null,
          definition,
        })) ?? false
      );
    } finally {
      preparing = false;
    }
  }
  async function toggle(version: WorkflowVersion, owner = work) {
    await owner?.submit({
      type: "put",
      workflow: version.id,
      expected: version.version,
      definition: {
        ...version.definition,
        enabled: !version.definition.enabled,
      },
    });
  }
  async function remove(version: WorkflowVersion, owner = work) {
    return (
      (await owner?.submit({
        type: "remove",
        workflow: version.id,
        expected: version.version,
      })) ?? false
    );
  }
</script>

<section class="store-workflows" aria-label="Workflows">
  <header class="store-workflows__header">
    <div>
      <h1>Workflows</h1>
      <p>Run commands and agents when repositories change.</p>
    </div>
  </header>
  <WorkflowFeedback owners={[...controllers.entries()]} />
  {#if !homes.length}<StoreWorkEmpty
      kind="repositories"
      title="Choose a native metadata home"
      description="Register a native repository and create its WorkStore project before saving workflows."
    />
  {:else if !work}<StoreWorkEmpty
      title="Workflows unavailable"
      description="This connection does not support native workflow metadata."
    />
  {:else if !work.state.page && work.state.loading && !work.state.readError}<div
      class="workflow-grid"
    >
      <span class="sr-only" role="status">Loading workflows</span>
      {#each [1, 2, 3] as key (key)}<WorkflowSkeleton />{/each}
    </div>
  {:else}<div class="workflow-grid">
      {#each rows as { repository, owner, version } (`${repository}:${version.id}`)}<WorkflowCard
          workflow={version}
          repositoryName={sources.find(
            (r) => r.id === version.definition.repository,
          )?.name ?? version.definition.repository.slice(0, 8)}
          busy={owner.state.writing || !!owner.state.pending}
          writable={canWrite(repository)}
          onOpen={() => open(version, false, repository)}
          onToggle={() => toggle(version, owner)}
          onDuplicate={() => open(version, true, repository)}
          onRemove={() => remove(version, owner)}
          onRun={() => {
            home = repository;
            running = version;
          }}
        />{/each}{#if writable}<button
          class="workflow-create-card"
          disabled={!!work.state.pending}
          onclick={() => open(null)}
          ><span class="workflow-create-card__icon"
            ><Plus aria-hidden="true" /></span
          ><span>Create workflow</span></button
        >{/if}
    </div>{/if}
</section>
<StoreNotificationHost owner={notifications} />
{#if editor}{#key editorEpoch}<WorkflowEditor
      acquireRepository={controller.acquireRepository}
      initial={draft}
      bind:home
      {homes}
      repositories={sources}
      version={editing}
      {authors}
      busy={preparing || !!work?.state.writing}
      pending={work?.state.pending?.request_id ?? ""}
      onRetry={() => work?.retry()}
      {writable}
      onSave={save}
      onClose={(next) => {
        drafts.set(`${home}:${editing?.id ?? "new"}`, next);
        editor = false;
      }}
      onHistory={editing ? () => (history = editing) : undefined}
      onTrigger={editing ? () => (running = editing) : undefined}
      onDelete={editing ? () => remove(editing!) : undefined}
      onDuplicate={(current) => {
        drafts.set(`${home}:${editing?.id ?? "new"}`, current);
        editing = null;
        draft = initialDraft({
          ...current.definition,
          name: `${current.definition.name || "Workflow"} copy`,
          enabled: false,
        });
        editorEpoch++;
      }}
    />{/key}{/if}
{#if history && work}<WorkflowHistory
    {work}
    workflow={history}
    {writable}
    onClose={() => (history = null)}
  />{/if}
{#if running && work}<WorkflowRunDialog
    {work}
    workflow={running}
    onClose={() => (running = null)}
  />{/if}
