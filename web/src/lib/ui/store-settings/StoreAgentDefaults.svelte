<script lang="ts">
  import StoreSettingsSkeleton from "./StoreSettingsSkeleton.svelte";
  import StoreSettingsFooter from "./StoreSettingsFooter.svelte";
  import StoreSettingsCard from "./StoreSettingsCard.svelte";
  import SectionIcon from "@lucide/svelte/icons/bot";
  import StoreAgentChatGptAccounts from "./StoreAgentChatGptAccounts.svelte";
  import StoreConversationMetadataSettings from "./StoreConversationMetadataSettings.svelte";
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
  <StoreSettingsCard
    id={`${id}-preferences`}
    title="Preferences"
    Icon={SectionIcon}
  >
    <Field.Group class="store-settings__editable-rows">
      <StoreAgentMentionPreference settings={manager.settings} />
      <StoreWorkField
        class="store-settings__option-row"
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
      <StoreSettingsDisclosure
        label="Conversation metadata"
        description="Generate a title, summary, keywords and goals after the first completed exchange. Later updates are refreshed manually."
        inset={false}
      >
        <StoreConversationMetadataSettings {manager} embedded />
      </StoreSettingsDisclosure>
    </Field.Group>
  </StoreSettingsCard>
{:else if manager && !manager.settings.error}
  <StoreSettingsSkeleton
    heading={false}
    label="Loading agent preferences"
    cards={[{ rows: ["toggle", "toggle", "disclosure"] }]}
  />
{/if}
{@render runtimeSection?.()}
{#if manager}<StoreAgentChatGptAccounts accounts={manager.chatgpt} />{/if}
{#if manager && draft}
  <StoreSettingsCard
    id={`${id}-defaults`}
    title="Agent defaults"
    Icon={SectionIcon}
    description="Runtime preferences and environment settings inherited by new local agents. Agent overrides take priority."
  >
    <StoreAgentConfiguration
      {manager}
      bind:draft
      {secrets}
      pending={busy}
      scope="agent-defaults"
    >
      <Field.Group class="store-settings__editable-rows">
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
      <StoreSettingsFooter>
        {#snippet description()}<p>
            Saved changes take effect on the next start.
          </p>{/snippet}
        {#if manager.settings.pending}<Button
            variant="outline"
            onclick={() => manager!.settings.retry()}
            >Check retained save</Button
          >{/if}
        {#if manager.settings.value && manager.settings.value.revision !== draft.revision && !manager.settings.pending}
          <Button
            variant="outline"
            onclick={() => {
              draft!.revision = manager!.settings.value!.revision;
            }}>Keep draft and use latest version</Button
          >
        {/if}
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
      </StoreSettingsFooter>
    </StoreAgentConfiguration>
    <StoreNotice
      noticeKey="agent-defaults-command"
      title="Agent defaults need attention"
      message={manager.settings.error}
    />
  </StoreSettingsCard>
{:else if manager && !manager.settings.error}
  <StoreSettingsSkeleton
    heading={false}
    label="Loading agent defaults"
    cards={[{ rows: ["input", "input", "input", "disclosure"], actions: 1 }]}
  />
{/if}
