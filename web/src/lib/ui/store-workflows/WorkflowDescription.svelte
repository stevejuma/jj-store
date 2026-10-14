<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Textarea } from "@lapismd/design-core/shadcn/textarea";
  import FileText from "@lucide/svelte/icons/file-text";
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  let {
    id,
    value = $bindable(""),
    disabled = false,
  }: {
    id: string;
    value?: string;
    disabled?: boolean;
  } = $props();
</script>

<Field.Field
  orientation="horizontal"
  class="store-settings__option-row workflow-inspector-row--wide"
>
  <Field.Content
    ><Field.Label for={`${id}-description`}
      ><StoreSettingsIcon Icon={FileText} />Description</Field.Label
    >
    <Field.Description>Describe what this workflow does.</Field.Description
    ></Field.Content
  >
  <Textarea id={`${id}-description`} bind:value rows={3} {disabled} />
</Field.Field>
