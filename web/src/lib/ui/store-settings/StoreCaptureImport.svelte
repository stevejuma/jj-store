<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { CopyableValue } from "@lapismd/design-core/workspace/settings";
  import type { CaptureManagement } from "../../controller/capture-types.js";
  import { captureId } from "../../controller/capture-validation.js";
  let { management }: { management: CaptureManagement } = $props();
  const requestId = $derived(management.state.importRequestDraft);
  const result = $derived(management.state.importStatus);
  const busy = $derived(management.state.busy || !management.supported);
</script>

<section
  class="store-settings__section"
  aria-labelledby="capture-import-heading"
>
  <header><h3 id="capture-import-heading">History import progress</h3></header>
  <Field.Group
    ><Field.Field data-invalid={!!requestId && !captureId(requestId)}>
      <Field.Label for="capture-import-request"
        >Previous import request ID</Field.Label
      >
      <Input
        id="capture-import-request"
        bind:value={management.state.importRequestDraft}
        aria-invalid={!!requestId && !captureId(requestId)}
        maxlength={32}
        disabled={busy}
      />
      <Field.Description
        >Inspect the original request without selecting a new source.</Field.Description
      >
    </Field.Field></Field.Group
  >
  <div class="store-settings__actions">
    <Button
      variant="outline"
      disabled={busy || !captureId(requestId)}
      onclick={() => management.inspectImport(requestId)}
      >Inspect import request</Button
    >
    {#if result}<Button
        variant="outline"
        disabled={busy}
        onclick={() => management.inspectImport(result.request_id)}
        >Refresh import status</Button
      >{/if}
  </div>
  {#if result}
    <CopyableValue label="Import request ID" value={result.request_id} />
    <dl class="store-settings__rows">
      <div>
        <dt>History acceptance</dt>
        <dd>{result.receipt ? "Accepted" : "No receipt available"}</dd>
      </div>
      <div>
        <dt>History publication</dt>
        <dd>
          {result.progress.phase}{result.progress.boundary_available
            ? " · Boundary available"
            : " · Boundary unavailable"}
        </dd>
      </div>
      {#if result.receipt}<div>
          <dt>Accepted boundary</dt>
          <dd>
            {result.receipt.complete_byte_offset.toLocaleString("en-GB")} bytes ·
            {result.receipt.complete_records.toLocaleString("en-GB")} records
          </dd>
        </div>{/if}
      <div>
        <dt>History captured / published</dt>
        <dd>
          {result.progress.captured_byte_offset.toLocaleString("en-GB")} / {result.progress.committed_byte_offset.toLocaleString(
            "en-GB",
          )} bytes
        </dd>
      </div>
      <div>
        <dt>Live tail coverage</dt>
        <dd>{result.progress.tail_phase}</dd>
      </div>
      {#if result.progress.tail}
        <div>
          <dt>Tail captured / published</dt>
          <dd>
            {result.progress.tail.captured_byte_offset.toLocaleString("en-GB")} /
            {result.progress.tail.committed_byte_offset.toLocaleString("en-GB")} bytes
          </dd>
        </div>
        <div>
          <dt>Tail evidence</dt>
          <dd>
            {result.progress.tail.history_available
              ? "Available"
              : "Unavailable"} · {result.progress.tail.outstanding_ranges.toLocaleString(
              "en-GB",
            )} outstanding ranges · {result.progress.tail.receipts.toLocaleString(
              "en-GB",
            )} receipts
          </dd>
        </div>
      {/if}
    </dl>
    <p>
      History completion covers the accepted record boundary. Live-tail coverage
      is reported separately. Published AgentStore data appears automatically in
      Agents.
    </p>
  {/if}
</section>
