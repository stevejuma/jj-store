<script lang="ts">
  import { Button } from "@lapismd/design-core/shadcn/button";
  import ReviewIcon from "@lucide/svelte/icons/git-compare-arrows";
  import TrashIcon from "@lucide/svelte/icons/trash-2";
  import StoreReadOnlyRow from "../store-settings/StoreReadOnlyRow.svelte";
  import CommunityAgentRuntimeFields from "../community-agent-profile/CommunityAgentRuntimeFields.svelte";
  import StoreAgentSetupEditor from "./StoreAgentSetupEditor.svelte";

  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSession } from "../../controller/agent-execution-types.js";
  import "../community-agent-profile/agent-profile.css";
  import "../community-channel-settings/CommunityChannelSettings.css";
  let {
    manager,
    session,
    onReview,
  }: {
    manager: AgentExecution;
    session: AgentSession;
    onReview?: (repository: string, commit: string, operation: string) => void;
  } = $props();
  const sectionId = $props.id();
  const agent = $derived(
    manager.value?.setups.find((setup) => setup.id === session.setup_id),
  );
  const preferences = $derived(agent?.preferences ?? session.preferences);
  const runner = $derived(agent?.runner ?? session.runner);
  const harness = $derived(
    manager.value?.runners.find((r) => r.id === runner)?.label ?? runner,
  );
  let editorOpen = $state(false);
  const connected = $derived(
    session.connected && ["idle", "running"].includes(session.state),
  );
  const blocked = $derived(manager.busy || !!manager.pending);
</script>

<div class="agent-runtime-settings lc-channel-settings">
  <section aria-labelledby={`${sectionId}-workspace`}>
    <h2 id={`${sectionId}-workspace`}>Workspace</h2>
    <dl class="agent-profile__runtime-fields" aria-label="Session workspace">
      <StoreReadOnlyRow
        label="Workspace"
        copyLabel="Workspace path"
        value={session.workspace}
        copyable
        stacked
      />
      <StoreReadOnlyRow
        label="Starting revision"
        value={session.base_commit}
        copyable
        stacked
        onOpen={onReview
          ? () =>
              onReview(
                session.repository,
                session.base_commit,
                session.operation,
              )
          : undefined}
        openLabel="Open starting revision"
      />
    </dl>
  </section>
  <section aria-labelledby={`${sectionId}-agent`}>
    <h2 id={`${sectionId}-agent`}>Agent settings</h2>
    <CommunityAgentRuntimeFields
      editable={manager.localManagement()}
      {harness}
      model={preferences.model || "Harness default"}
      thinking={preferences.effort || "Harness default"}
      mode={preferences.mode || "Choose execution mode"}
      disabled={blocked}
      harnessEditable={!!agent && manager.localManagement()}
      onEdit={() => (editorOpen = true)}
    />
  </section>
  <section aria-labelledby={`${sectionId}-actions`}>
    <h2 id={`${sectionId}-actions`}>Actions</h2>
    <div
      class="lc-channel-settings__management-zone"
      aria-label="Session workspace actions"
    >
      <Button
        type="button"
        variant="ghost"
        class="lc-channel-settings__row"
        disabled={!onReview}
        onclick={() =>
          onReview?.(
            session.repository,
            session.review_commit,
            session.operation,
          )}
      >
        <ReviewIcon data-icon="inline-start" aria-hidden="true" />Review changes
      </Button>
      {#if !connected && session.state !== "cleaned"}
        <Button
          type="button"
          variant="ghost"
          class="lc-channel-settings__row lc-channel-settings__destructive"
          disabled={blocked}
          onclick={() =>
            manager.commandSession(session.id, { action: "cleanup" })}
        >
          <TrashIcon data-icon="inline-start" aria-hidden="true" />Clean up
          workspace
        </Button>
      {/if}
    </div>
    {#if !connected && session.state !== "cleaned"}<p
        class="agent-execution-meta"
      >
        Cleanup retains a JJ bookmark and refuses ignored or untracked files.
      </p>{/if}
  </section>
</div>
{#if editorOpen}<StoreAgentSetupEditor
    {manager}
    {session}
    setup={agent}
    bind:open={editorOpen}
  />{/if}

<style>
  .agent-runtime-settings dl {
    margin: 0;
  }
</style>
