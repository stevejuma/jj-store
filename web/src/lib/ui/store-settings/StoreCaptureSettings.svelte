<script lang="ts">
  import { captureSummary } from "../../controller/capture-presentation.js";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { CopyableValue } from "@lapismd/design-core/workspace/settings";
  import type { CaptureManagement } from "../../controller/capture-types.js";
  import StoreCaptureDiagnostics from "./StoreCaptureDiagnostics.svelte";
  import StoreCaptureSources from "./StoreCaptureSources.svelte";
  import StoreCaptureImport from "./StoreCaptureImport.svelte";
  let {
    management,
    repositoryName = () => "Previously bound repository",
  }: {
    management?: CaptureManagement;
    repositoryName?: (id: string) => string;
  } = $props();
  const state = $derived(management?.state);
  $effect(() => {
    const owner = management;
    owner?.setActive(owner.supported);
    return () => owner?.setActive(false);
  });
  const ready = $derived(
    management?.supported &&
      state?.status === "ready" &&
      state.value?.enabled &&
      state.value.policy &&
      !state.pending &&
      !state.busy,
  );
</script>

<section
  class="store-settings__section"
  aria-labelledby="capture-settings-heading"
>
  <header>
    <h2 id="capture-settings-heading">Agent transcript capture</h2>
  </header>
  {#if !management?.supported || !state}
    <p>Capture management is not available for this connection.</p>
  {:else}
    <p>
      Watch configured harness locations for active sessions linked to this
      server. Settings apply to the whole local service. Status and published
      Agents update automatically.
    </p>
    <div class="store-settings__actions">
      <Button
        variant="outline"
        disabled={state.busy}
        onclick={() => management.refresh()}>Refresh capture</Button
      >
      {#if state.value?.enabled}
        <Button
          variant="outline"
          disabled={!ready}
          onclick={() =>
            management.control(
              state.value?.policy?.paused ? "resume" : "pause",
            )}
          >{state.value.policy?.paused
            ? "Resume capture"
            : "Pause capture"}</Button
        >
        <Button
          variant="outline"
          disabled={!ready || state.value.policy?.paused}
          onclick={() => management.rescan()}>Rescan capture</Button
        >
      {/if}
    </div>
    {#if state.status === "loading" || state.status === "refreshing"}<p
        role="status"
      >
        Loading capture status…
      </p>{/if}
    {#if state.value?.enabled}
      <p role="status">{captureSummary(state.value)}</p>
      <StoreCaptureDiagnostics value={state.value} />
      <p>
        Pause policy persists across restarts. Worker status updates every five
        seconds while this view is open.
      </p>
      <StoreCaptureSources {management} {repositoryName} />
    {:else if state.value}<p>
        Transcript capture is disabled in server configuration.
      </p>
      <p>
        Enable selected Codex profiles in the server’s <code>agent_capture</code
        >
        configuration, then restart this local backend. Only active sessions in linked
        repository folders are captured.
      </p>{:else}<p>Loading service coverage…</p>{/if}
    {#if state.pending?.uncertain}
      <Alert.Root
        ><Alert.Title>Capture outcome unknown</Alert.Title><Alert.Description
          >Retain the original request. A missing receipt or refused retry does
          not prove the original command failed.</Alert.Description
        ></Alert.Root
      >
      <CopyableValue
        label="Capture request ID"
        value={state.pending.command.request_id}
      />
      <div class="store-settings__actions">
        <Button
          variant="outline"
          disabled={state.busy}
          onclick={() => management.retry()}
          >Retry original capture request</Button
        >
        {#if state.pending.kind === "import"}<Button
            variant="outline"
            disabled={state.busy}
            onclick={() =>
              management.inspectImport(state.pending!.command.request_id)}
            >Check original import receipt</Button
          >{/if}
      </div>
    {/if}
    {#if state.controlOutcome}<p>
        Original policy receipt: {state.controlOutcome.receipt.paused
          ? "pause"
          : "resume"} · {state.controlOutcome.replayed
          ? "Replayed"
          : "Accepted"} · {state.controlOutcome.runtime_requested
          ? "Worker change requested"
          : "No worker change requested"}
      </p>{/if}
    <StoreCaptureImport {management} />
    {#if state.message}<p role="status">{state.message}</p>{/if}
    {#if state.error}<Alert.Root variant="destructive"
        ><Alert.Title>Capture request unavailable</Alert.Title
        ><Alert.Description>{state.error}</Alert.Description></Alert.Root
      >{/if}
  {/if}
</section>
