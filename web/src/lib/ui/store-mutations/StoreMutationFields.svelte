<script lang="ts">
  import * as Select from "@lapismd/design-core/shadcn/select";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Checkbox } from "@lapismd/design-core/shadcn/checkbox";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import "../editor-focus.css";
  import { untrack } from "svelte";
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import StoreMutationCommitPicker from "./StoreMutationCommitPicker.svelte";
  import StoreMutationCommitTarget from "./StoreMutationCommitTarget.svelte";
  import type { StoreMutationAction } from "./model.js";

  let {
    action,
    operationHeads,
    description = $bindable(),
    workspace = $bindable(),
    bookmark = $bindable(),
    target = $bindable(),
    base = $bindable(),
    retainBookmarks = $bindable(),
    restoreDescendants = $bindable(),
    selectedRevision,
    revisions,
    now,
    onChange,
  }: {
    action: StoreMutationAction;
    operationHeads?: readonly string[];
    description: string;
    workspace: string;
    bookmark: string;
    target: string;
    base: string;
    retainBookmarks: boolean;
    restoreDescendants: boolean;
    selectedRevision?: RepositoryLogRevision;
    revisions: readonly RepositoryLogRevision[];
    now?: number;
    onChange: () => void;
  } = $props();

  let previousDescription = $state(description);
  $effect(() => {
    const current = description;
    if (current === previousDescription) return;
    previousDescription = current;
    untrack(onChange);
  });
</script>

<Field.Group>
  {#if action === "new" || action === "describe"}
    <Field.Field>
      <Field.Label>Description</Field.Label>
      <CommunityChatComposer
        mode="form"
        showSubmitButton={false}
        bind:value={description}
        ariaLabel="Description"
        placeholder={action === "new"
          ? "Describe the new change"
          : "Describe the selected change"}
        class="store-mutation-description-composer"
      />
    </Field.Field>
  {/if}
  {#if action === "new"}
    <Field.Field>
      <Field.Label for="store-mutation-workspace">Workspace</Field.Label>
      <Input
        id="store-mutation-workspace"
        bind:value={workspace}
        oninput={onChange}
        placeholder="Optional workspace name"
      />
    </Field.Field>
  {/if}
  {#if action.startsWith("bookmark-")}
    <Field.Field>
      <Field.Label for="store-mutation-bookmark">Bookmark</Field.Label>
      <Input
        id="store-mutation-bookmark"
        bind:value={bookmark}
        oninput={onChange}
        disabled={action !== "bookmark-create"}
      />
    </Field.Field>
  {/if}
  {#if action === "describe" || action === "abandon"}
    <Field.Field>
      <Field.Label>Commit</Field.Label>
      <StoreMutationCommitTarget {target} revision={selectedRevision} {now} />
    </Field.Field>
  {:else if action === "new" || action === "bookmark-create" || action === "bookmark-move"}
    <Field.Field>
      <Field.Label for="store-mutation-target">
        {action === "new" ? "Parent commit" : "Commit"}
      </Field.Label>
      <StoreMutationCommitPicker
        id="store-mutation-target"
        label={action === "new" ? "Parent commit" : "Commit"}
        bind:target
        {revisions}
        {now}
        {onChange}
      />
    </Field.Field>
  {/if}
  {#if action === "reconcile"}
    <Field.Field>
      <Field.Label for="store-mutation-base">Base operation</Field.Label>
      {#if operationHeads}
        <Select.Root type="single" bind:value={base} onValueChange={onChange}>
          <Select.Trigger id="store-mutation-base" aria-label="Base operation"
            >{base
              ? `${base.slice(0, 12)}…`
              : "Choose operation"}</Select.Trigger
          >
          <Select.Content
            >{#each operationHeads as operation}<Select.Item
                value={operation}
                label={operation}>{operation.slice(0, 12)}…</Select.Item
              >{/each}</Select.Content
          >
        </Select.Root>
      {:else}<Input
          id="store-mutation-base"
          bind:value={base}
          oninput={onChange}
          spellcheck="false"
          autocomplete="off"
        />
      {/if}<Field.Description
        >Choose one captured operation head as the base.</Field.Description
      >
    </Field.Field>
  {/if}
  {#if action === "abandon"}
    <Field.Set>
      <Field.Legend>Bookmark and descendant handling</Field.Legend>
      <Field.Group>
        <Field.Field orientation="horizontal">
          <Checkbox
            id="store-mutation-retain-bookmarks"
            checked={retainBookmarks}
            onCheckedChange={(checked) => {
              retainBookmarks = checked === true;
              onChange();
            }}
          />
          <Field.Label for="store-mutation-retain-bookmarks"
            >Retain bookmarks</Field.Label
          >
        </Field.Field>
        <Field.Field orientation="horizontal">
          <Checkbox
            id="store-mutation-restore-descendants"
            checked={restoreDescendants}
            onCheckedChange={(checked) => {
              restoreDescendants = checked === true;
              onChange();
            }}
          />
          <Field.Label for="store-mutation-restore-descendants"
            >Restore descendants</Field.Label
          >
        </Field.Field>
      </Field.Group>
    </Field.Set>
  {/if}
</Field.Group>
