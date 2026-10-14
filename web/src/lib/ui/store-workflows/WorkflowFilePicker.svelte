<script lang="ts">
  import { untrack } from "svelte";
  import File from "@lucide/svelte/icons/file-code";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import StoreModelFilePicker from "../store-settings/StoreModelFilePicker.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StoreDataSource } from "../../controller/native-data-source.js";
  import { workflowFiles } from "./workflow-files.js";
  let {
    id,
    repository,
    value = $bindable(""),
    acquireRepository,
    disabled = false,
  }: {
    id: string;
    repository: string;
    value?: string;
    acquireRepository?: StoreController["acquireRepository"];
    disabled?: boolean;
  } = $props();
  let source = $state<StoreDataSource>(),
    rows = $state<{ id: string; name: string }[]>([]),
    loading = $state(false),
    error = $state(""),
    open = $state(false);
  let generation = 0,
    abort = new AbortController();
  $effect(() => {
    const repo = repository,
      acquire = acquireRepository;
    untrack(() => {
      generation++;
      abort.abort();
      abort = new AbortController();
      rows = [];
      loading = false;
      error = "";
      source = undefined;
      try {
        source = repo ? acquire?.(repo) : undefined;
      } catch (cause) {
        error =
          cause instanceof Error
            ? cause.message
            : "Workflow source unavailable";
      }
    });
    const reader = untrack(() => source);
    return () => {
      generation++;
      abort.abort();
      reader?.close();
    };
  });
  async function load() {
    const reader = source,
      ticket = generation;
    if (!reader || loading) return;
    loading = true;
    try {
      const files = await workflowFiles(reader, abort.signal);
      if (ticket === generation) {
        rows = files;
        error = "";
      }
    } catch (cause) {
      if (ticket === generation && !abort.signal.aborted)
        error =
          cause instanceof Error ? cause.message : "Workflow files unavailable";
    } finally {
      if (ticket === generation) loading = false;
    }
  }
</script>

<StoreNotice
  noticeKey={`workflow-files:${id}:${repository}`}
  title="Workflow files unavailable"
  message={error}
  pending={loading}
  action={{
    id: "retry-workflow-files",
    label: "Retry workflow files",
    run: load,
  }}
/>
<div class="workflow-file-control">
  <StoreModelFilePicker
    {id}
    bind:value
    {disabled}
    supported={!!source}
    label="Choose workflow file"
    placeholder=".github/workflows/ci.yml"
    onBrowse={async () => {
      open = true;
      await load();
      return null;
    }}
  />
</div>
<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Browse GitHub workflows</Dialog.Title>
      <Dialog.Description
        >Choose YAML from the source repository's recorded revision.</Dialog.Description
      >
    </Dialog.Header>
    <StoreRepositoryPicker
      label="Workflow file"
      repositories={rows}
      {value}
      {disabled}
      {loading}
      inline
      controlled
      keepItemIcon
      searchLabel="Find a YAML workflow"
      groupLabel="GitHub workflows"
      emptyResult={loading
        ? "Loading YAML workflows…"
        : "No YAML workflows in this revision"}
      onChange={(path) => {
        value = path;
        open = false;
      }}
    >
      {#snippet itemIcon()}<File aria-hidden="true" />{/snippet}
    </StoreRepositoryPicker>
  </Dialog.Content>
</Dialog.Root>
