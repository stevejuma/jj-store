<script lang="ts">
  import StoreSearchAdvanced from "./StoreSearchAdvanced.svelte";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import StoreSettingsNotice from "./StoreSettingsNotice.svelte";
  import SearchXIcon from "@lucide/svelte/icons/search-x";
  import StoreSettingsEmpty from "./StoreSettingsEmpty.svelte";
  import SaveIcon from "@lucide/svelte/icons/save";
  import XIcon from "@lucide/svelte/icons/x";
  import CheckIcon from "@lucide/svelte/icons/check";
  import RotateCwIcon from "@lucide/svelte/icons/rotate-cw";
  import DatabaseIcon from "@lucide/svelte/icons/database";
  import * as Field from "@lapismd/design-core/shadcn/field";
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
    onDraft,
    onCancel,
    onSave,
    onControl,
  }: {
    settings: SearchSettings;
    supported: boolean;
    section?: "configuration" | "scopes" | "advanced";
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
          : section === "scopes"
            ? "File search scopes"
            : "Search worker limits"}
      </h2>
      <p>Save your settings, then apply them to this repository.</p>
    </div>
  </header>
  {#if !supported}<StoreSettingsEmpty
      Icon={SearchXIcon}
      title="Search editing unavailable"
      description="Search editing is not available for this connection."
    />
  {:else}
    {#if settings.draft}
      <form
        onsubmit={(event) => {
          event.preventDefault();
          void onSave();
        }}
      >
        <Field.Group>
          <div class="store-settings__rows">
            {#if settings.value}
              <dl class="store-settings__summary-rows">
                <div>
                  <dt>Saved</dt>
                  <dd>
                    {settings.value.desired.enabled ? "Enabled" : "Disabled"}
                  </dd>
                </div>
                <div>
                  <dt>Currently active</dt>
                  <dd>
                    {settings.value.effective.enabled
                      ? "Enabled"
                      : "Disabled"}{pending
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
            <Field.Group class="store-settings__editable-rows">
              {#if section === "configuration"}
                <Field.Field
                  orientation="horizontal"
                  class="store-settings__option-row"
                >
                  <Field.Content>
                    <Field.Label for="search-enabled"
                      >Enable text search</Field.Label
                    >
                    <Field.Description id="search-enabled-help"
                      >Search repository metadata and any selected file scopes.</Field.Description
                    >
                  </Field.Content>
                  <Switch
                    id="search-enabled"
                    aria-describedby="search-enabled-help"
                    checked={settings.draft.enabled}
                    disabled={busy}
                    onCheckedChange={(enabled) => onDraft({ enabled })}
                  />
                </Field.Field>
              {:else if section === "scopes"}
                <Field.Field
                  orientation="horizontal"
                  class="store-settings__option-row"
                >
                  <Field.Content>
                    <Field.Label for="search-source"
                      >Index file content in selected scopes</Field.Label
                    >
                    <Field.Description id="search-source-help"
                      >Metadata is indexed when search is enabled. File content
                      requires explicit bookmark or workspace scopes.</Field.Description
                    >
                  </Field.Content>
                  <Switch
                    id="search-source"
                    aria-describedby="search-source-help"
                    checked={settings.draft.indexSource}
                    disabled={busy}
                    onCheckedChange={(indexSource) => onDraft({ indexSource })}
                  />
                </Field.Field>
                {#each [{ key: "bookmarks", label: "Bookmark scopes", help: "One per line: scope=bookmark. Example: main=main." }, { key: "workspaces", label: "Workspace scopes", help: "One per line: scope=workspace. Example: local=default." }, { key: "include", label: "Include path prefixes", help: "One repository-relative prefix per line. Empty includes all paths in the selected scopes." }, { key: "exclude", label: "Exclude path prefixes", help: "One repository-relative prefix per line. Exclusions take precedence." }] as field}
                  <Field.Field
                    orientation="responsive"
                    class="store-settings__option-row"
                  >
                    <Field.Content
                      ><Field.Label for={`search-${field.key}`}
                        >{field.label}</Field.Label
                      >
                      <Field.Description id={`search-${field.key}-help`}
                        >{field.help}</Field.Description
                      >
                    </Field.Content>
                    <Textarea
                      class="store-settings__input"
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
                  </Field.Field>
                {/each}
              {/if}
              {#if section === "advanced"}<StoreSearchAdvanced
                  draft={settings.draft}
                  effective={settings.value?.effective}
                  disabled={busy}
                  {onDraft}
                />{/if}
              {#if section === "configuration"}
                <Field.Field
                  orientation="responsive"
                  class="store-settings__option-row"
                >
                  <Field.Content
                    ><Field.Label for="search-budget"
                      >Index budget (MiB)</Field.Label
                    >
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
                    value={settings.draft.budget}
                    oninput={(event) =>
                      onDraft({ budget: event.currentTarget.value })}
                  /></Field.Field
                >
              {/if}
            </Field.Group>
          </div>
          <div class="store-settings__action-row">
            <div>
              <strong>Save configuration</strong>
              <p>
                Saving preserves the running configuration until Apply or
                service restart.
              </p>
            </div>
            <div class="store-settings__actions">
              <Button type="submit" disabled={!ready || !settings.dirty}
                ><SaveIcon data-icon="inline-start" aria-hidden="true" />Save
                settings</Button
              ><Button
                type="button"
                variant="outline"
                disabled={busy || !settings.dirty}
                onclick={onCancel}
                ><XIcon data-icon="inline-start" aria-hidden="true" />Cancel
                changes</Button
              >
            </div>
          </div>
        </Field.Group>
      </form>
      <div class="store-settings__action-row">
        <div>
          <strong>Index maintenance</strong>
          <p>
            Apply verifies a replacement before activation. Rebuild recreates
            the text index from committed data. Repository work stays available.
          </p>
        </div>
        <div class="store-settings__actions">
          <Button
            variant="outline"
            disabled={!ready || settings.dirty || !pending}
            onclick={() => void onControl("apply")}
            ><CheckIcon data-icon="inline-start" aria-hidden="true" />Apply
            saved settings</Button
          >
          <Button
            variant="outline"
            disabled={!ready ||
              settings.dirty ||
              !!pending ||
              !settings.value?.effective.enabled}
            onclick={() => void onControl("retry")}
            ><RotateCwIcon data-icon="inline-start" aria-hidden="true" />Retry
            indexing</Button
          >
          <Button
            variant="outline"
            disabled={!ready ||
              settings.dirty ||
              !!pending ||
              !settings.value?.effective.enabled}
            onclick={() => void onControl("rebuild")}
            ><DatabaseIcon data-icon="inline-start" aria-hidden="true" />Rebuild
            text index</Button
          >
        </div>
      </div>
    {:else if settings.changing}<StoreSettingsNotice
        >Loading search settings…</StoreSettingsNotice
      >{/if}
    {#if settings.validation}<p role="alert">{settings.validation}</p>{/if}
    <StoreNotice
      noticeKey="search-settings"
      title="Search settings need attention"
      message={settings.error || settings.value?.error}
      pending={settings.changing}
    />
  {/if}
</section>
