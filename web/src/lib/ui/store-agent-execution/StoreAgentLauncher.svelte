<script lang="ts">
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import * as Empty from "@lapismd/design-core/shadcn/empty";
  import MessageIcon from "@lucide/svelte/icons/messages-square";
  import StoreAgentPicker from "./StoreAgentPicker.svelte";
  import StoreAgentRevisionPicker from "./StoreAgentRevisionPicker.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import CommunityAgentActivityFeed from "../community-agent-activity/CommunityAgentActivityFeed.svelte";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  let { manager }: { manager: AgentExecution } = $props();
  const formId = $props.id();
  const draft = $derived(manager.launchDraft);
  const target = $derived(manager.targets.find((t) => t.id === draft.targetId));
  const saved = $derived(
    manager.value?.setups.find((s) => s.id === manager.setupId),
  );
  const draftKey = $derived(`new:${manager.setupId || "unassigned"}`);
  $effect(() => {
    manager.drafts[draftKey] ??= "";
  });
  $effect(() => {
    if (!draft.targetId && manager.targets.length)
      draft.targetId = manager.targets[0].id;
  });
  $effect(() => {
    const selected = draft.targetId,
      contextual = manager.context.commit;
    untrack(() => {
      const target = manager.targets.find((t) => t.id === selected);
      if (target && !draft.commit) draft.commit = contextual || target.commit;
    });
  });
  $effect(() => {
    if (saved) draft.runner = saved.runner;
  });
  const valid = $derived(/^(?:[0-9a-f]{40}|[0-9a-f]{128})$/.test(draft.commit));
</script>

<div class="agent-conversation">
  <CommunityAgentActivityFeed conversationKey={draftKey} empty>
    {#snippet messageComposer()}
      <form
        id={formId}
        aria-label="Start agent session"
        onsubmit={(event) => {
          event.preventDefault();
          if (target && valid)
            void manager.launch(
              target,
              draft.runner,
              manager.setupId,
              draft.commit,
            );
        }}
      >
        <CommunityChatComposer
          mode="form"
          ariaLabel="Message to agent"
          placeholder={`Message ${saved?.name ?? "the agent"}…`}
          bind:value={
            () => manager.drafts[draftKey] ?? "",
            (value) => {
              manager.drafts[draftKey] = value;
            }
          }
          disabled={manager.busy || !!manager.pending}
        />
      </form>
    {/snippet}
    <Empty.Root class="agent-draft-empty"
      ><Empty.Header
        ><Empty.Media variant="icon"
          ><MessageIcon aria-hidden="true" /></Empty.Media
        ><Empty.Title>Start a chat with {saved?.name ?? "an agent"}</Empty.Title
        ><Empty.Description
          >Choose an agent, repository and starting revision, then connect.
          These selections are fixed for the session. Your message stays in the
          draft until you Send.</Empty.Description
        ></Empty.Header
      ><Empty.Content>
        <div class="agent-draft-selections">
          <Field.Group class="community-settings__option-group">
            <StoreWorkField label="Agent" bordered={false}
              ><StoreAgentPicker {manager} /></StoreWorkField
            >
            <StoreWorkField label="Repository" bordered={false}
              ><StoreRepositoryPicker
                variant="ghost"
                label="Repository workspace"
                repositories={manager.targets.map((item) => ({
                  id: item.id,
                  name: item.name,
                  description: item.folder,
                }))}
                value={draft.targetId}
                controlled
                disabled={manager.busy || !!manager.pending}
                onChange={(value) => {
                  draft.targetId = value;
                  draft.commit =
                    manager.targets.find((t) => t.id === value)?.commit ?? "";
                }}
              /></StoreWorkField
            >
            <StoreWorkField label="Starting revision" bordered={false}
              ><StoreAgentRevisionPicker {manager} {target} /></StoreWorkField
            ></Field.Group
          >
          <Button
            type="submit"
            form={formId}
            disabled={manager.busy ||
              !!manager.pending ||
              !target ||
              !valid ||
              !manager.value?.runners.find((r) => r.id === draft.runner)
                ?.available ||
              manager.value?.runtime !== "ready"}>Connect agent</Button
          >
        </div>
      </Empty.Content></Empty.Root
    >
    {#if !manager.targets.length}<p role="status">
        Register a local JJ folder or link a repository workspace to start a
        chat.
      </p>{/if}
  </CommunityAgentActivityFeed>
</div>
