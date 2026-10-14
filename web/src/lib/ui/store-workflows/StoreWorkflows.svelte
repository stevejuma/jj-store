<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import "../store-work/community-work-layout.css";
  import "../store-work/community-work-tasks.css";
  import "../store-work/task-detail-layout.css";
  import "./workflow-history.css";
  import { provideProfiles } from "../../profiles/context.svelte.js";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import StoreNotificationHost from "../store-notifications/StoreNotificationHost.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import {
    validateWorkflowHome,
    workflowDiscoveryPending,
    workflowCatalogLoading,
  } from "./workflow-home.js";
  import WorkflowFeedback from "./WorkflowFeedback.svelte";
  import WorkflowEditor from "./WorkflowEditor.svelte";
  import WorkflowHistory from "./WorkflowHistory.svelte";
  import WorkflowPageSkeleton from "./WorkflowPageSkeleton.svelte";
  import WorkflowOverview from "./WorkflowOverview.svelte";
  import WorkflowNavigation from "./WorkflowNavigation.svelte";
  import {
    workflowsRoute,
    workflowsRouteToHash,
    type WorkflowSelection,
  } from "../../controller/workflows-route-codec.js";
  import WorkflowRunDialog from "./WorkflowRunDialog.svelte";
  import {
    workflowAuthors,
    initialDefinition,
    initialDraft,
    type WorkflowDraft,
  } from "./workflow-presentation.js";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  import { createWorkflowDirectory } from "../../controller/workflow-directory.svelte.js";
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
  const discovering = $derived(
    !homes.length && workflowDiscoveryPending(controller),
  );
  const sources = $derived(
    controller.state.repositories.filter((r) => r.profile !== "git_sha1_jj045"),
  );
  const sourceName = (id: string) =>
    sources.find((r) => r.id === id)?.name ?? id.slice(0, 8);
  let preparing = $state(false);
  let home = $state(""),
    editor = $state(false),
    editorEpoch = $state(0),
    editing = $state<WorkflowVersion | null>(null),
    draft = $state<WorkflowDraft>(initialDraft(initialDefinition())),
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
  const controllers = createWorkflowDirectory(
      () => homes.map((r) => r.id),
      (id) => controller.projects?.transport(id).workflows,
    ),
    drafts = new Map<string, WorkflowDraft>();
  const work = $derived(controllers.get(home));
  const catalogLoading = $derived(workflowCatalogLoading(controllers));
  const rows = $derived(
    [...controllers.entries()].flatMap(([repository, owner]) =>
      (owner.state.page?.workflows ?? []).map((version) => ({
        home: repository,
        work: owner,
        workflow: version,
      })),
    ),
  );
  $effect(() => {
    if (!home && homes.length) home = homes[0]!.id;
  });
  const selection = $derived(controller.state.route.workflow ?? {});
  const visible = $derived(
    rows.filter(
      (entry) =>
        (!selection.scope || entry.home === selection.scope) &&
        (!selection.source ||
          entry.workflow.definition.repository === selection.source) &&
        `${entry.workflow.definition.name} ${sourceName(entry.workflow.definition.repository)}`
          .toLocaleLowerCase()
          .includes(selection.q?.trim().toLocaleLowerCase() ?? ""),
    ),
  );
  function navigate(fields: WorkflowSelection) {
    const next = workflowsRoute({ ...selection, ...fields });
    if (!workflowsRouteToHash(next)) {
      notifications.context.service.publish({
        scope: notifications.context.scope,
        key: "workflow-route",
        title: "Workflow search unavailable",
        message:
          "Search is limited to 512 UTF-8 bytes without control characters.",
        severity: "error",
        lifetime: "dismiss",
      });
      return;
    }
    notifications.context.service.resolve(
      notifications.context.scope,
      "workflow-route",
    );
    void controller.actions.openRoute?.(next);
  }
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
    const result = await owner?.submit({
      type: "remove",
      workflow: version.id,
      expected: version.version,
    });
    return result ?? false;
  }
</script>

<section
  class="store-workflows community-projects"
  aria-label="Workflows"
  data-run-detail={selection.run ? "" : undefined}
>
  <WorkflowFeedback owners={[...controllers.entries()]} />
  {#if !selection.run && selection.tab !== "history"}<WorkflowNavigation
      {selection}
      onChange={navigate}
    />{/if}
  {#if discovering}<WorkflowPageSkeleton
      view={selection.run ? "detail" : (selection.tab ?? "workflows")}
      {selection}
      onChange={navigate}
    />
  {:else if !homes.length}<StoreWorkEmpty
      kind="repositories"
      title="Choose a native metadata home"
      description="Register a native repository and create its WorkStore project before saving workflows."
    />
  {:else}
    {#if selection.tab === "history"}<WorkflowHistory
        entries={rows}
        owners={[...controllers.entries()]}
        {selection}
        {catalogLoading}
        repositoryName={sourceName}
        {canWrite}
        onWorkflow={(row) => open(row.workflow, false, row.home)}
        onChange={navigate}
      />
    {:else}<WorkflowOverview
        entries={visible}
        loading={catalogLoading}
        available={!!work}
        {writable}
        pending={!!work?.state.pending}
        repositoryName={sourceName}
        {canWrite}
        onOpen={(entry, duplicate) =>
          open(entry?.workflow ?? null, duplicate, entry?.home ?? home)}
        onToggle={(entry) => toggle(entry.workflow, entry.work)}
        onRemove={(entry) => void remove(entry.workflow, entry.work)}
        onRun={(entry) => {
          home = entry.home;
          running = entry.workflow;
        }}
      />{/if}
    {#if !selection.run && selection.tab !== "history"}{#each [...controllers.entries()].filter(([, owner]) => owner.state.page?.next) as [repository, owner] (repository)}<Button
          variant="outline"
          disabled={owner.state.loading}
          onclick={() => owner.more()}
          >More workflows in {sourceName(repository)}</Button
        >{/each}{/if}
  {/if}
</section>
<StoreNotificationHost owner={notifications} />
{#if editor}{#key editorEpoch}<WorkflowEditor
      {controller}
      acquireRepository={controller.acquireRepository}
      initial={draft}
      bind:home
      {homes}
      repositories={sources}
      version={editing}
      authors={workflowAuthors(controller)}
      busy={preparing || !!work?.state.writing}
      pending={work?.state.pending?.request_id ?? ""}
      onRetry={() => work?.retry()}
      {writable}
      onSave={save}
      onClose={(next) => {
        drafts.set(`${home}:${editing?.id ?? "new"}`, next);
        editor = false;
      }}
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
{#if running && work}<WorkflowRunDialog
    acquireRepository={controller.acquireRepository}
    repositoryName={sourceName(running.definition.repository)}
    {work}
    workflow={running}
    onClose={() => (running = null)}
  />{/if}
