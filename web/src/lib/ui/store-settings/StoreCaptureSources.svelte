<script lang="ts">
  import * as Table from "@lapismd/design-core/shadcn/table";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import {
    captureProfiles,
    captureHistorySupported,
    type CaptureManagement,
  } from "../../controller/capture-types.js";
  let {
    management,
    repositoryName,
  }: { management: CaptureManagement; repositoryName: (id: string) => string } =
    $props();
  const state = $derived(management.state);
  const sources = $derived(state.sources);
  const allowed = $derived(
    management.supported &&
      !state.busy &&
      !state.pending &&
      state.status === "ready" &&
      state.sourceState === "ready" &&
      state.value?.enabled &&
      !state.value.policy?.paused,
  );
</script>

<section
  class="store-settings__section"
  aria-labelledby="capture-source-heading"
>
  <header><h3 id="capture-source-heading">Source coverage</h3></header>
  <p>
    Source observations update automatically. Import history explicitly enrolls
    an inactive source; repository access is checked by the server.
  </p>
  {#if state.sourceState === "failed" || state.sourceState === "idle"}<p>
      Refresh capture to select current source coverage.
    </p>{/if}
  {#if sources}
    <p>
      {sources.sources.length} sources shown · {sources.total.toLocaleString(
        "en-GB",
      )} currently observed
    </p>
    {#if sources.sources.length}
      <Table.Root>
        <Table.Header
          ><Table.Row
            ><Table.Head>Source</Table.Head><Table.Head>Coverage</Table.Head
            ><Table.Head>History</Table.Head></Table.Row
          ></Table.Header
        >
        <Table.Body>
          {#each sources.sources as source, index (source.selector)}
            {@const label = `${captureProfiles[source.profile]} source ${index + 1}`}
            <Table.Row>
              <Table.Cell
                >{label}
                <p>{source.product_version} · {source.phase}</p>
                <p>
                  {source.last_bound_repository
                    ? repositoryName(source.last_bound_repository)
                    : "No recorded repository binding"}
                </p></Table.Cell
              >
              <Table.Cell
                >{source.observed
                  ? "Observed"
                  : "Unobserved"}{source.awaiting_activity
                  ? " · Awaiting activity"
                  : ""}{source.caught_up ? " · Caught up" : ""}{source.running
                  ? " · Running"
                  : ""}{source.degraded
                  ? " · Degraded"
                  : ""}{#if source.ingress_required_capacity_bytes !== null}<p>
                    Requires {source.ingress_required_capacity_bytes.toLocaleString(
                      "en-GB",
                    )} bytes
                  </p>{/if}</Table.Cell
              >
              <Table.Cell
                ><Button
                  variant="outline"
                  size="sm"
                  aria-label={`Import history for ${label}`}
                  disabled={!allowed ||
                    !captureHistorySupported(source.profile) ||
                    !source.awaiting_activity ||
                    source.running}
                  onclick={() => management.importHistory(source.selector)}
                  >{captureHistorySupported(source.profile)
                    ? "Import history"
                    : "Import unavailable"}</Button
                ></Table.Cell
              >
            </Table.Row>
          {/each}
        </Table.Body>
      </Table.Root>
    {:else}<p>No sources are observed in this snapshot.</p>{/if}
    {#if sources.next_cursor}<Button
        variant="outline"
        disabled={state.busy ||
          !management.supported ||
          state.sourceState !== "ready"}
        onclick={() => management.moreSources()}>More capture sources</Button
      >{/if}
  {/if}
</section>
