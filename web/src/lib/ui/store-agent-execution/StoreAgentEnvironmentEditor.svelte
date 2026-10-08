<script lang="ts">
  import StoreSettingsSelect from "../store-settings/StoreSettingsSelect.svelte";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import type { AgentEnvironment } from "../../controller/agent-settings.svelte.js";
  let {
    value = $bindable({}),
    secrets = $bindable({}),
    disabled = false,
    keychainSupported = false,
  }: {
    value?: AgentEnvironment;
    secrets?: Record<string, string>;
    disabled?: boolean;
    keychainSupported?: boolean;
  } = $props();
  let name = $state("");
  const id = $props.id();
  function add() {
    const key = name.trim();
    if (!/^[A-Za-z_][A-Za-z0-9_]*$/.test(key) || key in value) return;
    value = { ...value, [key]: { source: "literal", value: "" } };
    name = "";
  }
  function source(key: string, next: string) {
    const previous = value[key];
    if (previous.source === "keychain") delete secrets[previous.value];
    const ref = crypto.randomUUID();
    value[key] =
      next === "unset"
        ? { source: "unset" }
        : next === "keychain"
          ? { source: "keychain", value: ref }
          : next === "environment"
            ? { source: "environment", value: "" }
            : { source: "literal", value: "" };
  }
</script>

<Field.Group>
  {#if !Object.keys(value).length}<Field.Description
      >No variables set.</Field.Description
    >{/if}
  {#each Object.entries(value) as [key, entry] (key)}
    <Field.Field>
      <Field.Label for={`${id}-${key}`}>{key}</Field.Label>
      <StoreSettingsSelect
        aria-label={`Source for ${key}`}
        value={entry.source}
        {disabled}
        onValueChange={(next) => source(key, next)}
        options={[
          { value: "literal", label: "Value" },
          { value: "environment", label: "Server environment reference" },
          {
            value: "keychain",
            label: "Secret in device Keychain",
            disabled: !keychainSupported,
          },
          { value: "unset", label: "Unset inherited variable" },
        ]}
      />
      {#if entry.source === "keychain"}
        <Input
          id={`${id}-${key}`}
          type="password"
          autocomplete="new-password"
          {disabled}
          placeholder="Secret in Keychain; enter a replacement"
          value={secrets[entry.value] ?? ""}
          oninput={(event) => {
            if (!event.currentTarget.value && !(entry.value in secrets)) return;
            if (!(entry.value in secrets)) {
              const ref = crypto.randomUUID();
              value[key] = { source: "keychain", value: ref };
              secrets[ref] = event.currentTarget.value;
            } else secrets[entry.value] = event.currentTarget.value;
          }}
        />
      {:else if entry.source !== "unset"}
        <Input
          id={`${id}-${key}`}
          {disabled}
          value={entry.value}
          placeholder={entry.source === "environment"
            ? "Environment variable name on the server"
            : "Value"}
          oninput={(event) => {
            value[key] = { ...entry, value: event.currentTarget.value };
          }}
        />
      {/if}
      <Button
        type="button"
        variant="ghost"
        {disabled}
        onclick={() => {
          if (entry.source === "keychain") delete secrets[entry.value];
          const next = { ...value };
          delete next[key];
          value = next;
        }}>Remove {key}</Button
      >
    </Field.Field>
  {/each}
  <Field.Field>
    <Field.Label for={`${id}-new`}>Environment variable name</Field.Label>
    <Input
      id={`${id}-new`}
      bind:value={name}
      {disabled}
      placeholder="VARIABLE_NAME"
    />
    <Button
      type="button"
      variant="outline"
      disabled={disabled ||
        !/^[A-Za-z_][A-Za-z0-9_]*$/.test(name) ||
        name in value}
      onclick={add}>Add variable</Button
    >
  </Field.Field>
  <Field.Description
    >Agent variables override global values. Use a secret or reference for
    credentials. Secrets require at least eight characters. Saved changes apply
    on the next start.</Field.Description
  >
</Field.Group>
