<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import Message from "@lucide/svelte/icons/message-square";
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  import WorkflowSelect from "./WorkflowSelect.svelte";
  import type { WorkflowDefinition } from "../../controller/workflow-types.js";
  let {
    id,
    definition = $bindable(),
    disabled = false,
    conversational = false,
  }: {
    id: string;
    definition: WorkflowDefinition;
    disabled?: boolean;
    conversational?: boolean;
  } = $props();
  const conditions = $derived(definition.conditions ?? {});
  function update(
    fields: Partial<NonNullable<WorkflowDefinition["conditions"]>>,
  ) {
    definition.conditions = { ...conditions, ...fields };
  }
  const operators = [
    { value: "contains", label: "Contains" },
    { value: "not_contains", label: "Does not contain" },
    { value: "starts_with", label: "Starts with" },
    { value: "ends_with", label: "Ends with" },
    { value: "equals", label: "Is" },
    { value: "not_equals", label: "Is not" },
    { value: "is_empty", label: "Is empty" },
    { value: "is_not_empty", label: "Is not empty" },
  ];
</script>

<Field.Field
  orientation="horizontal"
  class="store-settings__option-row workflow-inspector-row--wide"
>
  <Field.Content
    ><Field.Label for={`${id}-text`}
      ><StoreSettingsIcon Icon={Message} />{conversational
        ? "Message text"
        : "Commit description"}</Field.Label
    ><Field.Description
      >Optional case-sensitive text condition.</Field.Description
    ></Field.Content
  >
  <div class="workflow-conditions__fields">
    <WorkflowSelect
      id={`${id}-text-mode`}
      label="Text match"
      value={conditions.text?.operator ?? "any"}
      {disabled}
      options={[{ value: "any", label: "Any text" }, ...operators]}
      onChange={(operator) =>
        update({
          text:
            operator === "any"
              ? undefined
              : {
                  operator: operator as NonNullable<
                    NonNullable<WorkflowDefinition["conditions"]>["text"]
                  >["operator"],
                  value: conditions.text?.value ?? "",
                },
        })}
    />
    {#if conditions.text && !["is_empty", "is_not_empty"].includes(conditions.text.operator)}<Input
        id={`${id}-text`}
        aria-label={conversational ? "Message text" : "Commit description"}
        value={conditions.text.value}
        {disabled}
        oninput={(event) =>
          update({
            text: {
              operator: conditions.text!.operator,
              value: event.currentTarget.value,
            },
          })}
      />{/if}
  </div>
</Field.Field>

<style>
  .workflow-conditions__fields {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    margin-left: 2.3rem;
    width: calc(100% - 2.3rem);
  }
  .workflow-conditions__fields :global(input) {
    min-width: 8rem;
    flex: 1;
  }
</style>
