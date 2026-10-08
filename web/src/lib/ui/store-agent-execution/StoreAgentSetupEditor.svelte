<script lang="ts">
  // Adapted CommunityAgentEditorDialog: local saved agent, no relay/provisioning state.
  import { untrack } from "svelte";
  import CommunityAgentEditorDialog from "../community-agent-editor/CommunityAgentEditorDialog.svelte";
  import { waitForRuntime } from "./runtime-preparation.js";
  import StoreAgentConfiguration from "./StoreAgentConfiguration.svelte";
  import { sameLaunchOptions } from "../../controller/agent-configuration.js";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type {
    AgentSetup,
    AgentSession,
  } from "../../controller/agent-execution-types.js";
  import "../community-agent-editor/agent-editor.css";
  let {
    manager,
    setup,
    session,
    open = $bindable(false),
  }: {
    manager: AgentExecution;
    setup?: AgentSetup;
    session?: AgentSession;
    open?: boolean;
  } = $props();
  const draftKey = untrack(() =>
    session
      ? `session:${session.id}:agent`
      : setup
        ? `agent:${setup.id}`
        : "new",
  );
  let draft = $state(
    untrack(() => {
      if (!manager.setupDrafts[draftKey]) {
        manager.setupDraftRevisions[draftKey] = manager.value?.revision ?? 0;
        manager.setupDrafts[draftKey] = {
          id: setup?.id ?? crypto.randomUUID().replaceAll("-", ""),
          name: setup?.name ?? session?.title ?? "",
          avatar: setup?.avatar ?? session?.avatar ?? "",
          description: setup?.description ?? "",
          instructions: setup?.instructions ?? "",
          runner:
            setup?.runner ??
            session?.runner ??
            manager.settings.value?.runner ??
            "codex",
          options: structuredClone(
            $state.snapshot(
              setup?.options ?? {
                inherit_defaults: !setup && !session,
                provider: null,
                endpoint: null,
                environment: {},
              },
            ),
          ),
          preferences: {
            ...(setup?.preferences ??
              session?.preferences ?? {
                model: null,
                effort: null,
                mode: null,
              }),
          },
        };
      }
      return manager.setupDrafts[draftKey];
    }),
  );
  const initialConfiguration = untrack(() =>
    structuredClone($state.snapshot(draft)),
  );
  let secretDrafts = $state(
    untrack(() => (manager.settings.setupSecretDrafts[draftKey] ??= {})),
  );
  let savingSecrets = $state(false);
  async function save(event: SubmitEvent) {
    event.preventDefault();
    if (savingSecrets || manager.busy) return;
    const launchPending =
      manager.value?.live_agents?.find((live) => live.agent_id === draft.id)
        ?.restart_required ?? false;
    savingSecrets = true;
    try {
      await manager.settings.saveSecrets(secretDrafts);
    } catch (cause) {
      manager.error = cause instanceof Error ? cause.message : String(cause);
      return;
    } finally {
      savingSecrets = false;
    }
    const acknowledgedRevision = await manager.saveSetup(
      {
        ...draft,
        preferences: {
          model: draft.preferences.model || null,
          effort: draft.preferences.effort || null,
          mode: draft.preferences.mode || null,
        },
      },
      undefined,
      manager.setupDraftRevisions[draftKey],
    );
    if (manager.pending || manager.error) return;
    if (acknowledgedRevision !== undefined)
      manager.setupDraftRevisions[draftKey] = acknowledgedRevision;
    const current =
      session && manager.value?.sessions.find((s) => s.id === session.id);
    if (
      current?.connected &&
      ["idle", "running"].includes(current.state) &&
      !launchPending &&
      sameLaunchOptions(draft, initialConfiguration) &&
      draft.runner === current.runner &&
      draft.preferences.mode
    ) {
      const request = await manager.commandSession(current.id, {
        action: "configure",
        preferences: { ...draft.preferences },
      });
      if (
        !(await waitForRuntime(
          manager,
          current.id,
          request,
          { ...draft.preferences },
          () => open,
        ))
      )
        return;
    }
    if (!manager.pending && !manager.error) {
      delete manager.setupDrafts[draftKey];
      delete manager.settings.setupSecretDrafts[draftKey];
      delete manager.setupDraftRevisions[draftKey];
      open = false;
    }
  }
</script>

<CommunityAgentEditorDialog
  keychainSupported={manager.settings.value?.keychain_supported ?? false}
  bind:secrets={secretDrafts}
  saveUnavailable={manager.value?.runtime === "preparing"
    ? "Preparing agent configuration. Save becomes available when preparation completes."
    : manager.value?.runtime === "unavailable"
      ? "Agent configuration is unavailable on this device."
      : ""}
  conflict={manager.setupConflict}
  latest={manager.value?.setups.find((s) => s.id === draft.id)}
  onUseLatest={() => {
    manager.setupDraftRevisions[draftKey] = manager.value?.revision ?? 0;
    manager.setupConflict = false;
    manager.error = "";
  }}
  bind:open
  bind:draft
  editing={!!setup}
  pending={savingSecrets || manager.busy || !!manager.pending}
  onSave={save}
>
  {#snippet configuration()}
    {#if session}<p>
        Save changes the agent defaults for future chats. Provider and
        environment changes require an explicit restart.
      </p>{/if}
    <StoreAgentConfiguration
      {manager}
      bind:draft
      secrets={secretDrafts}
      pending={savingSecrets || manager.busy || !!manager.pending}
      active={open}
      scope={`editor:${draftKey}`}
    />
  {/snippet}
</CommunityAgentEditorDialog>
