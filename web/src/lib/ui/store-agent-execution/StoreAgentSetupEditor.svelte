<script lang="ts">
  // Adapted CommunityAgentEditorDialog: local saved agent, no relay/provisioning state.
  import { untrack } from "svelte";
  import CommunityAgentEditorDialog from "../community-agent-editor/CommunityAgentEditorDialog.svelte";
  import type { EditorModelRow } from "../../community/agent-editor-model-rows.js";
  import { runtimeChoices } from "./presentation.js";
  import { waitForRuntime } from "./runtime-preparation.js";
  import { harnessCatalog } from "./harness-catalog.js";
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
  const catalogAgent = $derived(
    manager.value?.setups.find(
      (saved) => saved.id === draft.id && saved.runner === draft.runner,
    )?.id,
  );
  const catalogKey = $derived(manager.catalogs.key(draft.runner, catalogAgent));
  const catalog = $derived(
    harnessCatalog(
      manager.value?.sessions ?? [],
      draft.runner,
      manager.catalogs.values[catalogKey]?.facts,
    ),
  );
  $effect(() => {
    const runner = draft.runner;
    const available =
      manager.value?.runtime === "ready" &&
      manager.value.runners.some((r) => r.id === runner && r.available);
    if (open && available)
      untrack(() => {
        if (catalogAgent || !catalog.session)
          void manager.catalogs.load(runner, false, catalogAgent);
      });
  });
  const facts = $derived(catalog.facts);
  const hasCatalog = $derived(
    runtimeChoices(facts, "model").length ||
      runtimeChoices(facts, "thought_level").length,
  );
  const modelRows = $derived(
    (manager.value?.runners ?? []).flatMap((runner) => {
      const { facts } = harnessCatalog(
        manager.value?.sessions ?? [],
        runner.id,
        manager.catalogs.values[
          runner.id === draft.runner ? catalogKey : runner.id
        ]?.facts,
      );
      const options = runtimeChoices(facts, "model");
      return [
        {
          runtimeAgent: runner.id,
          harnessLabel: runner.label,
          model: "",
          name: "Harness default",
          keywords: [runner.id],
        },
        ...options.map((option) => ({
          runtimeAgent: runner.id,
          harnessLabel: runner.label,
          model: option.id,
          name: option.name,
          keywords: [option.id],
        })),
        ...(runner.id === draft.runner &&
        draft.preferences.model &&
        !options.some((o) => o.id === draft.preferences.model)
          ? [
              {
                runtimeAgent: runner.id,
                harnessLabel: runner.label,
                model: draft.preferences.model,
                name: `${draft.preferences.model} (saved preference)`,
                disabled: true,
                keywords: [draft.preferences.model],
              },
            ]
          : []),
      ];
    }),
  );
  function selectModel(row: EditorModelRow) {
    if (row.runtimeAgent !== draft.runner)
      draft.preferences = { model: null, effort: null, mode: null };
    draft.runner = row.runtimeAgent;
    draft.preferences.model = row.model || null;
  }
  const thinking = $derived(
    runtimeChoices(facts, "thought_level").map((o) => ({
      value: o.id,
      name: o.name,
    })),
  );
  const modes = $derived(
    runtimeChoices(facts, "mode").map((o) => ({ value: o.id, name: o.name })),
  );
  let secretDrafts = $state(
    untrack(() => (manager.settings.setupSecretDrafts[draftKey] ??= {})),
  );
  let savingSecrets = $state(false);
  async function save(event: SubmitEvent) {
    event.preventDefault();
    if (savingSecrets || manager.busy) return;
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
  runtimeNotice={session
    ? "Save changes the agent defaults for future chats. Model, thinking and execution mode also apply to this connected session when using its current harness. Its captured instructions and workspace stay unchanged."
    : catalog.remembered && hasCatalog
      ? `Showing choices from this harness's last session. Availability is checked again when a new session connects. Saved preferences remain until you change them. ${manager.catalogs.errors[catalogKey] ?? ""}`
      : manager.catalogs.errors[catalogKey]
        ? `No model or thinking choices have been advertised for this harness. Saved preferences remain until you change them. ${manager.catalogs.errors[catalogKey]}`
        : manager.catalogs.loading[catalogKey] && !hasCatalog
          ? "Loading models and thinking choices from the installed harness…"
          : !hasCatalog
            ? "No model or thinking choices have been advertised for this harness. Saved preferences remain until you change them."
            : catalog.session
              ? "Model and thinking choices come from the connected harness. Saved preferences remain until you change them."
              : "Model and thinking choices come from the installed harness. Availability is checked again when a chat connects."}
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
  rows={modelRows}
  {thinking}
  {modes}
  onModel={selectModel}
  onSave={save}
  modelsLoading={manager.catalogs.loading[catalogKey]}
  onRetryModels={manager.catalogs.errors[catalogKey]
    ? () => void manager.catalogs.load(draft.runner, true, catalogAgent)
    : undefined}
/>
