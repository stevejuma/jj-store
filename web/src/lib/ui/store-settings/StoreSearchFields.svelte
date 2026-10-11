<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import StoreSearchAdvanced from "./StoreSearchAdvanced.svelte";
  import type {
    SearchDraft,
    SearchConfiguration,
  } from "../../controller/search-settings.js";
  let {
    draft,
    effective,
    busy,
    section,
    onDraft,
  }: {
    draft: SearchDraft;
    effective?: SearchConfiguration;
    busy: boolean;
    section: "configuration" | "scopes" | "advanced";
    onDraft: (patch: Partial<SearchDraft>) => void;
  } = $props();
</script>

<Field.Group class="store-settings__editable-rows">
  {#if section === "configuration"}
    <Field.Field orientation="horizontal" class="store-settings__option-row">
      <Field.Content>
        <Field.Label for="search-enabled">Enable text search</Field.Label>
        <Field.Description id="search-enabled-help"
          >Search repository metadata and any selected file scopes.</Field.Description
        >
      </Field.Content>
      <Switch
        id="search-enabled"
        aria-describedby="search-enabled-help"
        checked={draft.enabled}
        disabled={busy}
        onCheckedChange={(enabled) => onDraft({ enabled })}
      />
    </Field.Field>
  {:else if section === "scopes"}
    <Field.Field orientation="horizontal" class="store-settings__option-row">
      <Field.Content>
        <Field.Label for="search-source"
          >Index file content in selected scopes</Field.Label
        >
        <Field.Description id="search-source-help"
          >Metadata is indexed when search is enabled. File content requires
          explicit bookmark or workspace scopes.</Field.Description
        >
      </Field.Content>
      <Switch
        id="search-source"
        aria-describedby="search-source-help"
        checked={draft.indexSource}
        disabled={busy}
        onCheckedChange={(indexSource) => onDraft({ indexSource })}
      />
    </Field.Field>
    {#each [{ key: "bookmarks", label: "Bookmark scopes", help: "One per line: scope=bookmark. Example: main=main." }, { key: "workspaces", label: "Workspace scopes", help: "One per line: scope=workspace. Example: local=default." }, { key: "include", label: "Include path prefixes", help: "One repository-relative prefix per line. Empty includes all paths in the selected scopes." }, { key: "exclude", label: "Exclude path prefixes", help: "One repository-relative prefix per line. Exclusions take precedence." }] as field}
      <Field.Field orientation="responsive" class="store-settings__option-row">
        <Field.Content
          ><Field.Label for={`search-${field.key}`}>{field.label}</Field.Label>
          <Field.Description id={`search-${field.key}-help`}
            >{field.help}</Field.Description
          >
        </Field.Content>
        <Textarea
          class="store-settings__input"
          id={`search-${field.key}`}
          value={draft[
            field.key as "bookmarks" | "workspaces" | "include" | "exclude"
          ]}
          disabled={busy}
          rows={2}
          maxlength={16384}
          spellcheck={false}
          oninput={(event) =>
            onDraft({ [field.key]: event.currentTarget.value })}
          aria-describedby={`search-${field.key}-help`}
        />
      </Field.Field>
    {/each}
  {/if}
  {#if section === "advanced"}<StoreSearchAdvanced
      {draft}
      {effective}
      disabled={busy}
      {onDraft}
    />{/if}
  {#if section === "configuration"}
    <Field.Field orientation="responsive" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for="search-budget">Index budget (MiB)</Field.Label>
        <Field.Description id="search-budget-help"
          >Set the text index budget in mebibytes.</Field.Description
        >
      </Field.Content><Input
        class="store-settings__input"
        id="search-budget"
        aria-describedby="search-budget-help"
        type="number"
        min="0.000001"
        max="2048"
        step="any"
        required
        disabled={busy}
        value={draft.budget}
        oninput={(event) => onDraft({ budget: event.currentTarget.value })}
      /></Field.Field
    >
  {/if}
</Field.Group>
