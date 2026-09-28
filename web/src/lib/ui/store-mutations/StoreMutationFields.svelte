<script lang="ts">
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { Checkbox } from "@lapismd/design-core/shadcn/checkbox";
  import * as Field from "@lapismd/design-core/shadcn/field";
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
    onChange: () => void;
  } = $props();
</script>

<Field.Group>
  {#if action === "new" || action === "describe"}
    <Field.Field>
      <Field.Label for="store-mutation-description">Description</Field.Label>
      <Textarea
        id="store-mutation-description"
        bind:value={description}
        oninput={onChange}
        rows={4}
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
  {#if action === "new" || action === "describe" || action === "bookmark-create" || action === "bookmark-move" || action === "abandon"}
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
        disabled={action === "describe" || action === "abandon"}
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
