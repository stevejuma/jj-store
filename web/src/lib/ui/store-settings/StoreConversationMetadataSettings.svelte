<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import StoreAgentConfiguration from "../store-agent-execution/StoreAgentConfiguration.svelte";
  import StoreAgentEnvironmentEditor from "../store-agent-execution/StoreAgentEnvironmentEditor.svelte";
  import StoreSettingsDisclosure from "./StoreSettingsDisclosure.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  let { manager }: { manager: AgentExecution } = $props();
  let secrets = $state<Record<string, string>>(
    untrack(() =>
      structuredClone($state.snapshot(manager.settings.metadataSecretDrafts)),
    ),
  );
  $effect(() => {
    manager.settings.metadataSecretDrafts = structuredClone(
      $state.snapshot(secrets),
    );
  });
  let draft = $state<
    import("../../controller/agent-configuration.js").AgentConfiguration | null
  >(null);
  $effect(() => {
    const value = manager.settings.value;
    untrack(() => {
      if (value && !draft) {
        draft = manager.settings.metadataDraft
          ? structuredClone($state.snapshot(manager.settings.metadataDraft))
          : {
              runner: "local",
              preferences: {
                model: value.metadata?.model ?? null,
                effort: null,
                mode: null,
              },
              options: structuredClone(
                $state.snapshot(value.metadata?.options) ?? {
                  inherit_defaults: false,
                  provider: "local",
                  endpoint: null,
                  account: null,
                  environment: {},
                },
              ),
            };
      }
    });
  });
  $effect(() => {
    if (draft)
      manager.settings.metadataDraft = structuredClone($state.snapshot(draft));
  });
  const busy = $derived(manager.settings.busy || !!manager.settings.pending);
  async function save(clear = false) {
    const current = manager.settings.value;
    if (!current || !draft) return;
    const value = structuredClone($state.snapshot(current));
    value.metadata = clear
      ? null
      : {
          model: draft.preferences.model!,
          options: structuredClone($state.snapshot(draft.options)),
        };
    if (await manager.settings.save(value, secrets))
      manager.catalogs.invalidate();
  }
</script>

<section
  class="store-settings__section"
  aria-label="Conversation metadata settings"
>
  <header class="store-settings__section-heading">
    <div>
      <h2>Conversation metadata</h2>
      <p>
        Generate a title, summary, keywords and goals after the first completed
        exchange. Later updates are refreshed manually.
      </p>
    </div>
  </header>
  {#if draft}
    <StoreAgentConfiguration
      {manager}
      bind:draft
      {secrets}
      pending={busy}
      scope="conversation-metadata"
      codingControls={false}
      modelLabel="Metadata model"
      providerLabel="Metadata provider"
      retryLabel="Retry metadata models"
    >
      <Field.Group class="store-settings__editable-rows"
        ><StoreSettingsDisclosure label="Metadata provider environment"
          ><h3>Provider environment</h3>
          <StoreAgentEnvironmentEditor
            bind:value={draft.options.environment}
            bind:secrets
            keychainSupported={manager.settings.value?.keychain_supported ??
              false}
            disabled={busy}
          /></StoreSettingsDisclosure
        ></Field.Group
      >
      <div class="store-settings__action-row">
        <Button
          disabled={busy || !draft.preferences.model}
          onclick={() => save()}>Save metadata model</Button
        ><Button
          variant="ghost"
          disabled={busy || !manager.settings.value?.metadata}
          onclick={() => save(true)}>Disable generation</Button
        >
      </div>
    </StoreAgentConfiguration>
  {/if}
  <StoreNotice
    noticeKey="conversation-metadata-settings"
    title="Metadata settings need attention"
    message={manager.settings.error}
  />
</section>
