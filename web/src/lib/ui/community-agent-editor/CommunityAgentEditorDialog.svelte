<script lang="ts">
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import CommunityAvatarPicker from "../community-avatar-picker/CommunityAvatarPicker.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import { avatarPreview } from "../community-avatar-picker/legacy-avatar.js";
  import StoreAgentAdvanced from "../store-agent-execution/StoreAgentAdvanced.svelte";
  import type { AgentSetup } from "../../controller/agent-execution-types.js";
  import type { Snippet } from "svelte";
  import "./agent-editor.css";
  let {
    open = $bindable(false),
    draft = $bindable(),
    secrets = $bindable({}),
    keychainSupported = false,
    editing = false,
    conflict = false,
    latest,
    onUseLatest,
    pending = false,
    saveUnavailable = "",
    configuration,
    onSave,
  }: {
    open?: boolean;
    draft: AgentSetup;
    secrets?: Record<string, string>;
    keychainSupported?: boolean;
    editing?: boolean;
    conflict?: boolean;
    latest?: AgentSetup;
    onUseLatest?: () => void;
    pending?: boolean;
    saveUnavailable?: string;
    configuration: Snippet;
    onSave: (event: SubmitEvent) => Promise<void>;
  } = $props();
  const editorId = $props.id();
  const validation = $derived(
    [...(draft.description ?? "")].length > 280
      ? "Keep the description within 280 characters."
      : new TextEncoder().encode(draft.instructions).length > 32768
        ? "Keep instructions within 32 KiB."
        : "",
  );
</script>

<Dialog.Root bind:open>
  <DialogFrame
    title={editing ? "Edit agent" : "Add agent"}
    description="Manage the agent presentation, saved instructions, and runtime defaults."
    descriptionHidden
    size="large"
    surface="inset"
    class="agent-editor"
    busy={pending}
    closeLabel="Close agent editor"
    onInteractOutside={(event) => {
      if (pending) event.preventDefault();
    }}
    onEscapeKeydown={(event) => {
      if (pending) event.preventDefault();
    }}
  >
    {#snippet sidebar()}
      <div class="agent-editor__sidebar">
        <CommunityAvatarPicker
          bind:value={
            () => avatarPreview(draft.avatar), (value) => (draft.avatar = value)
          }
          label="Choose agent image"
          disabled={pending}
          size="medium"
          fallbackName={draft.name || "Agent"}
        />
      </div>
    {/snippet}
    <form
      id={`${editorId}-form`}
      class="agent-editor__form"
      aria-label="Agent configuration"
      onsubmit={onSave}
    >
      <Field.Group>
        <Field.Field>
          <Field.Label for={`${editorId}-persona-name`}>Agent name</Field.Label>
          <Input
            id={`${editorId}-persona-name`}
            bind:value={draft.name}
            disabled={pending}
            maxlength={128}
            required
          />
        </Field.Field>
        <Field.Field>
          <Field.Label>Description <span>Optional</span></Field.Label>
          <CommunityChatComposer
            mode="form"
            bind:value={draft.description}
            ariaLabel="Agent description"
            placeholder="What this agent does, in a sentence"
            disabled={pending}
          />
        </Field.Field>
        <Field.Field>
          <Field.Label>Agent instructions</Field.Label>
          <CommunityChatComposer
            mode="form"
            bind:value={draft.instructions}
            ariaLabel="Agent instructions"
            placeholder="Describe how this agent should work."
            disabled={pending}
          />
          <Field.Description
            >Used with the first prompt of future sessions. Existing sessions
            retain their original instructions.</Field.Description
          >
        </Field.Field>
        <div class="agent-editor__runtime-editor">
          <h3>AI configuration</h3>
          {@render configuration()}
        </div>
        <StoreAgentAdvanced
          bind:draft
          bind:secrets
          {pending}
          {keychainSupported}
        />
        {#if saveUnavailable}<p role="status">{saveUnavailable}</p>{/if}
        {#if conflict}
          <Field.Field>
            <Field.Error
              >Agent configurations changed while this form was open. Your draft
              is retained.</Field.Error
            >
            {#if latest}
              <details>
                <summary>Latest saved configuration</summary>
                <dl class="agent-editor__runtime-defaults">
                  <div>
                    <dt>Name</dt>
                    <dd>{latest.name}</dd>
                  </div>
                  <div>
                    <dt>Description</dt>
                    <dd>{latest.description || "None"}</dd>
                  </div>
                  <div>
                    <dt>Instructions</dt>
                    <dd>{latest.instructions || "None"}</dd>
                  </div>
                  <div>
                    <dt>Harness</dt>
                    <dd>{latest.runner}</dd>
                  </div>
                  {#each Object.entries(latest.preferences) as [name, value] (name)}
                    <div>
                      <dt>{name}</dt>
                      <dd>{value || "Harness default"}</dd>
                    </div>
                  {/each}
                </dl>
              </details>
            {/if}
            <Button
              type="button"
              variant="outline"
              disabled={pending}
              onclick={onUseLatest}>Keep draft and use latest version</Button
            >
          </Field.Field>
        {/if}
        {#if validation}<Field.Error>{validation}</Field.Error>{/if}
      </Field.Group>
    </form>
    {#snippet footer()}
      <Button
        type="button"
        variant="outline"
        disabled={pending}
        onclick={() => (open = false)}>Cancel</Button
      >
      <Button
        type="submit"
        form={`${editorId}-form`}
        disabled={pending ||
          conflict ||
          !!saveUnavailable ||
          !draft.name.trim() ||
          !!validation}>{pending ? "Saving..." : "Save agent"}</Button
      >
    {/snippet}
  </DialogFrame>
</Dialog.Root>
