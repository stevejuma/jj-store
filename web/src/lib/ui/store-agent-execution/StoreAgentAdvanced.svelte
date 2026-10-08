<script lang="ts">
  import StoreSettingsDisclosure from "../store-settings/StoreSettingsDisclosure.svelte";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import * as Field from "@lapismd/design-core/shadcn/field";
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

<Field.Group
  class="community-settings__option-group"
  data-ui-part="agent-advanced-fields"
>
  <StoreSettingsDisclosure>
    {#if draft.options}
      <Field.Group class="community-settings__option-group">
        <StoreWorkField
          label="Inherit application defaults"
          id={`${id}-inherit`}
        >
          <Switch
            id={`${id}-inherit`}
            bind:checked={draft.options.inherit_defaults}
            disabled={pending}
          />
        </StoreWorkField>
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
</Field.Group>
