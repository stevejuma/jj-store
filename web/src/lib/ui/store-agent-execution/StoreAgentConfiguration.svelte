<script lang="ts">
  import { untrack, onDestroy } from "svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreSettingsSelect from "../store-settings/StoreSettingsSelect.svelte";
  import StoreAgentHarnessSelect from "./StoreAgentHarnessSelect.svelte";
  import StoreAgentModelSelect from "./StoreAgentModelSelect.svelte";
  import { harnessCatalog } from "./harness-catalog.js";
  import { runtimeChoices } from "./presentation.js";
  import {
    canonicalProvider,
    changeHarness,
    changeProvider,
    changeEndpoint,
    type AgentConfiguration,
  } from "../../controller/agent-configuration.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import "../store-settings/StoreSettings.css";
  let {
    manager,
    draft = $bindable(),
    secrets = {},
    pending = false,
    active = true,
    scope,
  }: {
    manager: AgentExecution;
    draft: AgentConfiguration;
    secrets?: Record<string, string>;
    pending?: boolean;
    active?: boolean;
    scope: string;
  } = $props();
  const id = $props.id();
  const runner = $derived(
    manager.value?.runners.find((r) => r.id === draft.runner),
  );
  const providers = $derived(runner?.providers ?? []);
  const effectiveProvider = $derived(
    canonicalProvider(
      draft.options.provider ??
        (draft.options.inherit_defaults &&
        manager.settings.value?.runner === draft.runner
          ? manager.settings.value.options.provider
          : null),
    ),
  );
  const provider = $derived(providers.find((p) => p.id === effectiveProvider));
  const result = $derived(manager.catalogs.values[scope]);
  const harness = $derived(
    harnessCatalog(manager.value?.sessions ?? [], draft.runner, result?.facts),
  );
  const facts = $derived(
    draft.runner === "local" ? result?.facts : harness.facts,
  );
  const source = $derived(
    draft.runner === "local"
      ? ""
      : !runtimeChoices(facts, "model").length &&
          !runtimeChoices(facts, "thought_level").length
        ? "No model or thinking choices have been advertised."
        : harness.remembered
          ? "Showing choices from this harness's last session."
          : harness.session?.connected
            ? "Showing choices from this harness's live session."
            : "",
  );
  const models = $derived(runtimeChoices(facts, "model"));
  const thinking = $derived(runtimeChoices(facts, "thought_level"));
  const modes = $derived(runtimeChoices(facts, "mode"));
  const generation = $derived(
    JSON.stringify([
      manager.settings.value?.revision,
      manager.compute.value?.settings,
      manager.compute.value?.state,
      manager.compute.value?.models,
    ]),
  );
  function load(retry = false) {
    if (active && runner?.available && manager.value?.runtime === "ready")
      void manager.catalogs.preview(
        scope,
        $state.snapshot(draft),
        $state.snapshot(secrets),
        generation,
        retry,
      );
  }
  $effect(() => {
    const snapshot = JSON.stringify([
      draft.runner,
      draft.options,
      secrets,
      generation,
      active,
      runner?.available,
      manager.value?.runtime,
    ]);
    untrack(() => manager.catalogs.cancelPreview(scope));
    const timer = setTimeout(() => {
      void snapshot;
      load();
    }, 250);
    return () => clearTimeout(timer);
  });
  $effect(() => {
    if (active && effectiveProvider === "paired")
      untrack(() => {
        void manager.compute.refresh();
      });
  });
  onDestroy(() => manager.catalogs.cancelPreview(scope));
  const peerId = $derived(manager.compute.value?.settings.target_peer);
  const peer = $derived(
    peerId ? manager.compute.value?.settings.peers?.[peerId] : undefined,
  );
  const status = $derived(
    !runner?.available
      ? "This harness is unavailable. Open Settings → Agents → Runtimes."
      : manager.catalogs.loading[scope]
        ? "Loading models from this configuration…"
        : manager.catalogs.errors[scope] ||
          (result?.status === "authentication_required"
            ? "Authentication is required. Check the provider credentials in Advanced, then retry."
            : result?.status === "unavailable"
              ? "This provider is unavailable. Check its endpoint or Compute settings, then retry."
              : result?.status === "empty"
                ? "The provider returned no models. Your saved selection is retained."
                : result
                  ? "Models advertised by this configuration. Saved changes apply on the next start."
                  : "Choose a provider and configure credentials to discover models."),
  );
</script>

<div class="store-settings__rows" data-ui-part="agent-configuration-fields">
  <Field.Group class="store-settings__editable-rows">
    <StoreWorkField label="Harness" id={`${id}-harness`}>
      <StoreAgentHarnessSelect
        id={`${id}-harness`}
        value={draft.runner}
        disabled={pending}
        options={manager.value?.runners ?? []}
        onValueChange={(value) => changeHarness(draft, value)}
      />
    </StoreWorkField>
    {#if providers.length}
      <StoreWorkField
        label="LLM provider"
        id={`${id}-provider`}
        description={provider?.guidance ?? ""}
      >
        <StoreSettingsSelect
          id={`${id}-provider`}
          value={draft.options.provider == null
            ? null
            : canonicalProvider(draft.options.provider)}
          disabled={pending}
          options={[
            {
              value: null,
              label: `${draft.options.inherit_defaults ? "Application" : "Harness"} default (${provider?.label ?? "This machine"})`,
            },
            ...providers.map((p) => ({ value: p.id, label: p.label })),
          ]}
          onValueChange={(value) => changeProvider(draft, value)}
        />
      </StoreWorkField>
      {#if provider && !provider.managed}
        <StoreWorkField
          label={provider.endpoint_env === "DATABRICKS_HOST"
            ? "Workspace URL"
            : "Provider endpoint"}
          id={`${id}-endpoint`}
          description={provider.credential_env
            ? `Credential: ${provider.credential_env}. Configure it in Advanced.`
            : ""}
        >
          <Input
            id={`${id}-endpoint`}
            value={draft.options.endpoint ?? ""}
            placeholder={provider.default_endpoint ?? "https://"}
            disabled={pending}
            oninput={(event) =>
              changeEndpoint(draft, event.currentTarget.value)}
          />
        </StoreWorkField>
      {/if}
      {#if effectiveProvider === "paired"}
        <StoreWorkField
          label="Inference host"
          description="Selected in Settings → Compute → Sharing."
          ><span
            >{peer
              ? `${peer.name} · ${peerId}`
              : (peerId ?? "No trusted inference host selected")}</span
          ></StoreWorkField
        >
      {/if}
    {/if}
    <StoreWorkField label="Model" id={`${id}-model`}>
      <StoreAgentModelSelect
        id={`${id}-model`}
        bind:value={draft.preferences.model}
        options={models}
        disabled={pending}
        allowCustom={!!provider && !provider.managed}
        defaultLabel={provider?.managed
          ? "Host default model"
          : provider?.model_required
            ? "Use configured default model"
            : "Harness default"}
        onChange={() => {
          draft.preferences.effort = null;
        }}
      />
    </StoreWorkField>
    {#if thinking.length || draft.preferences.effort}
      <StoreWorkField label="Thinking" id={`${id}-thinking`}>
        <StoreSettingsSelect
          id={`${id}-thinking`}
          bind:value={draft.preferences.effort}
          disabled={pending}
          options={[
            { value: null, label: "Harness default" },
            ...thinking.map((o) => ({ value: o.id, label: o.name })),
            ...(draft.preferences.effort &&
            !thinking.some((o) => o.id === draft.preferences.effort)
              ? [
                  {
                    value: draft.preferences.effort,
                    label: `${draft.preferences.effort} (saved preference)`,
                    disabled: true,
                  },
                ]
              : []),
          ]}
        />
      </StoreWorkField>
    {/if}
    <StoreWorkField label="Execution mode" id={`${id}-mode`}>
      <StoreSettingsSelect
        id={`${id}-mode`}
        bind:value={draft.preferences.mode}
        disabled={pending || !modes.length}
        options={[
          { value: null, label: "Harness default" },
          ...modes.map((o) => ({ value: o.id, label: o.name })),
          ...(draft.preferences.mode &&
          !modes.some((o) => o.id === draft.preferences.mode)
            ? [
                {
                  value: draft.preferences.mode,
                  label: `${draft.preferences.mode} (saved preference)`,
                  disabled: true,
                },
              ]
            : []),
        ]}
      />
    </StoreWorkField>
  </Field.Group>
  <div class="store-settings__action-row store-settings__action-row--joined">
    <div><p role="status">{source} {status}</p></div>
    <div class="store-settings__actions">
      <Button
        type="button"
        variant="outline"
        size="sm"
        disabled={pending ||
          !runner?.available ||
          manager.catalogs.loading[scope]}
        onclick={() => load(true)}>Retry models</Button
      >
    </div>
  </div>
</div>
