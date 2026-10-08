<script lang="ts">
  import StoreAgentMentionPreference from "./StoreAgentMentionPreference.svelte";
  import StoreSettingsDisclosure from "./StoreSettingsDisclosure.svelte";
  import { isMacDesktopWindow } from "../../desktop-chrome.js";
  import { untrack, type Snippet } from "svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreAgentEnvironmentEditor from "../store-agent-execution/StoreAgentEnvironmentEditor.svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSettingsValue } from "../../controller/agent-settings.svelte.js";
  import StoreAgentConfiguration from "../store-agent-execution/StoreAgentConfiguration.svelte";
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
      <StoreAgentConfiguration
        {manager}
        bind:draft
        {secrets}
        pending={busy}
        scope="agent-defaults"
      />
      <Field.Group class="community-settings__option-group">
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
