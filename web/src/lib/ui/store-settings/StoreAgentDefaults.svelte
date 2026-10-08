<script lang="ts">
  import StoreSettingsDisclosure from "./StoreSettingsDisclosure.svelte";
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  import { untrack } from "svelte";
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
  let { manager }: { manager?: AgentExecution } = $props();
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
  $effect(() => {
    if (manager && draft?.runner)
      untrack(() => void manager?.catalogs.load(draft!.runner));
  });
  const facts = $derived(
    draft && manager?.catalogs.values[draft.runner]?.facts,
  );
  const models = $derived(runtimeChoices(facts, "model"));
  const modes = $derived(runtimeChoices(facts, "mode"));
  const thinking = $derived(runtimeChoices(facts, "thought_level"));
  const busy = $derived(manager?.settings.busy || !!manager?.settings.pending);
</script>

{#if manager && draft}
  <h2>Conversations</h2>
  <Field.Group class="community-settings__option-group">
    <StoreWorkField
      label="Automatically mention agents"
      id={`${id}-mention`}
      description="Address selected agents in thread replies. Existing conversation preferences are retained."
    >
      <Switch
        id={`${id}-mention`}
        bind:checked={draft.automatically_mention}
        disabled={busy}
      />
    </StoreWorkField>
  </Field.Group>
  <h2>Preferences</h2>
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
  <h2>Agent defaults</h2>
  <p>
    Runtime preferences and environment settings inherited by new local agents.
    Agent overrides take priority.
  </p>
  <Field.Group class="community-settings__option-group">
    <StoreWorkField label="Default harness" id={`${id}-harness`}>
      <select id={`${id}-harness`} bind:value={draft.runner} disabled={busy}>
        {#each manager.value?.runners ?? [] as runner (runner.id)}<option
            value={runner.id}>{runner.label}</option
          >{/each}
      </select>
    </StoreWorkField>
    {#if draft.runner === "local"}
      <StoreWorkField label="Provider" id={`${id}-provider`}>
        <select
          id={`${id}-provider`}
          bind:value={draft.options.provider}
          disabled={busy}
        >
          <option value="local">This machine</option><option value="paired"
            >Paired compute</option
          ><option value="openai">OpenAI compatible endpoint</option>
        </select>
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
    <StoreWorkField label="Model" id={`${id}-model`}>
      <select
        id={`${id}-model`}
        value={draft.preferences.model ?? ""}
        disabled={busy}
        onchange={(event) => {
          draft!.preferences.model = event.currentTarget.value || null;
        }}
      >
        <option value="">Harness default</option>
        {#each models as model (model.id)}<option value={model.id}
            >{model.name}</option
          >{/each}
        {#if draft.preferences.model && !models.some((m) => m.id === draft!.preferences.model)}<option
            value={draft.preferences.model}
            >{draft.preferences.model} (saved preference)</option
          >{/if}
      </select>
    </StoreWorkField>
    {#if thinking.length}<StoreWorkField label="Effort" id={`${id}-effort`}>
        <select
          id={`${id}-effort`}
          value={draft.preferences.effort ?? ""}
          disabled={busy}
          onchange={(event) => {
            draft!.preferences.effort = event.currentTarget.value || null;
          }}
        >
          <option value="">Harness default</option
          >{#each thinking as option (option.id)}<option value={option.id}
              >{option.name}</option
            >{/each}
        </select>
      </StoreWorkField>{/if}
    {#if modes.length}<StoreWorkField label="Execution mode" id={`${id}-mode`}
        ><select
          id={`${id}-mode`}
          value={draft.preferences.mode ?? ""}
          disabled={busy}
          onchange={(event) => {
            draft!.preferences.mode = event.currentTarget.value || null;
          }}
          ><option value="">Harness default</option
          >{#each modes as mode (mode.id)}<option value={mode.id}
              >{mode.name}</option
            >{/each}</select
        ></StoreWorkField
      >{/if}
    <StoreSettingsDisclosure>
      <h3>Environment variables</h3>
      <StoreAgentEnvironmentEditor
        keychainSupported={manager.settings.value?.keychain_supported ?? false}
        bind:value={draft.options.environment}
        bind:secrets
        disabled={busy}
      />
    </StoreSettingsDisclosure>
    <div class="store-settings__action-row">
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
  </Field.Group>
{/if}
