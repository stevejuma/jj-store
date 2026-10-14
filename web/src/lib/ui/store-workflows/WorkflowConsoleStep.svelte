<script lang="ts">
  import { onDestroy, untrack } from "svelte";
  import * as Collapsible from "@lapismd/design-core/shadcn/collapsible";
  import ChevronDown from "@lucide/svelte/icons/chevron-down";
  import ChevronRight from "@lucide/svelte/icons/chevron-right";
  import Refresh from "@lucide/svelte/icons/refresh-cw";
  import { sectionDisclosure } from "../disclosure-layout.svelte.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import WorkflowConsoleLogs from "./WorkflowConsoleLogs.svelte";
  import WorkflowRunStatus from "./WorkflowRunStatus.svelte";
  import { readConsoleEvidence } from "./workflow-console-reads.js";
  import {
    stepDuration,
    workflowConsole,
    type ConsoleStep,
    type ConsoleCacheEntry,
  } from "./workflow-console.js";
  import type {
    WorkflowPage,
    WorkflowTransport,
    WorkflowEvidence,
  } from "../../controller/workflow-types.js";
  let {
    step,
    identity,
    observed,
    page,
    transport,
    seconds,
    timestamps,
    cache,
    initialOpen = false,
    onOpen,
    unscoped = false,
  }: {
    step: ConsoleStep;
    identity: string[];
    observed: number;
    page: WorkflowPage;
    transport: WorkflowTransport;
    seconds: boolean;
    timestamps: boolean;
    initialOpen?: boolean;
    onOpen(value: boolean): void;
    unscoped?: boolean;
    cache: Map<string, ConsoleCacheEntry>;
  } = $props();
  const disclosure = sectionDisclosure(
    () => [...identity, step.id],
    untrack(() => initialOpen),
  );
  let chunks = $state<WorkflowEvidence[]>([]),
    loading = $state(false),
    loaded = $state(false),
    error = $state("");
  let generation = 0,
    abort = new AbortController();
  let loadedStep = $state<ConsoleStep | null>(null);
  const display = $derived(loadedStep ?? step);
  const name = $derived(
    step.output.job
      ? `${step.output.job} / ${step.output.name ?? step.id}`
      : (step.output.name ?? step.id),
  );
  const label = $derived(
    unscoped
      ? "Run output"
      : `Step ${name} · ${display.status.replaceAll("_", " ")}`,
  );
  async function load(snapshot: WorkflowPage) {
    const key = `${snapshot.epoch}:${snapshot.position}:${identity.join(":")}:${step.id}`;
    const saved = cache.get(key);
    if (saved) {
      chunks = saved.chunks;
      loadedStep = saved.step ?? null;
      loaded = true;
      error = "";
      return;
    }
    const ticket = ++generation;
    abort.abort();
    abort = new AbortController();
    loading = true;
    try {
      const attempt = identity[3]!;
      const rows = await readConsoleEvidence(
        transport,
        snapshot,
        attempt,
        abort.signal,
        step.startSequence,
        unscoped ? undefined : step.id,
      );
      if (ticket !== generation) return;
      const projection = workflowConsole(
        snapshot.runs[0]!,
        snapshot.workflows[0]?.definition,
        rows,
        attempt,
      );
      loadedStep = projection.steps.find((s) => s.id === step.id) ?? null;
      chunks = unscoped
        ? projection.unscoped
        : (projection.steps.find((s) => s.id === step.id)?.chunks ?? []);
      if (cache.size > 64) cache.clear();
      cache.set(key, { chunks, step: loadedStep ?? undefined });
      loaded = true;
      error = "";
    } catch (cause) {
      if (ticket === generation && !abort.signal.aborted)
        error =
          cause instanceof Error ? cause.message : "Step logs unavailable";
    } finally {
      if (ticket === generation) loading = false;
    }
  }
  $effect(() => {
    const snapshot = page,
      open = disclosure.open;
    if (open) untrack(() => void load(snapshot));
    return () => {
      generation++;
      abort.abort();
    };
  });
  onDestroy(() => {
    generation++;
    abort.abort();
  });
</script>

<StoreNotice
  noticeKey={`workflow-step-logs:${identity.join(":")}:${step.id}:${page.position}`}
  title="Step logs unavailable"
  message={error}
  pending={loading}
  action={{
    id: "retry-step-logs",
    label: "Retry step logs",
    run: () => load(page),
  }}
/>
<Collapsible.Root
  class="workflow-console-step"
  data-step={unscoped ? undefined : step.id}
  bind:open={
    () => disclosure.open,
    (value) => {
      disclosure.open = value;
      onOpen(value);
    }
  }
>
  <Collapsible.Trigger
    class="workflow-console-step__summary"
    aria-label={label}
  >
    {#if loading && !loaded}<Refresh
        class="workflow-console-spinner"
        aria-label={`Loading logs for ${step.id}`}
      />{:else if disclosure.open}<ChevronDown
        aria-hidden="true"
      />{:else}<ChevronRight aria-hidden="true" />{/if}
    {#if !unscoped}<WorkflowRunStatus status={display.status} console />{/if}
    <span class="workflow-console-step__name"
      >{unscoped ? "Run output" : name}</span
    >
    {#if !unscoped}<span class="workflow-console-step__duration"
        >{stepDuration(display, observed)}</span
      >{/if}
  </Collapsible.Trigger>
  <Collapsible.Content>
    {#if loaded}<WorkflowConsoleLogs
        {chunks}
        {seconds}
        {timestamps}
        started={display.started}
      />{/if}
    {#each Object.entries(display.output).filter(([key]) => !["name", "parent", "job", "kind", "order"].includes(key)) as [key, value] (key)}<dl
        class="workflow-console-output"
      >
        <dt>{key}</dt>
        <dd>{value}</dd>
      </dl>{/each}
    {#if loaded && !chunks.length && !Object.keys(display.output).length}<p
        class="workflow-console-empty"
      >
        No logs recorded for this step.
      </p>{/if}
  </Collapsible.Content>
</Collapsible.Root>
