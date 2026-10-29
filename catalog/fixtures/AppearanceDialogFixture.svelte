<script lang="ts">
  import FixtureControls from "./FixtureControls.svelte";
  import * as Dialog from "@lapismd/design-core/shadcn/dialog";
  import * as AlertDialog from "@lapismd/design-core/shadcn/alert-dialog";
  import { DialogFrame } from "@lapismd/design-core/forms/dialog";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { ownAppearance } from "../../web/src/lib/appearance/runtime.svelte.js";
  import { createAppearanceStore } from "../../web/src/lib/appearance/store.svelte.js";
  import {
    lightThemes,
    themeNames,
  } from "../../web/src/lib/appearance/theme-catalog.js";
  import StoreWorkField from "../../web/src/lib/ui/store-work/StoreWorkField.svelte";
  import CommunityChatComposer from "../../web/src/lib/ui/community-chat-composer/CommunityChatComposer.svelte";
  import { createMemoryStoryStorage } from "./memory-storage.js";

  let { alert = false }: { alert?: boolean } = $props();
  const appearance = ownAppearance(
    createAppearanceStore(createMemoryStoryStorage()),
  );
  let index = $state(0);
  let title = $state("Retained task title");
  let body = $state("Retained task description");
  function select() {
    const theme = themeNames[index]!;
    appearance.update({
      theme,
      mode: lightThemes.has(theme) ? "light" : "dark",
    });
  }
  function next() {
    index = (index + 1) % themeNames.length;
    select();
  }
  select();
</script>

<div class="appearance-dialog-fixture">
  <h1>Modal palettes</h1>
  <p>Secondary text follows the selected app palette.</p>
</div>
{#if alert}
  <AlertDialog.Root open>
    <AlertDialog.Content>
      <AlertDialog.Header>
        <AlertDialog.Title>Confirm action</AlertDialog.Title>
        <AlertDialog.Description
          >Secondary text follows the selected app palette.</AlertDialog.Description
        >
      </AlertDialog.Header>
      <FixtureControls floating={false}
        ><button type="button" onclick={next}>Next palette</button
        ></FixtureControls
      >
    </AlertDialog.Content>
  </AlertDialog.Root>
{:else}
  <Dialog.Root open>
    <DialogFrame
      title="Create task"
      description="Secondary text follows the selected app palette."
      preventScroll={false}
    >
      <Field.Group class="community-settings__option-group">
        <StoreWorkField
          label="Task title"
          id="appearance-task-title"
          description="Name the work to complete."
        >
          <Input id="appearance-task-title" bind:value={title} />
        </StoreWorkField>
        <StoreWorkField
          label="Task description"
          compact
          description="Add context and acceptance criteria with Markdown."
        >
          <CommunityChatComposer
            bind:value={body}
            mode="form"
            ariaLabel="Task description"
            placeholder="Describe the task"
          />
        </StoreWorkField>
      </Field.Group>
      <FixtureControls floating={false}
        ><button type="button" onclick={next}>Next palette</button
        ></FixtureControls
      >
    </DialogFrame>
  </Dialog.Root>
{/if}

<style>
  .appearance-dialog-fixture {
    background: var(--background);
    color: var(--foreground);
  }
  .appearance-dialog-fixture p {
    color: var(--muted-foreground);
  }
</style>
