<script lang="ts">
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import XIcon from "@lucide/svelte/icons/x";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import type {
    StorageOperation,
    StorageAction,
  } from "../../controller/storage-settings.js";
  import StoreStoragePosition from "./StoreStoragePosition.svelte";
  let {
    report,
    disabled,
    onStart,
  }: {
    report: StorageOperation;
    disabled: boolean;
    onStart: (action: StorageAction) => Promise<void>;
  } = $props();
  const words = (value: string) => value.replaceAll("_", " ");
</script>

<dl class="store-settings__rows">
  <StoreReadOnlyRow
    label={report.kind === "check" ? "Integrity check" : "Checkpoint"}
    value={report.kind === "check" && report.phase === "complete"
      ? "Captured data verified"
      : `Operation ${words(report.phase)}`}
    live="status"
  />
  {#if report.message}<div>
      <dt>Details</dt>
      <dd><span role="alert">{report.message}</span></dd>
    </div>{/if}
  {#if report.phase === "incomplete"}<div>
      <dt>Check coverage</dt>
      <dd>The check did not verify all captured data within its limits.</dd>
    </div>{/if}
  {#if report.phase === "interrupted"}<div>
      <dt>Recovery</dt>
      <dd>
        The service stopped during this check. Start a new integrity check to
        verify current data.
      </dd>
    </div>{/if}
  {#if report.selection === "selected"}<div>
      <dt>Selection</dt>
      <dd>The exact captured checkpoint is selected.</dd>
    </div>
  {:else if report.selection === "covered"}<div>
      <dt>Selection</dt>
      <dd>
        A newer selected checkpoint covers this target. The original publication
        is not proven.
      </dd>
    </div>{/if}
  <StoreReadOnlyRow
    label="Request"
    value={report.request_id}
    copyLabel="Maintenance request"
  />
</dl>
<header class="store-settings__section-heading">
  <h2>Captured target</h2>
</header>
<StoreStoragePosition position={report.target} />
{#if report.kind === "checkpoint" && ["prepared", "unknown"].includes(report.phase)}
  <div class="store-settings__action-row">
    <div>
      <strong>Publish captured checkpoint</strong>
      <p>
        Publishing keeps newer changes and retained history. This target stays
        fixed across retries.
      </p>
    </div>
    <div class="store-settings__actions">
      <Button {disabled} onclick={() => void onStart("publish_checkpoint")}
        ><SaveIcon
          data-icon="inline-start"
          aria-hidden="true"
        />{report.phase === "prepared"
          ? "Publish checkpoint"
          : "Retry checkpoint"}</Button
      >
      {#if report.phase === "prepared"}<Button
          variant="outline"
          {disabled}
          onclick={() => void onStart("cancel_checkpoint")}
          ><XIcon data-icon="inline-start" aria-hidden="true" />Cancel
          checkpoint</Button
        >{/if}
    </div>
  </div>
{/if}
