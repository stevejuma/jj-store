<script lang="ts">
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Checkbox } from "@lapismd/design-core/shadcn/checkbox";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { CommunityChatComposer } from "@lapismd/lapis-community/components/community-chat-composer";
  import { untrack } from "svelte";
  import type { RepositoryLogRevision } from "../../projects/repository-log.js";
  import StoreMutationCommitTarget from "./StoreMutationCommitTarget.svelte";
  import type { StoreMutationAction } from "./model.js";

  let {
    action,
    description = $bindable(),
    workspace = $bindable(),
    bookmark = $bindable(),
    target = $bindable(),
    base = $bindable(),
    retainBookmarks = $bindable(),
    restoreDescendants = $bindable(),
    selectedRevision,
    now,
    onChange,
  }: {
    action: StoreMutationAction;
    description: string;
    workspace: string;
    bookmark: string;
    target: string;
    base: string;
    retainBookmarks: boolean;
    restoreDescendants: boolean;
    selectedRevision?: RepositoryLogRevision;
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
      <Input
        id="store-mutation-target"
        bind:value={target}
        oninput={onChange}
        spellcheck="false"
        autocomplete="off"
      />
      <Field.Description
        >Use the complete 128-character native ID.</Field.Description
      >
    </Field.Field>
  {/if}
  {#if action === "reconcile"}
    <Field.Field>
      <Field.Label for="store-mutation-base">Base operation</Field.Label>
      <Input
        id="store-mutation-base"
        bind:value={base}
        oninput={onChange}
        spellcheck="false"
        autocomplete="off"
      />
      <Field.Description
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
