<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { formatCommunityRelativeTime } from "../community-relative-time/model.js";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import X from "@lucide/svelte/icons/x";
  import type {
    WorkflowPage,
    WorkflowRun,
    WorkflowTransport,
    WorkflowEvidence,
  } from "../../controller/workflow-types.js";
  import type { WorkflowSelection } from "../../controller/workflows-route-codec.js";
  import {
    workflowConsole,
    type ConsoleCacheEntry,
  } from "./workflow-console.js";
  import { readConsoleEvidence } from "./workflow-console-reads.js";
  import WorkflowConsoleStep from "./WorkflowConsoleStep.svelte";
  import WorkflowConsoleSettings from "./WorkflowConsoleSettings.svelte";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreWorkEmpty from "../store-work/StoreWorkEmpty.svelte";
  import "./workflow-console.css";
  let {
    run,
    page,
    selection,
    transport,
    loading,
    onChange,
    onMore,
  }: {
    run: WorkflowRun;
    page: WorkflowPage;
    selection: WorkflowSelection;
    transport: WorkflowTransport;
    loading: boolean;
    onChange(fields: WorkflowSelection): void;
    onMore(): void;
  } = $props();
  const value = $derived(
    workflowConsole(
      run,
      page.workflows[0]?.definition,
      page.evidence,
      selection.attempt,
    ),
  );
  const identity = $derived([
    "workflow",
    selection.home ?? "",
    run.id,
    value.attempt?.id ?? "",
    "step",
  ]);
  let now = $state(Math.floor(Date.now() / 1000));
  const observed = $derived(
    selection.at === undefined && value.attempt?.status === "running"
      ? now * 1000
      : Number(value.attempt?.updated_at ?? 0),
  );
  let seconds = $state(false),
    timestamps = $state(false),
    fullscreen = $state(false),
    downloading = $state(false),
    downloadError = $state("");
  const opened = new Map<string, boolean>(),
    cache = new Map<string, ConsoleCacheEntry>();
  const downloadAbort = new AbortController();
  onMount(() => {
    const timer = window.setInterval(
      () => (now = Math.floor(Date.now() / 1000)),
      1000,
    );
    return () => window.clearInterval(timer);
  });
  function attemptLabel(row: WorkflowRun["attempts"][number], index: number) {
    return `Run attempt #${index + 1} (${formatCommunityRelativeTime(Number(row.started_at ?? row.updated_at) / 1000, { now, absoluteAfterDays: Infinity })})`;
  }
  onDestroy(() => downloadAbort.abort());
  async function download() {
    if (!value.attempt || downloading) return;
    const snapshot = page,
      attempt = value.attempt.id;
    downloading = true;
    try {
      const rows = await readConsoleEvidence(
        transport,
        snapshot,
        attempt,
        downloadAbort.signal,
      );
      if (downloadAbort.signal.aborted) return;
      const text = rows
        .filter((row) => row.data.type === "log")
        .map((row) => (row.data.type === "log" ? row.data.text : ""))
        .join("");
      const url = URL.createObjectURL(
        new Blob([text], { type: "text/plain;charset=utf-8" }),
      );
      const link = document.createElement("a");
      link.href = url;
      link.download = `workflow-${run.id}-attempt-${attempt}.log`;
      link.click();
      setTimeout(() => URL.revokeObjectURL(url), 1000);
      downloadError = "";
    } catch (cause) {
      if (!downloadAbort.signal.aborted)
        downloadError =
          cause instanceof Error ? cause.message : "Workflow logs unavailable";
    } finally {
      downloading = false;
    }
  }
</script>

<StoreNotice
  noticeKey={`workflow-log-download:${run.id}:${value.attempt?.id}`}
  title="Log download unavailable"
  message={downloadError}
  pending={downloading}
  action={{ id: "retry-log-download", label: "Retry download", run: download }}
/>
{#snippet panel()}
  <div class="workflow-console" role="region" aria-label="Step console">
    <header class="workflow-console-header">
      <div class="workflow-console-header__title">
        <h3>{page.workflows[0]?.definition.name ?? "Workflow run"}</h3>
        <p>
          {value.attempt?.status.replaceAll("_", " ") ?? "No attempt recorded"}
        </p>
      </div>
      <div class="workflow-console-header__actions">
        <StoreRepositoryPicker
          label="Run attempt"
          repositories={run.attempts.map((row, index) => ({
            id: row.id,
            name: attemptLabel(row, index),
            description: `${row.status.replaceAll("_", " ")} · ${row.host ?? "Unassigned"}`,
          }))}
          value={selection.attempt ?? value.attempt?.id ?? ""}
          controlled
          allowEmpty
          emptyLabel="Latest attempt"
          searchLabel="Find an attempt"
          groupLabel="Attempts"
          emptyResult="No matching attempts."
          unavailableLabel="Selected attempt"
          keepItemIcon
          onChange={(attempt) =>
            onChange({
              attempt: attempt || undefined,
              at: String(page.position),
            })}
        >
          {#snippet triggerIcon()}<WorkflowRunStatus
              status={value.attempt?.status ?? "pending"}
            />{/snippet}
          {#snippet itemIcon(id)}<WorkflowRunStatus
              status={run.attempts.find((row) => row.id === id)?.status ??
                "pending"}
            />{/snippet}
        </StoreRepositoryPicker>
        <WorkflowConsoleSettings
          bind:seconds
          bind:timestamps
          bind:fullscreen
          {downloading}
          outputs={value.outputs}
          hasMore={!!page.next}
          {loading}
          {onMore}
          onDownload={() => void download()}
        />
        {#if fullscreen}<Button
            variant="ghost"
            size="icon-sm"
            aria-label="Exit full screen"
            onclick={() => (fullscreen = false)}
            ><X aria-hidden="true" /></Button
          >{/if}
      </div>
    </header>
    <div class="workflow-console-body">
      {#key value.attempt?.id}
        {#each value.steps as step (step.id)}
          <WorkflowConsoleStep
            {step}
            {identity}
            {observed}
            {page}
            {transport}
            {seconds}
            {timestamps}
            {cache}
            initialOpen={opened.get(`${value.attempt?.id}:${step.id}`) ?? false}
            onOpen={(open) =>
              opened.set(`${value.attempt?.id}:${step.id}`, open)}
          />
        {/each}
        {#if value.unscoped.length}<WorkflowConsoleStep
            step={{
              id: "run-output",
              status: "",
              chunks: value.unscoped,
              output: {},
            }}
            {identity}
            {observed}
            {page}
            {transport}
            {seconds}
            {timestamps}
            {cache}
            unscoped
            initialOpen={opened.get(`${value.attempt?.id}:run-output`) ?? false}
            onOpen={(open) =>
              opened.set(`${value.attempt?.id}:run-output`, open)}
          />{/if}
      {/key}
      {#if !value.steps.length && !value.unscoped.length}<StoreWorkEmpty
          kind="tasks"
          title="No steps recorded"
          description="Step status updates will appear here as execution progresses."
        />{/if}
    </div>
  </div>
{/snippet}
{#if !fullscreen}{@render panel()}{/if}
<Dialog.Root bind:open={fullscreen}>
  <Dialog.Content
    class="workflow-console-fullscreen"
    dataUiComponent="workflow-console"
    showCloseButton={false}
  >
    <Dialog.Title class="sr-only">Workflow logs full screen</Dialog.Title>
    <Dialog.Description class="sr-only"
      >Steps and logs for the selected workflow attempt.</Dialog.Description
    >
    {#if fullscreen}{@render panel()}{/if}
  </Dialog.Content>
</Dialog.Root>
