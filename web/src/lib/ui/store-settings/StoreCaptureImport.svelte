<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/download";
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

<StoreSettingsCard
  id="capture-import-heading"
  title="History import progress"
  Icon={SectionIcon}
>
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
              >{requestId && !captureId(requestId)
                ? "Enter the original 32-character lowercase request ID."
                : "Inspect the original request without selecting a new source."}</Field.Description
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
    <StoreSettingsFooter>
      {#snippet description()}<div>
          <strong>Inspect history import</strong>
          <p>
            Check the accepted boundary and publication progress for the
            original request.
          </p>
        </div>{/snippet}

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
          ><RefreshCwIcon data-icon="inline-start" aria-hidden="true" />Refresh
          import status</Button
        >{/if}
    </StoreSettingsFooter>
  </Field.Group>
  {#if result}
    <dl class="store-settings__rows">
      <StoreReadOnlyRow label="Import request">
        <CopyableValue label="Import request ID" value={result.request_id} />
      </StoreReadOnlyRow>
      <StoreReadOnlyRow label="History acceptance"
        >{result.receipt
          ? "Accepted"
          : "No receipt available"}</StoreReadOnlyRow
      >
      <StoreReadOnlyRow label="History publication">
        {result.progress.phase}{result.progress.boundary_available
          ? " · Boundary available"
          : " · Boundary unavailable"}
      </StoreReadOnlyRow>
      {#if result.receipt}<StoreReadOnlyRow label="Accepted boundary">
          {result.receipt.complete_byte_offset.toLocaleString("en-GB")} bytes ·
          {result.receipt.complete_records.toLocaleString("en-GB")} records
        </StoreReadOnlyRow>{/if}
      <StoreReadOnlyRow label="History captured / published">
        {result.progress.captured_byte_offset.toLocaleString("en-GB")} / {result.progress.committed_byte_offset.toLocaleString(
          "en-GB",
        )} bytes
      </StoreReadOnlyRow>
      <StoreReadOnlyRow label="Live tail coverage"
        >{result.progress.tail_phase}</StoreReadOnlyRow
      >
      {#if result.progress.tail}
        <StoreReadOnlyRow label="Tail captured / published">
          {result.progress.tail.captured_byte_offset.toLocaleString("en-GB")} /
          {result.progress.tail.committed_byte_offset.toLocaleString("en-GB")} bytes
        </StoreReadOnlyRow>
        <StoreReadOnlyRow label="Tail evidence">
          {result.progress.tail.history_available ? "Available" : "Unavailable"} ·
          {result.progress.tail.outstanding_ranges.toLocaleString("en-GB")} outstanding
          ranges · {result.progress.tail.receipts.toLocaleString("en-GB")} receipts
        </StoreReadOnlyRow>
      {/if}
    </dl>
    <StoreSettingsNotice>
      {#if !result.receipt}
        No receipt is available. This does not prove rejection or completion.
      {:else}
        History completion covers the accepted record boundary. Live-tail
        coverage is reported separately. Published AgentStore data appears
        automatically in Agents.
      {/if}
    </StoreSettingsNotice>
  {/if}
</StoreSettingsCard>
