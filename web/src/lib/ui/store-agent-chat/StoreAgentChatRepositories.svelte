<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import XIcon from "@lucide/svelte/icons/x";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import type { StoreController } from "../../controller/contracts.js";
  let { controller }: { controller: StoreController } = $props();
  const manager = $derived(controller.execution!);
  const chat = $derived(manager.chat);
  const disabled = $derived(chat.busy || !!chat.pending);
  const repositories = $derived(
    controller.state.repositories.filter(
      (r) =>
        !chat.draft?.allowedRepositories ||
        chat.draft.allowedRepositories.includes(r.id),
    ),
  );
  const available = $derived(
    repositories.filter(
      (r) => !chat.draft?.attachments.some((a) => a.repository === r.id),
    ),
  );
</script>

<div class="agent-chat-selections" aria-label="Chat context">
  <p class="agent-chat-origin">
    {chat.draft?.origin.scope === "personal"
      ? "General chat"
      : chat.draft?.origin.scope.startsWith("project:")
        ? "Project context"
        : chat.draft?.origin.scope.startsWith("thread:")
          ? "Discussion context"
          : "Repository context"}
  </p>
  <Field.Group class="community-settings__option-group">
    {#if !manager.localManagement() && chat.draft?.origin.scope === "personal"}
      <StoreWorkField label="Conversation home" bordered={false}>
        <StoreRepositoryPicker
          variant="ghost"
          label="Conversation home"
          controlled
          repositories={controller.state.repositories.filter(
            (r) => r.profile === "native" && r.permission !== "read",
          )}
          value={chat.draft.origin.home ?? ""}
          disabled={disabled || chat.sessions.length > 0}
          emptyLabel="Choose WorkStore home"
          onChange={(value) => {
            if (chat.draft && !disabled) chat.draft.origin.home = value || null;
          }}
        />
      </StoreWorkField>
      <p>
        Choose an existing project or discussion repository to store this
        conversation.
      </p>
    {/if}
    {#each chat.draft?.attachments ?? [] as attachment (attachment.repository)}
      {@const repository = repositories.find(
        (r) => r.id === attachment.repository,
      )}
      {@const name = repository?.name ?? attachment.repository}
      {@const targets = manager.targets.filter(
        (t) => t.repository === attachment.repository,
      )}
      <StoreWorkField label="Repository" bordered={false}>
        <div class="agent-chat-repository-control">
          <StoreRepositoryPicker
            variant="ghost"
            label={`Repository ${name}`}
            controlled
            repositories={repositories.filter(
              (r) =>
                r.id === attachment.repository ||
                available.some((a) => a.id === r.id),
            )}
            value={attachment.repository}
            {disabled}
            onChange={(value) =>
              chat.selectRepository(attachment.repository, value)}
          />
          <Button
            variant="ghost"
            size="icon-sm"
            {disabled}
            aria-label={`Remove ${name}`}
            onclick={() => chat.removeAttachment(attachment.repository)}
          >
            <XIcon aria-hidden="true" />
          </Button>
        </div>
      </StoreWorkField>
      {#if targets.length > 1 || attachment.target_id}
        <StoreWorkField label="Workspace" bordered={false}>
          <StoreRepositoryPicker
            variant="ghost"
            label={`Workspace for ${name}`}
            controlled
            searchLabel="Search workspaces"
            groupLabel="Workspaces"
            emptyLabel="Choose workspace"
            emptyResult="No matching workspaces."
            repositories={targets.map((t) => ({
              id: t.id,
              name: t.name,
              description: t.folder,
            }))}
            value={attachment.target_id ?? ""}
            disabled={disabled || !!manager.targetError}
            onChange={(value) =>
              chat.selectWorkspace(attachment.repository, value)}
          />
        </StoreWorkField>
      {/if}
    {/each}
    {#if available.length && (chat.draft?.attachments.length ?? 0) < 8}
      <StoreWorkField
        label={chat.draft?.attachments.length ? "Add repository" : "Repository"}
        bordered={false}
      >
        <StoreRepositoryPicker
          variant="ghost"
          label="Attach repository"
          repositories={available}
          value=""
          controlled
          {disabled}
          emptyLabel="Choose repository"
          onChange={(value) => chat.attach(value)}
        />
      </StoreWorkField>
    {/if}
  </Field.Group>
</div>

<style>
  .agent-chat-selections {
    display: grid;
    width: 100%;
    gap: 0.75rem;
  }
  .agent-chat-origin {
    margin: 0;
    color: var(--muted-foreground);
    font-size: var(--community-text-secondary);
  }
  .agent-chat-repository-control {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    min-width: 0;
  }
</style>
