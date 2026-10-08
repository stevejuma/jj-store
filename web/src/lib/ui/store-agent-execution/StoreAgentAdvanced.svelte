<script lang="ts">
  import StoreSettingsSelect from "../store-settings/StoreSettingsSelect.svelte";
  import StoreSettingsDisclosure from "../store-settings/StoreSettingsDisclosure.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import type { AgentSetup } from "../../controller/agent-execution-types.js";
  import StoreAgentEnvironmentEditor from "./StoreAgentEnvironmentEditor.svelte";
  let {
    draft = $bindable(),
    secrets = $bindable({}),
    pending = false,
    keychainSupported = false,
  }: {
    draft: AgentSetup;
    secrets?: Record<string, string>;
    pending?: boolean;
    keychainSupported?: boolean;
  } = $props();
  const id = $props.id();
</script>

<StoreSettingsDisclosure>
  {#if draft.options}
    <Field.Group class="community-settings__option-group">
      <StoreWorkField label="Inherit application defaults" id={`${id}-inherit`}>
        <Switch
          id={`${id}-inherit`}
          bind:checked={draft.options.inherit_defaults}
          disabled={pending}
        />
      </StoreWorkField>
      {#if draft.runner === "local"}
        <StoreWorkField label="Inference provider" id={`${id}-provider`}>
          <StoreSettingsSelect
            id={`${id}-provider`}
            bind:value={draft.options.provider}
            disabled={pending}
            options={[
              { value: null, label: "Application default" },
              { value: "local", label: "This machine" },
              { value: "paired", label: "Paired compute" },
              { value: "openai", label: "OpenAI compatible endpoint" },
            ]}
          />
        </StoreWorkField>
        {#if draft.options.provider === "openai"}
          <StoreWorkField label="Provider endpoint" id={`${id}-endpoint`}>
            <Input
              id={`${id}-endpoint`}
              value={draft.options.endpoint ?? ""}
              disabled={pending}
              oninput={(event) => {
                draft.options!.endpoint = event.currentTarget.value || null;
              }}
            />
          </StoreWorkField>
        {/if}
        <StoreWorkField
          label="Model override"
          id={`${id}-model`}
          description="Saved provider settings determine model discovery. Availability is checked by the connected harness."
        >
          <Input
            id={`${id}-model`}
            value={draft.preferences.model ?? ""}
            disabled={pending}
            oninput={(event) => {
              draft.preferences.model = event.currentTarget.value || null;
            }}
          />
        </StoreWorkField>
      {/if}
    </Field.Group>
    <h3>Environment variables</h3>
    <StoreAgentEnvironmentEditor
      {keychainSupported}
      bind:value={draft.options.environment}
      bind:secrets
      disabled={pending}
    />
  {/if}
</StoreSettingsDisclosure>
