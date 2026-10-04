<script lang="ts">
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import StoreCaptureLoading from "./StoreCaptureLoading.svelte";
  import DownloadIcon from "@lucide/svelte/icons/download";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Badge } from "@lapismd/design-core/shadcn/badge";
  import * as Table from "@lapismd/design-core/shadcn/table";
  import StoreCaptureSourceIdentity from "./StoreCaptureSourceIdentity.svelte";
  import StoreCaptureSourceFilters from "./StoreCaptureSourceFilters.svelte";
  import StoreCaptureSourceFooter from "./StoreCaptureSourceFooter.svelte";
  import { captureSourceMatches } from "./capture-source-presentation.js";
  import {
    captureProfiles,
    captureHistorySupported,
    type CaptureManagement,
  } from "../../controller/capture-types.js";
  let {
    management,
    repositoryName,
    onImport,
    onRepository,
    repositoryNavigationReason = "",
  }: {
    management: CaptureManagement;
    repositoryName: (id: string) => string;
    onImport?: () => void;
    onRepository?: (id: string) => void;
    repositoryNavigationReason?: string;
  } = $props();
  const captureState = $derived(management.state);
  const sources = $derived(captureState.sources);
  let harness = $state("all");
  let statuses = $state<string[]>([]);
  const rows = $derived(
    (sources?.sources ?? [])
      .map((source, index) => ({
        source,
        label: `${captureProfiles[source.profile]} source ${index + 1}`,
      }))
      .filter(({ source }) => captureSourceMatches(source, harness, statuses)),
  );
  const navigationReason = $derived(
    repositoryNavigationReason ||
      (captureState.settings.dirty || captureState.settings.unknown
        ? "Save or cancel your Agent configuration before opening a repository."
        : captureState.settings.saving ||
            captureState.pending ||
            captureState.commandBusy
          ? "Resolve the current Agent request before opening a repository."
          : ""),
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
  <header class="store-settings__section-heading">
    <div>
      <h2 id="capture-source-heading">Source coverage</h2>
      <p>
        Sources update automatically. Import history enrolls an inactive source;
        the server checks repository access.
      </p>
    </div>
  </header>
  <StoreCaptureSourceFilters bind:harness bind:statuses />
  {#if captureState.sourceState === "failed"}
    <StoreSettingsNotice
      >Refresh capture to select current source coverage.</StoreSettingsNotice
    >
  {/if}
  <div class="store-capture-sources__table">
    {#if !sources && captureState.sourceState !== "failed"}
      <StoreCaptureLoading label="Loading capture sources" />
    {:else if sources}
      <Table.Root aria-label="Capture sources">
        <Table.Header>
          <Table.Row>
            <Table.Head scope="col">Source</Table.Head>
            <Table.Head scope="col">Status</Table.Head>
            <Table.Head scope="col">History</Table.Head>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {#each rows as { source, label } (source.selector)}
            <Table.Row>
              <Table.Cell>
                <StoreCaptureSourceIdentity
                  {source}
                  {label}
                  {repositoryName}
                  {onRepository}
                  {navigationReason}
                />
              </Table.Cell>
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
                  size="icon-sm"
                  title={captureHistorySupported(source.profile)
                    ? "Import history"
                    : "History import is unavailable for this harness"}
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
                  <DownloadIcon data-icon="inline-start" aria-hidden="true" />
                </Button>
              </Table.Cell>
            </Table.Row>
          {:else}
            <Table.Row
              ><Table.Cell colspan={3}>
                <div class="store-capture-sources__empty">
                  <strong
                    >{sources.sources.length
                      ? "No sources match these filters"
                      : "No capture sources"}</strong
                  >
                  <p class="store-settings__row-description">
                    {sources.sources.length
                      ? "Adjust the harness or status filter, or load more sources."
                      : "No sources are observed in this snapshot."}
                  </p>
                </div>
              </Table.Cell></Table.Row
            >
          {/each}
        </Table.Body>
        <Table.Footer
          ><Table.Row
            ><Table.Cell colspan={3}
              ><StoreCaptureSourceFooter
                {management}
                shown={rows.length}
              /></Table.Cell
            ></Table.Row
          ></Table.Footer
        >
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
  .store-capture-sources__states {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
  }
  .store-capture-sources__states {
    max-width: 18rem;
    min-width: 10rem;
  }
  .store-capture-sources__empty {
    padding: 1rem;
  }
</style>
