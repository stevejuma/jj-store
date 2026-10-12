<script lang="ts">
  import { globalSettingsSkeletonCards } from "./global-settings-skeleton.js";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/gauge";
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreNumericSettingRow from "./StoreNumericSettingRow.svelte";
  import { countUnits } from "./numeric-setting.js";
  import StoreSettingsPage from "./StoreSettingsPage.svelte";
  import StoreComputeHardware from "./StoreComputeHardware.svelte";
  import StoreComputeModels from "./StoreComputeModels.svelte";
  import StoreComputePeers from "./StoreComputePeers.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import type { AgentCompute } from "../../controller/agent-compute.svelte.js";
  import type { Model } from "../../api/client.js";
  let {
    management,
    active = $bindable("models"),
  }: { management?: AgentCompute; active?: string } = $props();
  let draft = $state<Model<"ComputeSettings"> | null>(
    untrack(() => management?.draft ?? null),
  );
  const id = $props.id();
  const status = $derived(management?.value);
  const blocked = $derived(management?.busy || !!management?.pending);
  const memoryInvalid = $derived(
    draft?.memory_gb !== null &&
      draft?.memory_gb !== undefined &&
      (!Number.isSafeInteger(draft.memory_gb) ||
        draft.memory_gb < 1 ||
        draft.memory_gb > 1024),
  );
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
    return draft && !memoryInvalid
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
      <StoreSettingsCard
        id={`${id}-resources`}
        title="Resource limits"
        Icon={SectionIcon}
      >
        <div class="store-settings__rows">
          <Field.Group class="store-settings__editable-rows">
            <StoreNumericSettingRow
              label="Maximum memory / VRAM"
              id={`${id}-memory`}
              description="One loaded model and two concurrent inference requests per host. Leave empty for automatic memory allocation."
              min={1}
              max={1024}
              slider
              sliderMax={64}
              optional
              units={countUnits("GB")}
              value={draft.memory_gb === null ? "" : String(draft.memory_gb)}
              disabled={blocked || status.state === "serving"}
              onValueChange={(value) => {
                draft!.memory_gb = value.trim()
                  ? Number.isSafeInteger(Number(value))
                    ? Number(value)
                    : -1
                  : null;
              }}
            />
          </Field.Group>
        </div>
      </StoreSettingsCard>
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
    <div role="region" aria-label="Compute configuration">
      <StoreSettingsFooter standalone>
        {#snippet description()}<div>
            <strong>Save compute configuration</strong>
            <p>
              Save resource limits and peer selection. Local models never fall
              back to cloud inference.
            </p>
          </div>{/snippet}

        <Button disabled={blocked || memoryInvalid} onclick={configure}
          >Save compute settings</Button
        >
      </StoreSettingsFooter>
    </div>
  {:else if !management?.error && management}<StoreSettingsSkeleton
      heading={false}
      label="Loading compute settings"
      cards={globalSettingsSkeletonCards("compute", active)}
    />{/if}
</StoreSettingsPage>
