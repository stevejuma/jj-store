<script lang="ts">
  import type { Snippet } from "svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import CommunityMarkdownContent from "../community-markdown-content/CommunityMarkdownContent.svelte";

  let {
    value = $bindable(""),
    editing = $bindable(false),
    disabled = false,
    label = "PR description",
    editLabel = "Edit description",
    showEditButton = true,
    additionalActions,
  }: {
    value?: string;
    editing?: boolean;
    disabled?: boolean;
    label?: string;
    editLabel?: string;
    showEditButton?: boolean;
    additionalActions?: Snippet;
  } = $props();
</script>

<Field.Field>
  {#if editing || showEditButton}
    <div class="forge-panel__toolbar">
      <Button
        variant="ghost"
        size="sm"
        disabled={disabled && !editing}
        onclick={() => (editing = !editing)}
        >{editing ? "Preview description" : editLabel}</Button
      >
    </div>
  {/if}
  {#if editing}
    <CommunityChatComposer
      mode="form"
      ariaLabel={label}
      bind:value
      {disabled}
      {additionalActions}
      placeholder="Add context for reviewers"
    />
  {:else if value.trim()}
    <CommunityMarkdownContent content={value} fullWidth />
  {:else}
    <p>No description yet.</p>
  {/if}
</Field.Field>
