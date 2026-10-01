<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import * as Alert from "@lapismd/design-core/shadcn/alert";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import { searchOperating } from "../../controller/production-search-settings.js";
  import type {
    SearchSettings,
    SearchDraft,
    SearchAction,
  } from "../../controller/search-settings.js";
  let {
    settings,
    supported,
    section = "configuration",
    onRefresh,
    onDraft,
    onCancel,
    onSave,
    onControl,
  }: {
    settings: SearchSettings;
    supported: boolean;
    section?: "configuration" | "scopes";
    onRefresh: () => Promise<void>;
    onDraft: (patch: Partial<SearchDraft>) => void;
    onCancel: () => void;
    onSave: () => Promise<void>;
    onControl: (action: SearchAction) => Promise<void>;
  } = $props();
  const busy = $derived(
    settings.changing || searchOperating(settings.value?.operation),
  );
  const ready = $derived(settings.state === "ready" && !busy);
  const pending = $derived(
    settings.value &&
      JSON.stringify(settings.value.desired) !==
        JSON.stringify(settings.value.effective),
  );
</script>

<section class="store-settings__section" aria-labelledby="search-configuration">
  <header class="store-settings__section-heading">
    <div>
      <h2 id="search-configuration">
        {section === "configuration"
          ? "Text-search configuration"
          : "File search scopes"}
      </h2>
      <p>Save your settings, then apply them to this repository.</p>
    </div>
    {#if supported}<Button
        variant="outline"
        disabled={settings.changing}
        onclick={() => void onRefresh()}>Refresh settings</Button
      >{/if}
  </header>
  {#if !supported}<p class="store-settings__empty">
      Search editing is not available for this connection.
    </p>
  {:else}
    {#if settings.value}
      <dl class="store-settings__rows">
        <div>
          <dt>Saved</dt>
          <dd>{settings.value.desired.enabled ? "Enabled" : "Disabled"}</dd>
        </div>
        <div>
          <dt>Currently active</dt>
          <dd>
            {settings.value.effective.enabled ? "Enabled" : "Disabled"}{pending
              ? " · Saved changes need applying"
              : " · Matches saved settings"}
          </dd>
        </div>
        <div>
          <dt>Operation</dt>
          <dd><span role="status">{settings.value.operation}</span></dd>
        </div>
      </dl>
    {/if}
    {#if settings.draft}
      <form
        onsubmit={(event) => {
          event.preventDefault();
          void onSave();
        }}
      >
        <Field.Group>
          {#if section === "configuration"}
            <Field.Field orientation="horizontal"
              ><Switch
                id="search-enabled"
                checked={settings.draft.enabled}
                disabled={busy}
                onCheckedChange={(enabled) => onDraft({ enabled })}
              /><Field.Label for="search-enabled"
                >Enable text search</Field.Label
              ></Field.Field
            >
          {:else}
            <Field.Field orientation="horizontal"
              ><Switch
                id="search-source"
                checked={settings.draft.indexSource}
                disabled={busy}
                onCheckedChange={(indexSource) => onDraft({ indexSource })}
              /><Field.Label for="search-source"
                >Index file content in selected scopes</Field.Label
              ></Field.Field
            >
            <p class="store-settings__empty">
              Metadata is indexed when search is enabled. File content requires
              explicit bookmark or workspace scopes.
            </p>
            {#each [{ key: "bookmarks", label: "Bookmark scopes", help: "One per line: scope=bookmark. Example: main=main." }, { key: "workspaces", label: "Workspace scopes", help: "One per line: scope=workspace. Example: local=default." }, { key: "include", label: "Include path prefixes", help: "One repository-relative prefix per line. Empty includes all paths in the selected scopes." }, { key: "exclude", label: "Exclude path prefixes", help: "One repository-relative prefix per line. Exclusions take precedence." }] as field}
              <Field.Field
                ><Field.Label for={`search-${field.key}`}
                  >{field.label}</Field.Label
                >
                <Textarea
                  id={`search-${field.key}`}
                  value={settings.draft[
                    field.key as
                      "bookmarks" | "workspaces" | "include" | "exclude"
                  ]}
                  disabled={busy}
                  rows={2}
                  maxlength={16384}
                  spellcheck={false}
                  oninput={(event) =>
                    onDraft({ [field.key]: event.currentTarget.value })}
                  aria-describedby={`search-${field.key}-help`}
                />
                <Field.Description id={`search-${field.key}-help`}
                  >{field.help}</Field.Description
                >
              </Field.Field>
            {/each}
          {/if}
          {#if section === "configuration"}
            <Field.Field
              ><Field.Label for="search-budget">Index budget (MiB)</Field.Label
              ><Input
                id="search-budget"
                type="number"
                min="0.000001"
                max="2048"
                step="any"
                required
                disabled={busy}
                value={settings.draft.budget}
                oninput={(event) =>
                  onDraft({ budget: event.currentTarget.value })}
              /></Field.Field
            >
          {/if}
          <div class="store-settings__action-row">
            <div class="flex flex-wrap gap-2">
              <Button type="submit" disabled={!ready || !settings.dirty}
                >Save settings</Button
              ><Button
                type="button"
                variant="outline"
                disabled={busy || !settings.dirty}
                onclick={onCancel}>Cancel changes</Button
              >
            </div>
            <p>
              Saving preserves the running configuration until Apply or service
              restart.
            </p>
          </div>
        </Field.Group>
      </form>
      <div class="store-settings__action-row">
        <div class="flex flex-wrap gap-2">
          <Button
            variant="outline"
            disabled={!ready || settings.dirty || !pending}
            onclick={() => void onControl("apply")}>Apply saved settings</Button
          >
          <Button
            variant="outline"
            disabled={!ready ||
              settings.dirty ||
              !!pending ||
              !settings.value?.effective.enabled}
            onclick={() => void onControl("retry")}>Retry indexing</Button
          >
          <Button
            variant="outline"
            disabled={!ready ||
              settings.dirty ||
              !!pending ||
              !settings.value?.effective.enabled}
            onclick={() => void onControl("rebuild")}>Rebuild text index</Button
          >
        </div>
        <p>
          Apply verifies a replacement before activation. Rebuild recreates the
          text index from committed data. Repository work stays available.
        </p>
      </div>
    {:else if settings.changing}<p role="status">
        Loading search settings…
      </p>{/if}
    {#if settings.error || settings.value?.error}<Alert.Root
        variant="destructive"
        ><Alert.Title>Search settings need attention</Alert.Title
        ><Alert.Description
          >{settings.error || settings.value?.error}</Alert.Description
        ></Alert.Root
      >{/if}
  {/if}
</section>
