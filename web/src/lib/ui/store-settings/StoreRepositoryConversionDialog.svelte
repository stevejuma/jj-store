<script lang="ts">
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import type { ConversionIntent } from "../../controller/repository-conversion-manager.js";
  import type { Model } from "../../api/client.js";
  let {
    open = $bindable(false),
    name = $bindable(""),
    path = $bindable(""),
    operation = $bindable(""),
    local = false,
    busy = false,
    pending = false,
    pendingIntent = null,
    operationHeads = [],
    validation = "",
    progress = "",
    preview = null,
    report = null,
    onEdit,
    onCheck,
    onStart,
    onStatus,
    onDownload,
  }: {
    open?: boolean;
    name?: string;
    path?: string;
    operation?: string;
    local?: boolean;
    busy?: boolean;
    pending?: boolean;
    pendingIntent?: ConversionIntent | null;
    operationHeads?: readonly string[];
    validation?: string;
    progress?: string;
    preview?: Model<"ConversionPreview"> | null;
    report?: Model<"ConversionReport"> | null;
    onEdit: () => void;
    onCheck: () => void;
    onStart: () => void;
    onStatus: () => void;
    onDownload: () => void;
  } = $props();
  const id = $props.id();
</script>

<Dialog.Root bind:open>
  <DialogFrame
    title={report ? "Conversion report" : "Convert repository"}
    description="Create and open an independent destination. The source stays available."
    {busy}
  >
    {#if report}
      <StoreSettingsNotice
        >Verified {report.collaboration.selections.length} collaboration selections
        and {Object.keys(report.code_mappings.commits).length} code commit mappings.
        Original project repository links keep their IDs.</StoreSettingsNotice
      >
      <p>Destination: {report.name}</p>
      <p>Repository ID: {report.destination}</p>
      <Button variant="outline" onclick={onDownload}
        >Download mapping report</Button
      >
    {:else}
      <StoreSettingsNotice
        >Conversion uses a committed code operation and the complete supported
        collaboration history. Working-file edits stay in the source folder.
        Preflight checks evidence and format support before creation.</StoreSettingsNotice
      >
      {#if pending}<StoreSettingsNotice
          >Recovering the original request for {pendingIntent?.name}. Closing
          this dialog or reconnecting preserves that request.</StoreSettingsNotice
        >{/if}
      <Field.Group>
        <StoreWorkField
          id={`${id}-name`}
          label="Destination name"
          disabled={busy || pending}
        >
          <Input
            id={`${id}-name`}
            bind:value={name}
            disabled={busy || pending}
            oninput={onEdit}
          />
        </StoreWorkField>
        {#if pending ? pendingIntent?.destination_profile === "local_jj_git" : !local}
          <StoreWorkField
            id={`${id}-path`}
            label="New folder"
            description="Enter an absolute path to a folder that does not exist."
            disabled={busy || pending}
          >
            <Input
              id={`${id}-path`}
              bind:value={path}
              disabled={busy || pending}
              oninput={onEdit}
              placeholder="/absolute/path/new-repository"
            />
          </StoreWorkField>
        {/if}
        <StoreWorkField
          id={`${id}-operation`}
          label="Committed operation"
          disabled={busy || pending}
        >
          {#if pending}<Input
              id={`${id}-operation`}
              value={operation}
              readonly
              disabled
            />{:else}
            <Select.Root
              type="single"
              bind:value={operation}
              disabled={busy}
              onValueChange={onEdit}
            >
              <Select.Trigger id={`${id}-operation`}
                >{operation
                  ? `${operation.slice(0, 12)}…`
                  : "Choose an operation head"}</Select.Trigger
              >
              <Select.Content
                ><Select.Group
                  >{#each operationHeads as head}<Select.Item
                      value={head}
                      label={head}>{head}</Select.Item
                    >{/each}</Select.Group
                ></Select.Content
              >
            </Select.Root>
          {/if}
        </StoreWorkField>
        {#if validation}<Field.Error>{validation}</Field.Error>{/if}
      </Field.Group>
      {#if preview?.blocker}<StoreSettingsNotice
          >Conversion is unavailable: {preview.blocker}</StoreSettingsNotice
        >{:else if preview?.prepared_source}<StoreSettingsNotice
          >Preflight passed for {preview.history_selections} collaboration selections.
          Code identities and repository-bound record references will be mapped; project
          links stay unchanged.</StoreSettingsNotice
        >{/if}
    {/if}
    {#if progress}<StoreSettingsNotice>{progress}</StoreSettingsNotice>{/if}
    {#snippet footer()}
      <Button variant="outline" disabled={busy} onclick={() => (open = false)}
        >Close</Button
      >
      {#if !report}
        {#if pending}<Button
            variant="outline"
            disabled={busy}
            onclick={onStatus}>Check status</Button
          ><Button disabled={busy} onclick={onStart}>Retry conversion</Button>
        {:else if preview?.prepared_source}<Button
            disabled={busy}
            onclick={onStart}>Convert and open destination</Button
          >
        {:else}<Button
            disabled={busy ||
              !name.trim() ||
              !operation ||
              (!local && !path.trim())}
            onclick={onCheck}>{busy ? "Checking…" : "Check conversion"}</Button
          >{/if}
      {/if}
    {/snippet}
  </DialogFrame>
</Dialog.Root>
