<script lang="ts">
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import SearchIcon from "@lucide/svelte/icons/search";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
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
  <header><h2 id="capture-import-heading">History import progress</h2></header>
  <Field.Group>
    <div class="store-settings__rows">
      <Field.Group class="store-settings__editable-rows">
        <Field.Field
          orientation="horizontal"
          class="store-settings__option-row"
          data-invalid={!!requestId && !captureId(requestId)}
        >
          <Field.Content>
            <Field.Label for="capture-import-request"
              >Previous import request ID</Field.Label
            >
            <Field.Description id="capture-import-help"
              >Inspect the original request without selecting a new source.</Field.Description
            >
          </Field.Content>
          <Input
            class="store-settings__input"
            id="capture-import-request"
            bind:value={management.state.importRequestDraft}
            aria-describedby="capture-import-help"
            aria-invalid={!!requestId && !captureId(requestId)}
            maxlength={32}
            disabled={busy}
          />
        </Field.Field>
      </Field.Group>
    </div>
    <div class="store-settings__action-row">
      <div>
        <strong>Inspect history import</strong>
        <p>
          Check the accepted boundary and publication progress for the original
          request.
        </p>
      </div>
      <div class="store-settings__actions">
        <Button
          variant="outline"
          disabled={busy || !captureId(requestId)}
          onclick={() => management.inspectImport(requestId)}
          ><SearchIcon data-icon="inline-start" aria-hidden="true" />Inspect
          import request</Button
        >
        {#if result}<Button
            variant="outline"
            disabled={busy}
            onclick={() => management.inspectImport(result.request_id)}
            ><RefreshCwIcon
              data-icon="inline-start"
              aria-hidden="true"
            />Refresh import status</Button
          >{/if}
      </div>
    </div>
  </Field.Group>
  {#if result}
    <dl class="store-settings__rows">
      <div>
        <dt>Import request</dt>
        <dd>
          <CopyableValue label="Import request ID" value={result.request_id} />
        </dd>
      </div>
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
    <StoreSettingsNotice>
      History completion covers the accepted record boundary. Live-tail coverage
      is reported separately. Published AgentStore data appears automatically in
      Agents.
    </StoreSettingsNotice>
  {/if}
</section>
