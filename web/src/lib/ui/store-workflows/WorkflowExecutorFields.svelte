<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import Terminal from "@lucide/svelte/icons/terminal";
  import Bot from "@lucide/svelte/icons/bot";
  import Message from "@lucide/svelte/icons/message-square";
  import WorkflowIcon from "@lucide/svelte/icons/workflow";
  import WorkflowFilePicker from "./WorkflowFilePicker.svelte";
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  import CommunityChatComposer from "../community-chat-composer/CommunityChatComposer.svelte";
  import type { WorkflowDefinition } from "../../controller/workflow-types.js";
  import type { StoreController } from "../../controller/contracts.js";
  let {
    id,
    action,
    repository,
    acquireRepository,
    disabled,
  }: {
    id: string;
    action: WorkflowDefinition["actions"][number];
    repository: string;
    acquireRepository?: StoreController["acquireRepository"];
    disabled: boolean;
  } = $props();
</script>

{#if action.executor.type === "command"}<Field.Field
    orientation="horizontal"
    class="store-settings__option-row"
    ><Field.Content
      ><Field.Label for={`${id}-command`}
        ><StoreSettingsIcon Icon={Terminal} />Command name</Field.Label
      ><Field.Description>Configured on the execution host.</Field.Description
      ></Field.Content
    ><Input
      id={`${id}-command`}
      bind:value={action.executor.name}
      {disabled}
    /></Field.Field
  >
{:else if action.executor.type === "github_workflow"}
  <Field.Field
    orientation="horizontal"
    class="store-settings__option-row workflow-inspector-row--wide"
  >
    <Field.Content
      ><Field.Label for={`${id}-workflow-file`}
        ><StoreSettingsIcon Icon={WorkflowIcon} />Workflow file</Field.Label
      >
      <Field.Description
        >The recorded revision supplies this file. Its YAML triggers must also
        match.</Field.Description
      ></Field.Content
    >
    <WorkflowFilePicker
      id={`${id}-workflow-file`}
      {repository}
      {acquireRepository}
      bind:value={action.executor.path}
      {disabled}
    />
  </Field.Field>
{:else}<Field.Field orientation="horizontal" class="store-settings__option-row"
    ><Field.Content
      ><Field.Label for={`${id}-agent`}
        ><StoreSettingsIcon Icon={Bot} />Agent</Field.Label
      ></Field.Content
    ><Input
      id={`${id}-agent`}
      bind:value={action.executor.agent}
      {disabled}
    /></Field.Field
  ><Field.Field
    orientation="horizontal"
    class="store-settings__option-row workflow-inspector-row--wide"
    ><Field.Content
      ><Field.Label for={`${id}-prompt`}
        ><StoreSettingsIcon Icon={Message} />Prompt</Field.Label
      ></Field.Content
    ><CommunityChatComposer
      ariaLabel="Prompt"
      bind:value={action.executor.prompt}
      mode="form"
      {disabled}
    /></Field.Field
  >{/if}
