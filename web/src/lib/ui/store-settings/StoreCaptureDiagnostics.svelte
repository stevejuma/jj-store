<script lang="ts">
  import type { Model } from "../../api/client.js";
  import { captureProfiles } from "../../controller/capture-types.js";
  let { value }: { value: Model<"CaptureStatus"> } = $props();
  const runtime = $derived(value.runtime);
  const number = (n: number | bigint | null) =>
    n === null ? "None reported" : n.toLocaleString("en-GB");
</script>

<dl class="store-settings__rows">
  <div>
    <dt>Device policy</dt>
    <dd>
      {value.policy
        ? value.policy.paused
          ? "Paused"
          : "Enabled"
        : "Unavailable"}
    </dd>
  </div>
  <div>
    <dt>Worker phase</dt>
    <dd>{runtime?.phase ?? "Unavailable"}</dd>
  </div>
  {#if runtime}
    <div>
      <dt>Discovery</dt>
      <dd>
        {runtime.discovery.phase} · {runtime.discovery.rescan_pending
          ? "Rescan pending"
          : "No rescan pending"}
      </dd>
    </div>
    <div>
      <dt>Roots</dt>
      <dd>
        {number(runtime.discovery.available_roots)} available · {number(
          runtime.discovery.unavailable_roots,
        )} unavailable · {number(runtime.discovery.unchecked_roots)} unchecked
      </dd>
    </div>
    <div>
      <dt>Sources</dt>
      <dd>
        {number(runtime.observed_sources)} observed · {number(
          runtime.awaiting_activity,
        )} awaiting activity · {number(runtime.caught_up)} caught up · {number(
          runtime.running_sources,
        )} running
      </dd>
    </div>
    <div>
      <dt>Source health</dt>
      <dd>
        {number(runtime.degraded_sources)} degraded · {number(
          runtime.unavailable,
        )} unavailable observations{runtime.source_limit_reached
          ? " · Source limit reached"
          : ""}
      </dd>
    </div>
    <div>
      <dt>Publication backlog</dt>
      <dd>
        {number(runtime.pending_batches)} batches · {number(
          runtime.pending_capacity_bytes,
        )} bytes
      </dd>
    </div>
    <div>
      <dt>Required ingress capacity</dt>
      <dd>
        {number(runtime.ingress_required_capacity_bytes)} · {number(
          runtime.ingress_capacity_refusals,
        )} refusals
      </dd>
    </div>
    <div>
      <dt>Required publication capacity</dt>
      <dd>{number(runtime.delivery_required_capacity_bytes)}</dd>
    </div>
  {/if}
</dl>
{#if runtime}
  {#each runtime.harnesses as harness (harness.profile)}
    <section
      class="store-settings__section"
      aria-label={`${captureProfiles[harness.profile]} coverage`}
    >
      <header>
        <h3>{captureProfiles[harness.profile]} {harness.product_version}</h3>
      </header>
      <dl class="store-settings__rows">
        <div>
          <dt>Configured roots</dt>
          <dd>
            {number(harness.roots)} configured roots · {number(
              harness.available_roots,
            )} available · {number(harness.unavailable_roots)} unavailable · {number(
              harness.unchecked_roots,
            )} unchecked
          </dd>
        </div>
        <div>
          <dt>Tracked sources</dt>
          <dd>
            {number(harness.tracked)} tracked sources · {number(
              harness.observed_sources,
            )} observed · {number(harness.awaiting_activity)} awaiting activity ·
            {number(harness.caught_up)} caught up · {number(
              harness.running_sources,
            )} running · {number(harness.degraded_sources)} degraded
          </dd>
        </div>
      </dl>
    </section>
  {/each}
{/if}
