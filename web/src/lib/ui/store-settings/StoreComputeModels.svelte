<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreModelFilePicker from "./StoreModelFilePicker.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { object } from "../../controller/agent-execution-types.js";
  import type { Model } from "../../api/client.js";
  import type {
    AgentCompute,
    ComputeValue,
  } from "../../controller/agent-compute.svelte.js";
  let {
    management,
    status,
    blocked = false,
    draft = $bindable(),
    onConfigure,
  }: {
    management?: AgentCompute;
    status: ComputeValue;
    blocked?: boolean;
    draft: Model<"ComputeSettings">;
    onConfigure: () => Promise<boolean>;
  } = $props();
  let importPath = $state("");
  const id = $props.id();
  const progress = $derived(object(status.progress));
</script>

<section class="store-settings__section" aria-labelledby={`${id}-models`}>
  <header class="store-settings__section-heading">
    <h2 id={`${id}-models`}>Local models</h2>
  </header>
  <div class="store-settings__rows">
    <dl class="store-settings__summary-rows">
      <StoreReadOnlyRow label="Status" value={status.state} live="status" />
      <StoreReadOnlyRow
        label="Native runtime"
        value={status.runtime_prepared ? "Prepared" : "Requires preparation"}
      />
      {#if status.progress}
        <StoreReadOnlyRow
          label="Progress"
          live="status"
          value={`${String(progress.stage ?? "Downloading")}${
            progress.total_bytes
              ? ` · ${Math.round((Number(progress.downloaded_bytes ?? 0) / Number(progress.total_bytes)) * 100)}%`
              : ""
          }`}
        />
      {/if}
    </dl>
    <Field.Group class="store-settings__editable-rows">
      <StoreWorkField
        class="store-settings__option-row"
        label="Model"
        id={`${id}-model`}
        description="Download weights explicitly, then start the model on this machine."
      >
        <select
          id={`${id}-model`}
          bind:value={draft.model}
          disabled={blocked || status.state === "serving"}
        >
          <option value={null}>Choose a model</option>
          {#each status.models as model (model.id)}
            <option value={model.id}
              >{model.name}{model.installed ? " · Installed" : ""}</option
            >
          {/each}
        </select>
      </StoreWorkField>
      <StoreWorkField
        class="store-settings__option-row"
        label="Custom model"
        id={`${id}-custom`}
        description="Hugging Face GGUF model reference, such as repository:quantization."
      >
        <Input
          id={`${id}-custom`}
          value={draft.model ?? ""}
          disabled={blocked || status.state === "serving"}
          oninput={(event) => {
            draft.model = event.currentTarget.value || null;
          }}
        />
      </StoreWorkField>
    </Field.Group>
    <div
      class="store-settings__action-row store-settings__action-row--joined store-settings__action-row--full-width"
    >
      <div>
        <strong>Run a local model</strong>
        <p>
          Prepare the runtime, download weights, then start inference. Downloads
          stay on this machine.
        </p>
      </div>
      <div class="store-settings__actions store-settings__actions--wrapping">
        <Button
          variant="outline"
          disabled={blocked || !status.supported}
          onclick={() => management?.command({ action: "prepare" })}
          >Prepare runtime</Button
        >
        <Button
          variant="outline"
          disabled={blocked || !draft.model || status.state === "serving"}
          onclick={async () => {
            if (await onConfigure())
              await management?.command({ action: "download" });
          }}>Download model</Button
        >
        <Button
          disabled={blocked ||
            !status.runtime_prepared ||
            !status.models.some(
              (model) => model.id === draft.model && model.installed,
            ) ||
            status.state === "serving"}
          onclick={async () => {
            if (await onConfigure())
              await management?.command({ action: "start" });
          }}>Start model</Button
        >
        <Button
          variant="outline"
          disabled={management?.busy || status.state === "off"}
          onclick={() => management?.command({ action: "stop" })}
          >Stop compute</Button
        >
      </div>
    </div>
  </div>
</section>

<section class="store-settings__section" aria-labelledby={`${id}-storage`}>
  <header class="store-settings__section-heading">
    <h2 id={`${id}-storage`}>Model storage</h2>
  </header>
  <div class="store-settings__rows">
    <Field.Group class="store-settings__editable-rows">
      <StoreWorkField
        class="store-settings__option-row"
        label="Import GGUF"
        id={`${id}-import`}
        description="Absolute path on the server machine. The app copies weights into its managed storage."
      >
        <StoreModelFilePicker
          id={`${id}-import`}
          bind:value={importPath}
          supported={status.model_file_picker}
          disabled={blocked ||
            management?.modelPicking ||
            status.state === "serving"}
          onBrowse={async () => (await management?.pickModelFile()) ?? null}
        />
      </StoreWorkField>
    </Field.Group>
    <div class="store-settings__action-row store-settings__action-row--joined">
      <div>
        <strong>Import model weights</strong>
        <p>Imported models remain available after restart.</p>
      </div>
      <div class="store-settings__actions">
        <Button
          variant="outline"
          disabled={blocked ||
            management?.modelPicking ||
            !importPath ||
            status.state === "serving"}
          onclick={() =>
            management?.command({ action: "import", path: importPath })}
          >Import model</Button
        >
      </div>
    </div>
    {#each status.models.filter((model) => model.installed) as model (model.id)}
      <div
        class="store-settings__action-row store-settings__action-row--joined"
      >
        <div>
          <strong>{model.name}</strong>
          <p>Installed model weights on this machine.</p>
        </div>
        <div class="store-settings__actions">
          <Button
            variant="outline"
            disabled={blocked || status.state === "serving"}
            onclick={() =>
              management?.command({ action: "remove", model: model.id })}
            >Remove weights</Button
          >
        </div>
      </div>
    {/each}
  </div>
  <StoreNotice
    noticeKey="compute-model-picker"
    title="Model file picker unavailable"
    message={management?.modelPickerError}
  />
</section>
