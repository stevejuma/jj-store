<script lang="ts">
  import StoreAgentMentionPreference from "./StoreAgentMentionPreference.svelte";
  import StoreSettingsSelect from "./StoreSettingsSelect.svelte";
  import StoreSettingsDisclosure from "./StoreSettingsDisclosure.svelte";
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  import { untrack, type Snippet } from "svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreAgentEnvironmentEditor from "../store-agent-execution/StoreAgentEnvironmentEditor.svelte";
  import { runtimeChoices } from "../store-agent-execution/presentation.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSettingsValue } from "../../controller/agent-settings.svelte.js";
  import CommunityAgentEditorModelPicker from "../community-agent-editor/CommunityAgentEditorModelPicker.svelte";
  import {
    harnessCatalog,
    harnessModelRows,
  } from "../store-agent-execution/harness-catalog.js";
  import type { EditorModelRow } from "../../community/agent-editor-model-rows.js";
  let {
    manager,
    runtimeSection,
  }: { manager?: AgentExecution; runtimeSection?: Snippet } = $props();
  let draft = $state<AgentSettingsValue | null>(
    untrack(() => manager?.settings.draft ?? null),
  );
  let secrets = $state(untrack(() => manager?.settings.secretDrafts ?? {}));
  const id = $props.id();
  $effect(() => {
    const value = manager?.settings.value;
    if (value)
      untrack(() => {
        if (!draft) {
          draft = structuredClone($state.snapshot(value));
          if (manager) manager.settings.draft = draft;
        }
      });
  });
  function loadModels(runner: string, retry = false) {
    if (
      manager?.value?.runtime === "ready" &&
      manager.value.runners.some((r) => r.id === runner && r.available)
    )
      void manager.catalogs.load(runner, retry);
  }
  $effect(() => {
    const runner = draft?.runner;
    const available =
      manager?.value?.runtime === "ready" &&
      manager.value.runners.some((r) => r.id === runner && r.available);
    if (runner && available) untrack(() => loadModels(runner));
  });
  const catalog = $derived(
    harnessCatalog(
      manager?.value?.sessions ?? [],
      draft?.runner ?? "",
      manager?.catalogs.values[draft?.runner ?? ""]?.facts,
    ),
  );
  const facts = $derived(catalog.facts);
  const models = $derived(
    harnessModelRows(
      manager?.value ?? null,
      (runner) => manager?.catalogs.values[runner]?.facts,
      draft?.runner ?? "",
      draft?.preferences.model,
    ),
  );
  function selectModel(row: EditorModelRow) {
    if (!draft) return;
    if (row.runtimeAgent !== draft.runner)
      draft.preferences = { model: null, effort: null, mode: null };
    draft.runner = row.runtimeAgent;
    draft.preferences.model = row.model || null;
  }
  const modes = $derived(runtimeChoices(facts, "mode"));
  const thinking = $derived(runtimeChoices(facts, "thought_level"));
  const busy = $derived(manager?.settings.busy || !!manager?.settings.pending);
</script>

{#if manager && draft}
  <StoreAgentMentionPreference settings={manager.settings} />
  <section
    class="store-settings__section"
    aria-labelledby={`${id}-preferences`}
  >
    <header class="store-settings__section-heading">
      <h2 id={`${id}-preferences`}>Preferences</h2>
    </header>
    <Field.Group class="community-settings__option-group">
      <StoreWorkField
        label="Keep awake while agents are active"
        id={`${id}-awake`}
        description="Desktop only. Releases when agents stop or after one hour without agent activity."
      >
        <Switch
          id={`${id}-awake`}
          bind:checked={draft.keep_awake}
          disabled={busy || !isMacDesktopWindow()}
        />
      </StoreWorkField>
    </Field.Group>
  </section>
{/if}
{@render runtimeSection?.()}
{#if manager && draft}
  <section class="store-settings__section" aria-labelledby={`${id}-defaults`}>
    <header class="store-settings__section-heading">
      <div>
        <h2 id={`${id}-defaults`}>Agent defaults</h2>
        <p>
          Runtime preferences and environment settings inherited by new local
          agents. Agent overrides take priority.
        </p>
      </div>
    </header>
    <div class="store-settings__rows">
      <Field.Group class="store-settings__editable-rows">
        <div class="store-settings__model-picker-row">
          <CommunityAgentEditorModelPicker
            id={`${id}-model`}
            pending={busy}
            rows={models}
            runtimeAgent={draft.runner}
            model={draft.preferences.model ?? ""}
            onSelect={selectModel}
            onHarnessChange={loadModels}
          />
          <p class="store-settings__row-description">
            Selected model: {models.find(
              (r) =>
                r.runtimeAgent === draft!.runner &&
                r.model === (draft!.preferences.model ?? ""),
            )?.name ?? draft.preferences.model}
          </p>
        </div>
      </Field.Group>
      <div
        class="store-settings__action-row store-settings__action-row--joined"
      >
        <div>
          <p class="store-settings__row-description" role="status">
            {manager.catalogs.loading[draft.runner]
              ? "Loading models from the installed harness…"
              : manager.catalogs.errors[draft.runner] ||
                (catalog.remembered
                  ? "Showing choices from this harness’s last session."
                  : catalog.session?.connected
                    ? "Choices advertised by the connected harness."
                    : "Choices advertised by the installed harness.")}
          </p>
        </div>
        <div class="store-settings__actions">
          {#if manager.catalogs.errors[draft.runner]}<Button
              variant="outline"
              size="sm"
              disabled={manager.catalogs.loading[draft.runner]}
              onclick={() => loadModels(draft!.runner, true)}
              >Retry models</Button
            >{/if}
        </div>
      </div>
      <Field.Group class="store-settings__editable-rows">
        {#if draft.runner === "local"}
          <StoreWorkField label="Provider" id={`${id}-provider`}>
            <StoreSettingsSelect
              id={`${id}-provider`}
              bind:value={draft.options.provider}
              disabled={busy}
              options={[
                { value: null, label: "Harness default" },
                { value: "local", label: "This machine" },
                { value: "paired", label: "Paired compute" },
                { value: "openai", label: "OpenAI compatible endpoint" },
              ]}
            />
          </StoreWorkField>
        {/if}
        {#if draft.runner === "local" && draft.options.provider === "openai"}
          <StoreWorkField
            label="Provider endpoint"
            id={`${id}-endpoint`}
            description="OpenAI compatible base URL. Add credentials as a secret or server environment reference in Advanced."
            ><Input
              id={`${id}-endpoint`}
              value={draft.options.endpoint ?? ""}
              disabled={busy}
              oninput={(event) => {
                draft!.options.endpoint = event.currentTarget.value || null;
              }}
            /></StoreWorkField
          >
        {/if}
        {#if thinking.length}<StoreWorkField
            label="Thinking"
            id={`${id}-effort`}
          >
            <StoreSettingsSelect
              id={`${id}-effort`}
              bind:value={draft.preferences.effort}
              disabled={busy}
              options={[
                { value: null, label: "Harness default" },
                ...thinking.map((option) => ({
                  value: option.id,
                  label: option.name,
                })),
              ]}
            />
          </StoreWorkField>{/if}
        {#if modes.length}<StoreWorkField
            label="Execution mode"
            id={`${id}-mode`}
          >
            <StoreSettingsSelect
              id={`${id}-mode`}
              bind:value={draft.preferences.mode}
              disabled={busy}
              options={[
                { value: null, label: "Harness default" },
                ...modes.map((mode) => ({ value: mode.id, label: mode.name })),
              ]}
            /></StoreWorkField
          >{/if}
        <StoreSettingsDisclosure>
          <h3>Environment variables</h3>
          <StoreAgentEnvironmentEditor
            keychainSupported={manager.settings.value?.keychain_supported ??
              false}
            bind:value={draft.options.environment}
            bind:secrets
            disabled={busy}
          />
        </StoreSettingsDisclosure>
      </Field.Group>
      <div
        class="store-settings__action-row store-settings__action-row--joined"
      >
        <p>Saved changes take effect on the next start.</p>
        <Button
          disabled={busy}
          onclick={async () => {
            if (await manager.settings.save(draft!, secrets)) {
              manager.catalogs.invalidate();
              void manager.catalogs.load(draft!.runner);
              draft = structuredClone($state.snapshot(manager.settings.value!));
              manager.settings.draft = draft;
            }
          }}>Save defaults</Button
        >
      </div>
    </div>
    <StoreNotice
      noticeKey="agent-defaults-command"
      title="Agent defaults need attention"
      message={manager.settings.error}
    />
    {#if manager.settings.pending}<Button
        variant="outline"
        onclick={() => manager!.settings.retry()}>Check retained save</Button
      >{/if}
    {#if manager.settings.value && manager.settings.value.revision !== draft.revision && !manager.settings.pending}
      <Button
        variant="outline"
        onclick={() => {
          draft!.revision = manager!.settings.value!.revision;
        }}>Keep draft and use latest version</Button
      >
    {/if}
  </section>
{/if}

<style>
  .store-settings__model-picker-row {
    min-width: 0;
    padding: 1rem;
  }
</style>
