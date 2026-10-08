<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Switch } from "@lapismd/design-core/shadcn/switch";
  import StoreWorkField from "../store-work/StoreWorkField.svelte";
  import type { AgentSettings } from "../../controller/agent-settings.svelte.js";
  let { settings }: { settings: AgentSettings } = $props();
  const id = $props.id();
</script>

<section
  class="store-settings__section"
  aria-labelledby={`${id}-conversations`}
>
  <header class="store-settings__section-heading">
    <h2 id={`${id}-conversations`}>Conversations</h2>
  </header>
  <Field.Group class="community-settings__option-group">
    <StoreWorkField
      label="Automatically mention agents"
      id={`${id}-mention`}
      description="Keep selected agents addressed in replies. Saves immediately and stays in sync with the chat composer."
    >
      <Switch
        id={`${id}-mention`}
        bind:checked={
          () => settings.value?.automatically_mention ?? false,
          (enabled) => {
            void settings.setAutomaticallyMention(enabled);
          }
        }
        disabled={!settings.value || settings.busy || !!settings.pending}
      />
    </StoreWorkField>
  </Field.Group>
</section>
