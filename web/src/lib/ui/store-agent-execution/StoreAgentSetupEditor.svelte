<script lang="ts">
  // @community-copy CommunityAgentEditorDialog.svelte (Apache-2.0), device setups.
  import { untrack } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Tabs from "@lapismd/design-core/shadcn/tabs";
  import type { AgentExecution } from "../../controller/agent-execution.svelte.js";
  import type { AgentSetup } from "../../controller/agent-execution-types.js";
  let {
    manager,
    setup,
    open = $bindable(false),
  }: { manager: AgentExecution; setup?: AgentSetup; open?: boolean } = $props();
  const formId = $props.id();
  const draftKey = untrack(() => setup?.id ?? "new");
  const draft = untrack(() => {
    if (!manager.setupDrafts[draftKey])
      manager.setupDrafts[draftKey] = {
        id: setup?.id ?? crypto.randomUUID().replaceAll("-", ""),
        name: setup?.name ?? "",
        avatar: setup?.avatar ?? "🤖",
        instructions: setup?.instructions ?? "",
        runner: setup?.runner ?? "codex",
        preferences: {
          ...(setup?.preferences ?? { model: "", effort: "", mode: "" }),
        },
      };
    return manager.setupDrafts[draftKey];
  });
  async function save(event: SubmitEvent) {
    event.preventDefault();
    await manager.saveSetup({
      id: draft.id,
      name: draft.name,
      avatar: draft.avatar,
      instructions: draft.instructions,
      runner: draft.runner,
      preferences: {
        model: draft.preferences.model || null,
        effort: draft.preferences.effort || null,
        mode: draft.preferences.mode || null,
      },
    });
    if (!manager.pending && !manager.error) {
      delete manager.setupDrafts[draftKey];
      open = false;
    }
  }
</script>

<Dialog.Root bind:open
  ><Dialog.Content class="agent-setup-editor"
    ><Dialog.Header
      ><Dialog.Title
        >{setup ? "Edit saved setup" : "New saved setup"}</Dialog.Title
      ><Dialog.Description
        >Choose how this agent appears and the preferences used when opening a
        session.</Dialog.Description
      ></Dialog.Header
    >
    <form
      class="agent-execution-form"
      onsubmit={save}
      aria-label="Saved agent setup"
    >
      <Tabs.Root value="identity"
        ><Tabs.List
          ><Tabs.Trigger value="identity">Identity</Tabs.Trigger><Tabs.Trigger
            value="instructions">Instructions</Tabs.Trigger
          ><Tabs.Trigger value="runtime">Runtime</Tabs.Trigger></Tabs.List
        >
        <Tabs.Content value="identity"
          ><div class="agent-execution-form">
            <label for={`${formId}-name`}>Name</label><Input
              id={`${formId}-name`}
              bind:value={draft.name}
              required
              maxlength={128}
            /><label for={`${formId}-avatar`}>Avatar emoji</label><Input
              id={`${formId}-avatar`}
              bind:value={draft.avatar}
              required
              maxlength={128}
            />
          </div></Tabs.Content
        >
        <Tabs.Content value="instructions"
          ><label for={`${formId}-instructions`}>Instructions</label><Textarea
            id={`${formId}-instructions`}
            bind:value={draft.instructions}
            rows={8}
            maxlength={32768}
          />
          <p>
            Instructions are included with the first prompt. Credentials belong
            in the runner’s authentication settings.
          </p></Tabs.Content
        >
        <Tabs.Content value="runtime"
          ><div class="agent-execution-form">
            <label for={`${formId}-runner`}>Runner</label><select
              id={`${formId}-runner`}
              bind:value={draft.runner}
              ><option value="codex">Codex</option><option value="cursor"
                >Cursor</option
              ></select
            ><label for={`${formId}-model`}>Model preference</label><Input
              id={`${formId}-model`}
              bind:value={draft.preferences.model}
              placeholder="Choose after connection"
            /><label for={`${formId}-effort`}>Effort preference</label><Input
              id={`${formId}-effort`}
              bind:value={draft.preferences.effort}
              placeholder="Runner default"
            /><label for={`${formId}-mode`}>Execution mode preference</label
            ><Input
              id={`${formId}-mode`}
              bind:value={draft.preferences.mode}
              placeholder="Choose after connection"
            />
            <p>
              Available choices and applied preferences are shown after the
              runner connects.
            </p>
          </div></Tabs.Content
        >
      </Tabs.Root>
      <Dialog.Footer
        ><Button variant="outline" type="button" onclick={() => (open = false)}
          >Cancel</Button
        ><Button
          type="submit"
          disabled={manager.busy || !!manager.pending || !draft.name.trim()}
          >Save setup</Button
        ></Dialog.Footer
      >
    </form>
  </Dialog.Content></Dialog.Root
>
