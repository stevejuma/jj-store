<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreComputeHardware from "./StoreComputeHardware.svelte";
  import StoreComputeModels from "./StoreComputeModels.svelte";
  import StoreComputePeers from "./StoreComputePeers.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { AgentCompute } from "../../controller/agent-compute.svelte.js";
  import type { Model } from "../../api/client.js";
  let { management }: { management?: AgentCompute } = $props();
  let draft = $state<Model<"ComputeSettings"> | null>(
    untrack(() => management?.draft ?? null),
  );
  let active = $state("models");
  const id = $props.id();
  const status = $derived(management?.value);
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
  bind:value={active}
  tabs={[
    { value: "models", label: "Models" },
    { value: "resources", label: "Resources" },
    { value: "sharing", label: "Sharing" },
  ]}
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
  {#if status?.supported === false}
    <p class="store-settings__empty">
      The compute runtime must be included with this local installation.
    </p>
  {/if}
  {#if status && draft}
    <Tabs.Content value="models">
      <StoreComputeModels
        {management}
        {status}
        {blocked}
        bind:draft
        onConfigure={configure}
      />
    </Tabs.Content>
    <Tabs.Content value="resources">
      <StoreComputeHardware value={status.hardware} />
      <section
        class="store-settings__section"
        aria-labelledby={`${id}-resources`}
      >
        <header class="store-settings__section-heading">
          <h2 id={`${id}-resources`}>Resource limits</h2>
        </header>
        <div class="store-settings__rows">
          <Field.Group class="store-settings__editable-rows">
            <StoreWorkField
              class="store-settings__option-row"
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
            </StoreWorkField>
          </Field.Group>
        </div>
      </section>
    </Tabs.Content>
    <Tabs.Content value="sharing">
      <StoreComputePeers
        {management}
        {status}
        {blocked}
        bind:draft
        onConfigure={configure}
      />
    </Tabs.Content>
    <section class="store-settings__section" aria-label="Compute configuration">
      <div class="store-settings__action-row">
        <div>
          <strong>Save compute configuration</strong>
          <p>
            Save resource limits and peer selection. Local models never fall
            back to cloud inference.
          </p>
        </div>
        <div class="store-settings__actions">
          <Button disabled={blocked} onclick={configure}
            >Save compute settings</Button
          >
        </div>
      </div>
    </section>
  {/if}
</StoreSettingsPage>
