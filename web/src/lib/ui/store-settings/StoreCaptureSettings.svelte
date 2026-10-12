<script lang="ts">
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import InfoIcon from "@lucide/svelte/icons/info";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import SearchIcon from "@lucide/svelte/icons/search";
  import BotIcon from "@lucide/svelte/icons/bot";
  import PauseIcon from "@lucide/svelte/icons/pause";
  import PlayIcon from "@lucide/svelte/icons/play";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import { captureSummary } from "../../controller/capture-presentation.js";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CopyableValue } from "@lapismd/design-core/workspace/settings";
  import type { CaptureManagement } from "../../controller/capture-types.js";
  import StoreCaptureConfiguration from "./StoreCaptureConfiguration.svelte";
  import StoreCaptureRoots from "./StoreCaptureRoots.svelte";
  import StoreCaptureDiagnostics from "./StoreCaptureDiagnostics.svelte";
  import StoreCaptureSources from "./StoreCaptureSources.svelte";
  import StoreCaptureImport from "./StoreCaptureImport.svelte";
  import StoreCaptureLoading from "./StoreCaptureLoading.svelte";
  let {
    management,
    section = $bindable("capture"),
    onRepository,
    repositoryNavigationReason,
    repositoryName = () => "Previously bound repository",
  }: {
    management?: CaptureManagement;
    section?: string;
    repositoryName?: (id: string) => string;
    onRepository?: (id: string) => void;
    repositoryNavigationReason?: string;
  } = $props();
  const state = $derived(management?.state);
  $effect(() => {
    const owner = management;
    owner?.setActive(owner.supported);
    return () => owner?.setActive(false);
  });
  const ready = $derived(
    !state?.settings.saving &&
      !state?.settings.value?.restart_required &&
      management?.supported &&
      state?.status === "ready" &&
      state.value?.enabled &&
      state.value.policy &&
      !state.pending &&
      !state.busy,
  );
</script>

<div
  class="store-settings-group"
  role="region"
  aria-label="Agent capture settings"
>
  {#if section === "capture"}
    <StoreSettingsCard
      id="capture-policy"
      title="Agent transcript capture"
      Icon={BotIcon}
      description="Watch configured harness locations for active sessions linked to this server. Settings apply to the whole local service. Status and published Agents update automatically."
    >
      <StoreSettingsFooter>
        {#snippet description()}<div>
            <strong>Automatic transcript capture</strong>
            <p>
              Pause policy persists across restarts. Worker status updates every
              five seconds while this view is open.
            </p>
          </div>{/snippet}
        <Button
          variant="outline"
          disabled={!ready}
          onclick={() =>
            management?.control(
              state?.value?.policy?.paused ? "resume" : "pause",
            )}
        >
          {#if state?.value?.policy?.paused}<PlayIcon
              data-icon="inline-start"
              aria-hidden="true"
            />{:else}<PauseIcon
              data-icon="inline-start"
              aria-hidden="true"
            />{/if}
          {state?.value?.policy?.paused ? "Resume capture" : "Pause capture"}
        </Button>
        <Button
          variant="outline"
          disabled={!ready || state?.value?.policy?.paused}
          onclick={() => management?.rescan()}
          ><RefreshCwIcon data-icon="inline-start" aria-hidden="true" />Rescan
          capture</Button
        >
      </StoreSettingsFooter></StoreSettingsCard
    >
  {/if}
  {#if !management?.supported || !state}
    <StoreSettingsEmpty
      Icon={BotIcon}
      title="Capture unavailable"
      description="Capture management is not available for this connection."
    />
  {:else}
    {#if section === "configuration"}<StoreCaptureConfiguration
        {management}
      />{:else}
      {#if state.settings.value?.restart_required}<StoreSettingsNotice
          >Saved capture settings require a configuration reload. Runtime
          controls are unavailable until reload.</StoreSettingsNotice
        >{/if}
      {#if state.value?.enabled}
        {#if section === "capture"}
          <StoreSettingsNotice
            >{captureSummary(state.value)}</StoreSettingsNotice
          >
          <StoreCaptureRoots value={state.value} />
        {:else if section === "sources"}
          <StoreCaptureSources
            {management}
            {repositoryName}
            {onRepository}
            {repositoryNavigationReason}
            onImport={() => {
              section = "imports";
            }}
          />
        {:else if section === "diagnostics"}
          <StoreCaptureDiagnostics value={state.value} />
        {/if}
      {:else if state.value}
        <StoreSettingsNotice>
          Transcript capture is disabled in server configuration.
        </StoreSettingsNotice>
        <dl class="store-settings__rows">
          <div>
            <dt>Enable transcript capture</dt>
            <dd>
              Choose harnesses and enable capture in the Configuration tab, then
              reload saved settings. Only active sessions in linked repository
              folders are captured.
            </dd>
          </div>
        </dl>
      {:else if section !== "sources" && section !== "imports"}
        {#if state.status === "failed"}<StoreSettingsEmpty
            Icon={BotIcon}
            title="Capture status unavailable"
            description="Status refresh will retry automatically while this page is open."
          />{:else}<StoreCaptureLoading
            label="Loading capture status"
            tab={section}
          />{/if}
      {/if}
      {#if section === "sources" && !state.value?.enabled && !state.value}
        <StoreCaptureSources
          {management}
          {repositoryName}
          {onRepository}
          {repositoryNavigationReason}
        />
      {/if}
      {#if state.pending?.uncertain}
        <Alert.Root role="status"
          ><InfoIcon aria-hidden="true" /><Alert.Title
            >Capture outcome unknown</Alert.Title
          ><Alert.Description
            >Retain the original request. A missing receipt or refused retry
            does not prove the original command failed.</Alert.Description
          ></Alert.Root
        >
        <StoreSettingsFooter>
          {#snippet description()}<div>
              <CopyableValue
                label="Capture request ID"
                value={state.pending!.command.request_id}
              />
            </div>{/snippet}

          <Button
            variant="outline"
            disabled={state.busy || state.settings.value?.restart_required}
            onclick={() => management.retry()}
            ><RotateCwIcon data-icon="inline-start" aria-hidden="true" />Retry
            original capture request</Button
          >
          {#if state.pending.kind === "import"}<Button
              variant="outline"
              disabled={state.busy}
              onclick={() =>
                management.inspectImport(state.pending!.command.request_id)}
              ><SearchIcon data-icon="inline-start" aria-hidden="true" />Check
              original import receipt</Button
            >{/if}
        </StoreSettingsFooter>
      {/if}
      {#if section === "capture" && state.controlOutcome}<StoreSettingsNotice>
          Original policy receipt: {state.controlOutcome.receipt.paused
            ? "pause"
            : "resume"} · {state.controlOutcome.replayed
            ? "Replayed"
            : "Accepted"} · {state.controlOutcome.runtime_requested
            ? "Worker change requested"
            : "No worker change requested"}
        </StoreSettingsNotice>{/if}
      {#if section === "imports"}<StoreCaptureImport {management} />{/if}
    {/if}
    <StoreNotice
      scope="installation-capture"
      noticeKey="capture-outcome"
      title="Agent capture"
      message={state.message}
      severity="info"
      lifetime="timed"
    />
    <StoreNotice
      scope="installation-capture"
      noticeKey="capture-request"
      title="Capture request unavailable"
      message={state.pending?.uncertain ? undefined : state.error}
      pending={state.busy || !!state.pending}
    />
  {/if}
</div>
