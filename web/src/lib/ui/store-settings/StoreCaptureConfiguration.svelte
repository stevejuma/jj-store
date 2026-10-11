<script lang="ts">
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreReadOnlyRow from "./StoreReadOnlyRow.svelte";
  import StoreNumericSettingRow from "./StoreNumericSettingRow.svelte";
  import { secondsUnits, bytesUnits, countUnits } from "./numeric-setting.js";
  import ActivityIcon from "@lucide/svelte/icons/activity";
  import SlidersIcon from "@lucide/svelte/icons/sliders-horizontal";
  import BotIcon from "@lucide/svelte/icons/bot";
  import StoreCaptureHarnessConfiguration from "./StoreCaptureHarnessConfiguration.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import type { CaptureManagement } from "../../controller/capture-types.js";
  import type { CaptureDraft } from "../../controller/capture-configuration.js";
  let { management }: { management: CaptureManagement } = $props();
  const settings = $derived(management.state.settings);
  const busy = $derived(
    management.state.commandBusy ||
      settings.saving ||
      !!settings.unknown ||
      !!management.state.pending,
  );
  const limits: {
    key: keyof Pick<CaptureDraft, "readers" | "interval" | "sources" | "bytes">;
    label: string;
    max: number;
    help: string;
  }[] = [
    {
      key: "readers",
      label: "Concurrent readers",
      max: 4,
      help: "Maximum transcript readers active at once (1–4).",
    },
    {
      key: "interval",
      label: "Discovery interval (seconds)",
      max: 30,
      help: "Check configured sources every 1–30 seconds.",
    },
    {
      key: "sources",
      label: "Tracked source limit",
      max: 4096,
      help: "Maximum retained source identities (1–4,096). Lowering this does not remove existing sources.",
    },
    {
      key: "bytes",
      label: "Pending byte budget",
      max: 67108864,
      help: "Durable pending transcript capacity in bytes (up to 64 MiB). Lowering this does not delete queued content.",
    },
  ];
</script>

<StoreSettingsCard
  id="capture-configuration-title"
  title="Capture configuration"
  layout="page"
  description="Settings apply to this local installation after the server restarts."
>
  {#if settings.value && settings.draft}
    <form
      onsubmit={(event) => {
        event.preventDefault();
        void management.saveConfiguration();
      }}
    >
      <Field.Group class="store-settings__form-sections">
        <StoreSettingsCard
          id="capture-config-status"
          title="Capture status"
          Icon={ActivityIcon}
        >
          <dl class="store-settings__rows">
            <StoreReadOnlyRow
              label="Saved capture"
              value={settings.value.desired.enabled ? "Enabled" : "Disabled"}
            />
            <StoreReadOnlyRow
              label="Currently configured"
              value={`${settings.value.effective.enabled ? "Enabled" : "Disabled"} · ${settings.value.restart_required ? "Restart required" : "Matches saved settings"}`}
            />
          </dl></StoreSettingsCard
        >
        <StoreSettingsCard
          id="capture-config-options"
          title="Capture options"
          Icon={SlidersIcon}
        >
          <Field.Group class="store-settings__editable-rows">
            <Field.Field
              orientation="horizontal"
              class="store-settings__option-row"
            >
              <Field.Content
                ><Field.Label for="capture-enabled"
                  >Enable automatic capture</Field.Label
                ><Field.Description id="capture-enabled-help"
                  >Capture active sessions only when their repository or
                  workspace is linked to this server.</Field.Description
                ></Field.Content
              >
              <Switch
                id="capture-enabled"
                aria-describedby="capture-enabled-help"
                checked={settings.draft.enabled}
                disabled={busy}
                onCheckedChange={(enabled) => management.configure({ enabled })}
              />
            </Field.Field>
            {#each limits as f}
              <StoreNumericSettingRow
                id={`capture-${f.key}`}
                label={f.label}
                description={f.help}
                min={1}
                max={f.max}
                disabled={busy}
                value={settings.draft[f.key]}
                units={f.key === "interval"
                  ? secondsUnits
                  : f.key === "bytes"
                    ? bytesUnits
                    : countUnits(f.key === "readers" ? "readers" : "sources")}
                onValueChange={(value) =>
                  management.configure({ [f.key]: value })}
              />
            {/each}
          </Field.Group>
        </StoreSettingsCard>
        <StoreSettingsCard
          id="capture-config-harnesses"
          title="Harnesses"
          Icon={BotIcon}
          label="Harness configuration"
        >
          <StoreCaptureHarnessConfiguration {management} {busy} />
          <StoreCaptureHarnessConfiguration
            {management}
            {busy}
            harness="cursor"
          />
          <StoreCaptureHarnessConfiguration
            {management}
            {busy}
            harness="open_code"
          />
        </StoreSettingsCard>
        <StoreSettingsFooter standalone>
          {#snippet description()}<div>
              <strong>Save configuration</strong>
              <p>
                Restart the local server to activate saved changes. In the
                desktop app, quit and reopen the app. Existing capture history
                and pause policy are retained.
              </p>
            </div>{/snippet}

          <Button
            type="button"
            variant="outline"
            disabled={settings.saving ||
              !settings.dirty ||
              (!!settings.unknown && !settings.unknownObserved)}
            onclick={() => management.cancelConfiguration()}
            >{settings.conflict || settings.unknown
              ? "Use saved configuration"
              : "Cancel changes"}</Button
          >
          <Button
            type="submit"
            disabled={busy || !settings.dirty || settings.conflict}
            >Save configuration</Button
          >
        </StoreSettingsFooter>
      </Field.Group>
    </form>
    {#if settings.validation}<p role="alert">{settings.validation}</p>{/if}
    {#if settings.conflict && !settings.unknown}<StoreSettingsNotice
        >Server configuration changed while you were editing. Your draft is
        retained. Refresh to inspect the latest saved configuration, then use
        saved configuration before editing again.</StoreSettingsNotice
      >{/if}
    {#if settings.unknown}<StoreSettingsNotice
        >Save outcome unknown. Refresh to compare the saved configuration with
        your original draft. A different configuration does not prove that the
        original save failed.</StoreSettingsNotice
      >{/if}
  {:else if settings.error}<StoreSettingsNotice
      >Capture configuration is unavailable. Refresh settings to retry.</StoreSettingsNotice
    >{:else}<StoreSettingsSkeleton
      heading={false}
      label="Loading capture configuration"
      cards={[
        { rows: ["fact", "fact"] },
        { rows: ["toggle", "numeric", "numeric", "numeric", "numeric"] },
        { rows: ["input", "input", "input"], actions: 2 },
      ]}
    />{/if}
  <StoreNotice
    scope="installation-capture"
    noticeKey="capture-settings-error"
    title="Agent configuration needs attention"
    message={settings.error}
    pending={settings.saving}
  />
  <StoreNotice
    scope="installation-capture"
    noticeKey="capture-settings-saved"
    title="Agent configuration"
    message={settings.message}
    severity="info"
    lifetime="timed"
  />
</StoreSettingsCard>
