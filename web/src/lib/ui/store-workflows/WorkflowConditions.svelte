<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  import WorkflowAuthorPicker from "./WorkflowAuthorPicker.svelte";
  import WorkflowSelect from "./WorkflowSelect.svelte";
  import WorkflowThreadPicker from "./WorkflowThreadPicker.svelte";
  import WorkflowReactionPicker from "./WorkflowReactionPicker.svelte";
  import { untrack } from "svelte";
  import type { StoreController } from "../../controller/contracts.js";
  import type { StoreDataSource } from "../../controller/native-data-source.js";
  import { ownNotifications } from "../../notifications/notification-context.svelte.js";
  import GitBranch from "@lucide/svelte/icons/git-branch";
  import Message from "@lucide/svelte/icons/message-square";
  import Smile from "@lucide/svelte/icons/smile";
  import type { WorkflowDefinition } from "../../controller/workflow-types.js";
  let {
    id,
    definition = $bindable(),
    authors = [],
    acquireRepository,
    disabled = false,
  }: {
    id: string;
    definition: WorkflowDefinition;
    authors?: readonly { id: string; name: string }[];
    acquireRepository?: StoreController["acquireRepository"];
    disabled?: boolean;
  } = $props();
  const conversational = $derived(
    ["message_posted", "reaction_added"].includes(definition.trigger.on),
  );
  const conditions = $derived(definition.conditions ?? {});
  let source = $state<StoreDataSource>();
  const { context } = ownNotifications();
  $effect(() => {
    const repository = definition.repository,
      acquire = acquireRepository,
      needed = conversational;
    const reader = untrack(() => {
      try {
        return needed && repository ? acquire?.(repository) : undefined;
      } catch {
        context.service.publish({
          scope: context.scope,
          key: `workflow-source-${id}`,
          title: "Source repository unavailable",
          message:
            "Choose an accessible source repository to select threads and custom emoji. Your filters are retained.",
          severity: "error",
          lifetime: "condition",
        });
        return undefined;
      }
    });
    source = reader;
    return () => {
      reader?.close();
      context.service.resolve(context.scope, `workflow-source-${id}`);
    };
  });
  let emojiMode = $state<string>();
  const excludedEmoji = $derived(
    emojiMode ? emojiMode === "exclude" : !!conditions.excluded_emoji,
  );
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

{#if definition.trigger.on !== "manual"}
  {#if !conversational}
    <Field.Field orientation="horizontal" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for={`${id}-revset`}
          ><StoreSettingsIcon Icon={GitBranch} />Revision match</Field.Label
        ><Field.Description
          >Optional JJ revset at the published operation.</Field.Description
        ></Field.Content
      >
      <Input
        id={`${id}-revset`}
        aria-label="Revision match"
        placeholder="e.g. bookmarks(main)"
        value={conditions.revset ?? ""}
        {disabled}
        oninput={(event) =>
          update({ revset: event.currentTarget.value || undefined })}
      />
    </Field.Field>
  {/if}
  <WorkflowAuthorPicker
    {id}
    bind:definition
    {authors}
    {disabled}
    {conversational}
  />
  {#if conversational}
    <Field.Field orientation="horizontal" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for={`${id}-thread`}
          ><StoreSettingsIcon Icon={Message} />Thread</Field.Label
        ><Field.Description
          >Leave empty for any thread in the source repository.</Field.Description
        ></Field.Content
      >
      <WorkflowThreadPicker
        id={`${id}-thread`}
        {source}
        value={conditions.thread ?? undefined}
        {disabled}
        onChange={(thread) => update({ thread })}
      />
    </Field.Field>
  {/if}
  {#if definition.trigger.on === "reaction_added"}
    <Field.Field orientation="horizontal" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for={`${id}-message`}
          ><StoreSettingsIcon Icon={Message} />Message</Field.Label
        ><Field.Description>Optional exact message ID.</Field.Description
        ></Field.Content
      >
      <Input
        id={`${id}-message`}
        placeholder="Any message"
        value={conditions.message ?? ""}
        {disabled}
        oninput={(event) =>
          update({ message: event.currentTarget.value || undefined })}
      />
    </Field.Field>
    <Field.Field
      orientation="horizontal"
      class="store-settings__option-row workflow-inspector-row--wide"
    >
      <Field.Content
        ><Field.Label><StoreSettingsIcon Icon={Smile} />Reaction</Field.Label
        ><Field.Description
          >Choose an emoji, Like (+) or Dislike (-).</Field.Description
        ></Field.Content
      >
      <div class="workflow-conditions__fields">
        <WorkflowSelect
          id={`${id}-emoji-mode`}
          label="Reaction match"
          value={excludedEmoji ? "exclude" : "include"}
          {disabled}
          options={[
            { value: "include", label: "Is", Icon: Smile },
            { value: "exclude", label: "Is not", Icon: Smile },
          ]}
          onChange={(mode) => {
            emojiMode = mode;
            update({
              emoji: mode === "include" ? conditions.excluded_emoji : undefined,
              excluded_emoji: mode === "exclude" ? conditions.emoji : undefined,
            });
          }}
        />
        <WorkflowReactionPicker
          id={`${id}-emoji`}
          {source}
          value={conditions.excluded_emoji ?? conditions.emoji ?? undefined}
          {disabled}
          onChange={(emoji) =>
            update(excludedEmoji ? { excluded_emoji: emoji } : { emoji })}
        />
      </div>
    </Field.Field>
  {:else}
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
  {/if}
{/if}

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
