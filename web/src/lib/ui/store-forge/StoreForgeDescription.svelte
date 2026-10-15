<script lang="ts">
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
  }: {
    value?: string;
    editing?: boolean;
    disabled?: boolean;
    label?: string;
    editLabel?: string;
  } = $props();
</script>

<Field.Field>
  <div class="forge-panel__toolbar">
    <Field.Title>Description</Field.Title>
    <Button
      variant="ghost"
      size="sm"
      disabled={disabled && !editing}
      onclick={() => (editing = !editing)}
      >{editing ? "Preview description" : editLabel}</Button
    >
  </div>
  {#if editing}
    <CommunityChatComposer
      mode="form"
      ariaLabel={label}
      bind:value
      {disabled}
      placeholder="Add context for reviewers"
    />
  {:else if value.trim()}
    <CommunityMarkdownContent content={value} fullWidth />
  {:else}
    <p>No description yet.</p>
  {/if}
</Field.Field>
