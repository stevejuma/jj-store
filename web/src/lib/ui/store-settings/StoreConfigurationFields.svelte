<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import StoreNumericSettingNumberRow from "./StoreNumericSettingNumberRow.svelte";
  import { bytesUnits, countUnits, secondsUnits } from "./numeric-setting.js";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import {
    serviceFields,
    copy,
    type Configuration,
  } from "../../controller/server-configuration.js";
  import type { ServerSettingsManagement } from "../../controller/server-settings.svelte.js";
  let {
    management,
    section,
  }: {
    management: ServerSettingsManagement;
    section: "service" | "protection";
  } = $props();
  const edit = $derived(management.state.drafts[section]);
  const disabled = $derived(management.state.saving || Boolean(edit.unknown));
  const service = $derived(edit.draft as Configuration["service"]);
  const protection = $derived(edit.draft as Configuration["protection"]);
  let texts = $state({ bookmarks: "", commits: "" });
  $effect(() => {
    if (section === "protection" && !edit.dirty && protection)
      texts = {
        bookmarks: protection.bookmarks.join("\n"),
        commits: protection.commits.join("\n"),
      };
  });
  function rules(key: "bookmarks" | "commits", text: string) {
    texts[key] = text;
    management.configure("protection", {
      ...copy(protection),
      [key]: [...new Set(text.split("\n").filter(Boolean))].sort(),
    });
  }
</script>

<Field.Group class="store-settings__editable-rows">
  {#if section === "service"}
    {#each serviceFields as field}
      <StoreNumericSettingNumberRow
        id={`service-${field.key}`}
        label={field.label}
        min={field.min}
        max={field.max}
        {disabled}
        units={field.key === "request_seconds"
          ? secondsUnits
          : field.key === "max_blob_bytes"
            ? bytesUnits
            : countUnits(
                field.key === "inventory_nodes" ? "nodes" : "requests",
              )}
        value={service[field.key]}
        description={`Running: ${management.state.value?.effective.service[field.key].toLocaleString("en-GB")}`}
        onValueChange={(value) =>
          management.configure("service", {
            ...copy(service),
            [field.key]: value,
          })}
      />
    {/each}
  {:else}
    {#each [{ key: "bookmarks", label: "Protected bookmarks", help: "One exact bookmark name per line." }, { key: "commits", label: "Protected commits", help: "One full 128-character commit ID per line." }] as field}
      <Field.Field orientation="responsive" class="store-settings__option-row">
        <Field.Content
          ><Field.Label for={`protection-${field.key}`}
            >{field.label}</Field.Label
          >
          <Field.Description
            >{field.help} Applies to every native repository on this installation.</Field.Description
          >
        </Field.Content>
        <Textarea
          class="store-settings__input"
          id={`protection-${field.key}`}
          rows={4}
          maxlength={60000}
          {disabled}
          spellcheck={false}
          value={texts[field.key as "bookmarks" | "commits"]}
          oninput={(event) =>
            rules(
              field.key as "bookmarks" | "commits",
              event.currentTarget.value,
            )}
        />
      </Field.Field>
    {/each}
  {/if}
</Field.Group>
