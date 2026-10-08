<script lang="ts">
  import * as ButtonGroup from "@lapismd/design-core/shadcn/button-group";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import FileIcon from "@lucide/svelte/icons/file";
  let {
    id,
    value = $bindable(""),
    supported = false,
    disabled = false,
    onBrowse,
  }: {
    id: string;
    value?: string;
    supported?: boolean;
    disabled?: boolean;
    onBrowse: () => Promise<string | null>;
  } = $props();
</script>

<ButtonGroup.Root
  class="store-settings__field-action"
  style="width: 100%"
  aria-label="Choose GGUF model file"
>
  <Input
    {id}
    bind:value
    {disabled}
    placeholder="/absolute/path/model.gguf"
    maxlength={4096}
    autocomplete="off"
    spellcheck={false}
  />
  {#if supported}<Button
      type="button"
      variant="outline"
      {disabled}
      onclick={async () => {
        const path = await onBrowse();
        if (path !== null) value = path;
      }}><FileIcon data-icon="inline-start" />Browse…</Button
    >{/if}
</ButtonGroup.Root>
