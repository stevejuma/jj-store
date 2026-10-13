<script lang="ts">
  import { onDestroy } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import WorkflowPageSkeleton from "./WorkflowPageSkeleton.svelte";
  import History from "@lucide/svelte/icons/history";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import StoreWorkOwnershipGroups from "../store-work/StoreWorkOwnershipGroups.svelte";
  import WorkflowHistoryRows from "./WorkflowHistoryRows.svelte";
  import WorkflowRunDetail from "./WorkflowRunDetail.svelte";
  import {
    createWorkflowHistory,
    type WorkflowHistoryEntry,
  } from "./workflow-history.svelte.js";
  import type { WorkflowController } from "../../controller/workflows.svelte.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  let {
    entries,
    catalogLoading = false,
    owners,
    selection,
    repositoryName,
    canWrite,
    principal,
    onChange,
  }: {
    entries: WorkflowHistoryEntry[];
    catalogLoading?: boolean;
    owners: readonly (readonly [string, WorkflowController])[];
    selection: WorkflowSelection;
    repositoryName(id: string): string;
    canWrite(home: string): boolean;
    principal(home: string): string | undefined;
    onChange(fields: WorkflowSelection): void;
  } = $props();
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
  const filtered = $derived(
    history.groups.map((group) => ({
      group,
      rows: group.rows.filter((run) => {
        const q = selection.q?.trim().toLocaleLowerCase() ?? "",
          current = run.attempts.at(-1);
        return (
          (!selection.state || current?.status === selection.state) &&
          (!selection.source || run.source.repository === selection.source) &&
          `${group.workflow.definition.name} ${run.id} ${run.source.bookmark ?? ""} ${current?.host ?? ""} ${current?.status ?? ""}`
            .toLocaleLowerCase()
            .includes(q)
        );
      }),
    })),
  );
  const rows = $derived(
    filtered
      .flatMap(({ group, rows }) =>
        rows.map((run) => ({
          home: group.home,
          workflow: group.workflow,
          run,
          position: group.page?.position,
          actor: { created_by: run.requester },
        })),
      )
      .sort(
        (a, b) =>
          Number(b.run.created_at) - Number(a.run.created_at) ||
          b.run.id.localeCompare(a.run.id),
      ),
  );
  const loading = $derived(
    catalogLoading || history.groups.some((group) => group.loading),
  );
  const back = () =>
    onChange({
      home: undefined,
      run: undefined,
      at: undefined,
      attempt: undefined,
    });
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
{#if selection.run}
  {#if !owner && catalogLoading}<WorkflowPageSkeleton view="detail" />
  {:else if !owner}<Button variant="ghost" onclick={back}
      >Back to history</Button
    ><StoreWorkEmpty
      title="Metadata home unavailable"
      description="Reconnect with access to the run's metadata home."
    />
  {:else if history.detail}<WorkflowRunDetail
      page={history.detail}
      work={owner}
      {selection}
      {repositoryName}
      writable={canWrite(selection.home!)}
      loading={history.detailLoading}
      {onChange}
      onClose={back}
      onRefresh={() => void history.inspect()}
      onMore={() => void history.inspect(true)}
    />
  {:else}<WorkflowPageSkeleton view="detail" />{/if}
{:else}
  {#if !loading || history.groups.some((group) => group.rows.length)}<header
      class="community-projects__surface-header"
    >
      <div>
        <History aria-hidden="true" />
        <h1>History</h1>
      </div>
    </header>{/if}
  {#each filtered as { group, rows } (`${group.home}:${group.workflow.id}`)}
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
    {#if group.page?.next}<Button
        variant="outline"
        disabled={group.loading}
        onclick={() => history.read(group, true)}
        >More runs for {group.workflow.definition.name}</Button
      >{/if}
  {/each}
  <StoreWorkOwnershipGroups
    {rows}
    kind="runs"
    Icon={History}
    isMine={(row) =>
      !!principal(row.home) && row.run.requester === principal(row.home)}
    >{#snippet children(items)}<WorkflowHistoryRows
        rows={items}
        {repositoryName}
        {onChange}
      />{/snippet}</StoreWorkOwnershipGroups
  >
  {#if loading && !history.groups.some((group) => group.rows.length)}<WorkflowPageSkeleton
      view="history"
    />
  {:else if !filtered.some(({ rows }) => rows.length) && !history.groups.some((group) => group.error)}<StoreWorkEmpty
      kind="activity"
      title="No matching runs"
      description="Change the filters, load more workflows or run a workflow to begin."
    />{/if}
{/if}
