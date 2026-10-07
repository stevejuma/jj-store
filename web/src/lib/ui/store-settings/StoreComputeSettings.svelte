<script lang="ts">
  import { untrack } from "svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreComputeHardware from "./StoreComputeHardware.svelte";
  import StoreComputeModels from "./StoreComputeModels.svelte";
  import StoreComputePeers from "./StoreComputePeers.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { AgentCompute } from "../../controller/agent-compute.svelte.js";
  import type { Model } from "../../api/client.js";
  import { object } from "../../controller/agent-execution-types.js";
  let { management }: { management?: AgentCompute } = $props();
  let draft = $state<Model<"ComputeSettings"> | null>(
    untrack(() => management?.draft ?? null),
  );
  const id = $props.id();
  const status = $derived(management?.value);
  const progress = $derived(object(status?.progress));
  const blocked = $derived(management?.busy || !!management?.pending);
  $effect(() => {
    if (management) {
      const owner = management;
      untrack(() => owner.start());
      return () => owner.stop();
    }
  });
  $effect(() => {
    const settings = status?.settings;
    if (settings)
      untrack(() => {
        if (
          !draft ||
          (!management?.pending && draft.revision !== settings.revision)
        ) {
          draft = structuredClone($state.snapshot(settings));
          if (management) management.draft = draft;
        }
      });
  });
  async function configure() {
    return draft
      ? ((await management?.command({
          action: "configure",
          settings: $state.snapshot(draft),
        })) ?? false)
      : false;
  }
</script>

<StoreSettingsPage
  section="compute"
  onRefresh={async () => {
    await management?.refresh();
  }}
  refreshLabel="Refresh compute"
  refreshDisabled={management?.busy}
>
  <StoreNotice
    noticeKey="compute-command"
    title="Compute needs attention"
    message={management?.error || status?.error || ""}
  />
  {#if !status?.supported}<p>
      The compute runtime must be included with this local installation.
    </p>{/if}
  {#if status && draft}
    <StoreComputeHardware value={status.hardware} />
    <h2>Local models</h2>
    <Field.Group class="community-settings__option-group">
      <StoreWorkField
        label="Model"
        id={`${id}-model`}
        description="Download weights explicitly, then start the model on this machine."
      >
        <select
          id={`${id}-model`}
          bind:value={draft.model}
          disabled={blocked || status.state === "serving"}
        >
          <option value={null}>Choose a model</option
          >{#each status.models as model (model.id)}<option value={model.id}
              >{model.name}{model.installed ? " · Installed" : ""}</option
            >{/each}
        </select>
      </StoreWorkField>
      <StoreWorkField
        label="Custom model"
        id={`${id}-custom`}
        description="Hugging Face GGUF model reference, such as repository:quantization."
      >
        <Input
          id={`${id}-custom`}
          value={draft.model ?? ""}
          disabled={blocked || status.state === "serving"}
          oninput={(event) => {
            draft!.model = event.currentTarget.value || null;
          }}
        />
      </StoreWorkField>
      <StoreWorkField label="Status"
        ><span role="status">{status.state}</span></StoreWorkField
      >
      {#if status.progress}<p role="status">
          {String(progress.stage ?? "Downloading")}
          {progress.total_bytes
            ? `${Math.round((Number(progress.downloaded_bytes ?? 0) / Number(progress.total_bytes)) * 100)}%`
            : ""}
        </p>{/if}
      <div class="store-settings__action-row">
        <p>
          Native runtime {status.runtime_prepared
            ? "prepared"
            : "requires preparation"}.
        </p>
        <Button
          disabled={blocked || !status.supported}
          onclick={() => management?.command({ action: "prepare" })}
          >Prepare runtime</Button
        >
      </div>
      <div class="store-settings__action-row">
        <p>Model downloads stay on this machine.</p>
        <Button
          variant="outline"
          disabled={blocked || !draft.model || status.state === "serving"}
          onclick={async () => {
            if (await configure())
              await management?.command({ action: "download" });
          }}>Download model</Button
        >
        <Button
          disabled={blocked ||
            !status.runtime_prepared ||
            !status.models.some(
              (model) => model.id === draft!.model && model.installed,
            ) ||
            status.state === "serving"}
          onclick={async () => {
            if (await configure())
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
    </Field.Group>
    <StoreComputeModels {management} {status} {blocked} />
    <h2>Resource limits</h2>
    <Field.Group class="community-settings__option-group"
      ><StoreWorkField
        label="Maximum memory / VRAM"
        id={`${id}-memory`}
        description="GB. One loaded model and two concurrent inference requests per host."
      >
        <Input
          id={`${id}-memory`}
          type="number"
          min={1}
          max={1024}
          value={draft.memory_gb ?? ""}
          disabled={blocked || status.state === "serving"}
          oninput={(event) => {
            draft!.memory_gb = event.currentTarget.value
              ? Number(event.currentTarget.value)
              : null;
          }}
        />
      </StoreWorkField></Field.Group
    >
    <StoreComputePeers
      {management}
      {status}
      {blocked}
      bind:draft
      onConfigure={configure}
    />
  {/if}
</StoreSettingsPage>
