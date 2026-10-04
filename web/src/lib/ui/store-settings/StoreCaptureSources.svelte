<script lang="ts">
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import StoreCaptureLoading from "./StoreCaptureLoading.svelte";
  import DownloadIcon from "@lucide/svelte/icons/download";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Badge } from "@lapismd/design-core/shadcn/badge";
  import * as Table from "@lapismd/design-core/shadcn/table";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import {
    captureProfiles,
    captureHistorySupported,
    type CaptureManagement,
  } from "../../controller/capture-types.js";
  let {
    management,
    repositoryName,
    onImport,
  }: {
    management: CaptureManagement;
    repositoryName: (id: string) => string;
    onImport?: () => void;
  } = $props();
  const captureState = $derived(management.state);
  const sources = $derived(captureState.sources);
  let harness = $state("all");
  const filters = [
    { value: "all", label: "All harnesses" },
    { value: "codex", label: "Codex" },
    { value: "cursor", label: "Cursor" },
    { value: "opencode", label: "OpenCode" },
  ];
  const profiles = {
    "codex-cli-jsonl-v1": "codex",
    "codex-desktop-jsonl-v1": "codex",
    "cursor-desktop-transcript-v1": "cursor",
    "opencode-read-only-api-v1": "opencode",
    "opencode-export-json-v1": "opencode",
  };
  const rows = $derived(
    (sources?.sources ?? [])
      .map((source, index) => ({
        source,
        label: `${captureProfiles[source.profile]} source ${index + 1}`,
      }))
      .filter(
        ({ source }) =>
          harness === "all" || profiles[source.profile] === harness,
      ),
  );
  const allowed = $derived(
    !captureState.settings.saving &&
      !captureState.settings.value?.restart_required &&
      management.supported &&
      !captureState.busy &&
      !captureState.pending &&
      captureState.status === "ready" &&
      captureState.sourceState === "ready" &&
      captureState.value?.enabled &&
      !captureState.value.policy?.paused,
  );
</script>

<section
  class="store-settings__section"
  aria-labelledby="capture-source-heading"
>
  <header><h2 id="capture-source-heading">Source coverage</h2></header>
  <div class="store-settings__rows store-settings__rows--controls">
    <div>
      <div>
        <strong>Observed sources</strong>
        <p class="store-settings__row-description">
          Source observations update automatically. Import history explicitly
          enrolls an inactive source; repository access is checked by the
          server.
        </p>
      </div>
      <div
        class="store-capture-sources__counts"
        role="status"
        aria-label="Source counts"
      >
        {#if sources}
          <Badge variant="secondary"
            >{rows.length.toLocaleString("en-GB")} shown</Badge
          >
          <Badge variant="outline"
            >{sources.sources.length.toLocaleString("en-GB")} loaded</Badge
          >
          <Badge variant="outline"
            >{sources.total.toLocaleString("en-GB")} observed</Badge
          >
        {/if}
      </div>
    </div>
  </div>
  {#if captureState.sourceState === "failed"}
    <StoreSettingsNotice
      >Refresh capture to select current source coverage.</StoreSettingsNotice
    >
  {/if}
  <div class="store-capture-sources__table">
    <div class="store-capture-sources__toolbar">
      <Select.Root type="single" bind:value={harness}>
        <Select.Trigger aria-label="Filter by harness">
          {filters.find((f) => f.value === harness)?.label}
        </Select.Trigger>
        <Select.Content>
          <Select.Group>
            <Select.Label>Harness</Select.Label>
            {#each filters as f (f.value)}
              <Select.Item value={f.value} label={f.label}
                >{f.label}</Select.Item
              >
            {/each}
          </Select.Group>
        </Select.Content>
      </Select.Root>
      {#if sources?.next_cursor}
        <Button
          variant="outline"
          size="sm"
          disabled={captureState.busy ||
            !management.supported ||
            captureState.sourceState !== "ready"}
          onclick={() => management.moreSources()}
        >
          More capture sources
        </Button>
      {/if}
    </div>
    {#if !sources && captureState.sourceState !== "failed"}
      <StoreCaptureLoading label="Loading capture sources" />
    {:else if sources}
      <Table.Root aria-label="Capture sources">
        <Table.Header>
          <Table.Row>
            <Table.Head scope="col">Source</Table.Head>
            <Table.Head scope="col">Repository</Table.Head>
            <Table.Head scope="col">Status</Table.Head>
            <Table.Head scope="col">History</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {#each rows as { source, label } (source.selector)}
            <Table.Row>
              <Table.Cell>
                <div class="store-settings__stack">
                  <strong>{label}</strong>
                  <span class="store-settings__row-description"
                    >{source.product_version}</span
                  >
                </div>
              </Table.Cell>
              <Table.Cell
                >{source.last_bound_repository
                  ? repositoryName(source.last_bound_repository)
                  : "No recorded repository binding"}</Table.Cell
              >
              <Table.Cell>
                <div class="store-capture-sources__states">
                  <Badge variant="secondary">{source.phase}</Badge>
                  <Badge variant="outline"
                    >{source.observed ? "Observed" : "Unobserved"}</Badge
                  >
                  {#if source.awaiting_activity}<Badge variant="outline"
                      >Awaiting activity</Badge
                    >{/if}
                  {#if source.caught_up}<Badge variant="outline"
                      >Caught up</Badge
                    >{/if}
                  {#if source.running}<Badge variant="outline">Running</Badge
                    >{/if}
                  {#if source.degraded}<Badge variant="destructive"
                      >Degraded</Badge
                    >{/if}
                </div>
                {#if source.ingress_required_capacity_bytes !== null}
                  <p class="store-settings__row-description">
                    Requires {source.ingress_required_capacity_bytes.toLocaleString(
                      "en-GB",
                    )} bytes
                  </p>
                {/if}
              </Table.Cell>
              <Table.Cell>
                <Button
                  variant="outline"
                  size="sm"
                  aria-label={`Import history for ${label}`}
                  disabled={!allowed ||
                    !captureHistorySupported(source.profile) ||
                    !source.awaiting_activity ||
                    source.running}
                  onclick={() => {
                    void management.importHistory(source.selector);
                    onImport?.();
                  }}
                >
                  <DownloadIcon
                    data-icon="inline-start"
                    aria-hidden="true"
                  />{captureHistorySupported(source.profile)
                    ? "Import history"
                    : "Import unavailable"}
                </Button>
              </Table.Cell>
            </Table.Row>
          {:else}
            <Table.Row
              ><Table.Cell colspan={4}>
                <div class="store-capture-sources__empty">
                  <strong
                    >{sources.sources.length
                      ? "No sources for this harness"
                      : "No capture sources"}</strong
                  >
                  <p class="store-settings__row-description">
                    {sources.sources.length
                      ? "Choose another harness or load more sources."
                      : "No sources are observed in this snapshot."}
                  </p>
                </div>
              </Table.Cell></Table.Row
            >
          {/each}
        </Table.Body>
      </Table.Root>
    {/if}
  </div>
</section>

<style>
  .store-capture-sources__table {
    min-width: 0;
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: var(--radius);
    background: var(--card, var(--background));
  }
  .store-capture-sources__toolbar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border-block-end: 1px solid var(--border);
  }
  .store-capture-sources__counts,
  .store-capture-sources__states {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
  }
  .store-capture-sources__counts {
    justify-content: flex-end;
  }
  .store-capture-sources__states {
    max-width: 18rem;
    min-width: 10rem;
  }
  .store-capture-sources__empty {
    padding: 1rem;
  }
  @media (max-width: 40rem) {
    .store-capture-sources__counts {
      justify-content: flex-start;
    }
  }
</style>
