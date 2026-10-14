<script lang="ts">
  import { createInboxResize } from "../community-inbox/inbox-resize.svelte.js";
  import "../community-inbox/inbox.css";
  import { onDestroy } from "svelte";
  import { AppShell, useAppShell } from "@lapismd/design-core/shell";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { ScrollArea } from "@lapismd/design-core/shadcn/scroll-area";
  import CommunityInboxThreadHeader from "../community-inbox/CommunityInboxThreadHeader.svelte";
  import "../community-inbox/inbox-thread.css";
  import WorkflowPageSkeleton from "./WorkflowPageSkeleton.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import WorkflowHistoryRows from "./WorkflowHistoryRows.svelte";
  import WorkflowHistorySidebar from "./WorkflowHistorySidebar.svelte";
  import WorkflowFilters from "./WorkflowFilters.svelte";
  import WorkflowSidebarHeader from "./WorkflowSidebarHeader.svelte";
  import WorkflowCatalogMore from "./WorkflowCatalogMore.svelte";
  import type { WorkflowEntryActions } from "./workflow-actions.js";
  import WorkflowHistoryDetail from "./WorkflowHistoryDetail.svelte";
  import {
    matchesWorkflowRun,
    type WorkflowHistoryRow,
  } from "./workflow-history-presentation.js";
  import {
    createWorkflowHistory,
    type WorkflowHistoryEntry,
  } from "./workflow-history.svelte.js";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  import "./workflow-history-list.css";
  let {
    entries,
    catalogLoading = false,
    owners,
    selection,
    repositoryName,
    canWrite,
    onChange,
    onWorkflow,
    sourceRepository,
    workflowActions,
  }: {
    entries: WorkflowHistoryEntry[];
    catalogLoading?: boolean;
    sourceRepository?: string;
    owners: readonly (readonly [string, WorkflowController])[];
    selection: WorkflowSelection;
    repositoryName(id: string): string;
    canWrite(home: string): boolean;
    onChange(fields: WorkflowSelection): void;
    onWorkflow(row: WorkflowHistoryRow): void;
    workflowActions: WorkflowEntryActions;
  } = $props();
  const shell = useAppShell();
  let sidebarOpen = $state(true);
  let bodyElement = $state<HTMLElement | null>(null),
    sidebarElement = $state<HTMLElement | null>(null);
  const resize = createInboxResize(
    () => bodyElement,
    () => sidebarElement,
  );
  const scoped = $derived(
    entries.filter(
      (entry) =>
        (!selection.scope || entry.home === selection.scope) &&
        (!selection.workflow || entry.workflow.id === selection.workflow),
    ),
  );
  const history = createWorkflowHistory(
    () => scoped,
    () => owners,
    () => selection,
  );
  const owner = $derived(owners.find(([home]) => home === selection.home)?.[1]);
  const loadedRows = $derived(
    history.groups.flatMap((group) =>
      group.rows.map((run) => ({
        home: group.home,
        workflow: group.workflow,
        run,
        position: group.page?.position,
      })),
    ),
  );
  const rows = $derived(
    loadedRows
      .filter((row) => matchesWorkflowRun(row, selection))
      .sort(
        (a, b) =>
          Number(b.run.created_at) - Number(a.run.created_at) ||
          b.run.id.localeCompare(a.run.id),
      ),
  );
  const pickerRows = $derived(
    loadedRows.filter(
      (row) =>
        !selection.source || row.run.source.repository === selection.source,
    ),
  );
  const loading = $derived(
    catalogLoading || history.groups.some((group) => group.loading),
  );
  const title = $derived(
    selection.workflow
      ? (entries.find(
          (entry) =>
            entry.home === selection.scope &&
            entry.workflow.id === selection.workflow,
        )?.workflow.definition.name ?? "Selected workflow")
      : sourceRepository
        ? "All workflows"
        : selection.source
          ? `${repositoryName(selection.source)} workflows`
          : selection.scope
            ? `History in ${repositoryName(selection.scope)}`
            : "All workflows",
  );
  const back = () =>
    onChange({
      home: undefined,
      run: undefined,
      at: undefined,
      attempt: undefined,
    });
  function select(fields: WorkflowSelection) {
    onChange({
      ...fields,
      home: undefined,
      run: undefined,
      at: undefined,
      attempt: undefined,
    });
    if (shell.mobile.resolvedMode === "mobile") shell.mobile.showMain();
  }
  const timer = setInterval(() => {
    if (selection.run && selection.at === undefined) void history.inspect();
  }, 3000);
  onDestroy(() => clearInterval(timer));
</script>

<StoreNotice
  noticeKey={`workflow-run:${selection.home}:${selection.run}:${selection.at}:${selection.attempt}`}
  title="Run details unavailable"
  message={history.detailError}
  pending={history.detailLoading}
  action={{ id: "retry-run", label: "Retry run", run: () => history.inspect() }}
/>
{#if selection.run}<WorkflowHistoryDetail
    {history}
    {owner}
    {catalogLoading}
    {selection}
    {sourceRepository}
    {repositoryName}
    writable={canWrite(selection.home ?? "")}
    {onChange}
    onClose={back}
  />
{:else}
  <AppShell.Body
    bind:ref={bodyElement}
    element="div"
    layout="regions"
    label="Workflow history"
    class="workflow-history-layout"
  >
    <AppShell.Body.Sidebar
      bind:ref={sidebarElement}
      side="left"
      panelId="workflow-history-tree"
      label="Workflow navigation"
      mobileLabel="Workflows"
      bind:open={sidebarOpen}
      class="workflow-history-sidebar"
      style={`--ui-shell-body-sidebar-width: ${resize.width ?? 256}px`}
    >
      <WorkflowSidebarHeader {sourceRepository} {selection} {onChange} />
      <WorkflowHistorySidebar
        entries={sourceRepository
          ? entries.filter(
              (entry) =>
                entry.workflow.definition.repository === sourceRepository,
            )
          : entries}
        {selection}
        {repositoryName}
        onSelect={select}
        {workflowActions}
        {canWrite}
        {sourceRepository}
      />
      <WorkflowCatalogMore {owners} {repositoryName} />
    </AppShell.Body.Sidebar>
    {#if sidebarOpen && shell.mobile.resolvedMode !== "mobile"}<button
        type="button"
        class="lc-community-inbox__resize-handle"
        data-resizing={resize.resizing ? "" : undefined}
        aria-label="Resize workflow navigation"
        title="Drag to resize. Double-click to reset width."
        onpointerdown={resize.begin}
        onpointermove={resize.move}
        onpointerup={resize.finish}
        onpointercancel={resize.finish}
        ondblclick={resize.reset}
        onkeydown={resize.key}><span aria-hidden="true"></span></button
      >{/if}
    <AppShell.Body.Content
      label="Workflow history runs"
      class="workflow-history-content"
    >
      <div class="workflow-history-content__inner">
        <CommunityInboxThreadHeader
          listLabel="Workflow navigation"
          plural="History"
          threadTitle={title}
          expanded={shell.mobile.resolvedMode === "mobile"
            ? shell.mobile.stage === "left" &&
              shell.mobile.activePanelId("left") === "workflow-history-tree"
            : sidebarOpen}
          openSidebar={() => {}}
          onToggle={() => {
            if (shell.mobile.resolvedMode === "mobile") {
              if (shell.mobile.stage === "left") shell.mobile.showMain();
              else shell.mobile.show("left", "workflow-history-tree");
            } else sidebarOpen = !sidebarOpen;
          }}
        />
        <ScrollArea class="workflow-history-scroll">
          <div class="workflow-history-main">
            <section
              class="workflow-history-card"
              aria-label="Run history"
              aria-busy={loading}
            >
              <header class="workflow-history-card__header">
                <span class="workflow-history-count"
                  >{rows.length}
                  {rows.length === 1 ? "run" : "runs"}<span class="sr-only">
                    in the loaded history</span
                  ></span
                >
                <WorkflowFilters {selection} rows={pickerRows} {onChange} />
              </header>
              {#each history.groups as group (`${group.home}:${group.workflow.id}`)}
                <StoreNotice
                  noticeKey={`workflow-history:${group.home}:${group.workflow.id}`}
                  title={`History unavailable for ${group.workflow.definition.name}`}
                  message={group.error}
                  pending={group.loading}
                  action={{
                    id: "retry-history",
                    label: "Retry history",
                    run: () => history.read(group),
                  }}
                />
              {/each}
              {#if loading && !rows.length}<WorkflowPageSkeleton
                  view="history"
                  listOnly
                />
              {:else}<WorkflowHistoryRows
                  {rows}
                  {repositoryName}
                  {onChange}
                  {onWorkflow}
                />
                {#if !loading && !rows.length && !history.groups.some((group) => group.error)}<StoreWorkEmpty
                    kind="activity"
                    title="No matching runs"
                    description="Change the filters, load more history or run this workflow to begin."
                  />{/if}
              {/if}
              {#if history.groups.some((group) => group.page?.next)}<footer
                  class="workflow-history-card__more"
                >
                  {#each history.groups.filter((group) => group.page?.next) as group (`${group.home}:${group.workflow.id}`)}<Button
                      variant="outline"
                      disabled={group.loading}
                      onclick={() => history.read(group, true)}
                      >More runs for {group.workflow.definition.name}</Button
                    >{/each}
                </footer>{/if}
            </section>
          </div>
        </ScrollArea>
      </div>
    </AppShell.Body.Content>
  </AppShell.Body>
{/if}
