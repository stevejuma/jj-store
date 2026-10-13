<script lang="ts">
  import * as Field from "@lapismd/design-core/shadcn/field";
  import { Input } from "@lapismd/design-core/shadcn/input";
  import StoreSettingsIcon from "../store-settings/StoreSettingsIcon.svelte";
  import WorkflowRevisionMatch from "./WorkflowRevisionMatch.svelte";
  import { createWorkflowCodeOptions } from "./workflow-code-options.svelte.js";
  import { workflowAuthors } from "./workflow-presentation.js";
  import StoreNotice from "../store-notifications/StoreNotice.svelte";
  import WorkflowAuthorPicker from "./WorkflowAuthorPicker.svelte";
  import WorkflowTextCondition from "./WorkflowTextCondition.svelte";
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
    controller,
    disabled = false,
  }: {
    id: string;
    definition: WorkflowDefinition;
    authors?: readonly { id: string; name: string }[];
    acquireRepository?: StoreController["acquireRepository"];
    controller?: StoreController;
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
      needed = definition.trigger.on !== "manual";
    const reader = untrack(() => {
      try {
        return needed && repository ? acquire?.(repository) : undefined;
      } catch {
        context.service.publish({
          scope: context.scope,
          key: `workflow-source-${id}`,
          title: "Source repository unavailable",
          message:
            "Choose an accessible source repository to load trigger choices. Your filters are retained.",
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
  const code = createWorkflowCodeOptions(() =>
    conversational ? undefined : source,
  );
  const knownAuthors = $derived(
    conversational && controller
      ? workflowAuthors(controller)
      : source
        ? code.authors
        : authors,
  );
  $effect(() => {
    const execution = controller?.execution,
      needed = conversational;
    untrack(() => {
      if (needed && execution?.available()) void execution.updates.status();
    });
  });
  $effect(() => {
    if (conversational || definition.trigger.on === "manual") return;
    const refresh = () => void code.refresh();
    const timer = setInterval(refresh, 5000);
    window.addEventListener("focus", refresh);
    return () => {
      clearInterval(timer);
      window.removeEventListener("focus", refresh);
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
</script>

<StoreNotice
  noticeKey={`workflow-code-${id}`}
  title="Trigger choices could not be loaded"
  message={code.error}
  pending={code.loading}
  action={{
    id: "retry-code-choices",
    label: "Retry",
    run: () => void code.refresh(),
  }}
/>
{#if conversational && controller?.execution?.available()}<StoreNotice
    noticeKey={`workflow-agents-${id}`}
    title="Agents could not be loaded"
    message={controller.execution.connectionError}
    action={{
      id: "retry-agent-choices",
      label: "Retry",
      run: () => void controller?.execution?.updates.status(),
    }}
  />{/if}
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
      <WorkflowRevisionMatch
        id={`${id}-revset`}
        value={conditions.revset ?? ""}
        info={code.info}
        symbols={code.symbols}
        {disabled}
        onChange={(revset) => update({ revset: revset || undefined })}
      />
    </Field.Field>
  {/if}
  <WorkflowAuthorPicker
    {id}
    bind:definition
    authors={knownAuthors}
    agents={conversational ? (controller?.execution?.value?.setups ?? []) : []}
    loading={!conversational && code.loading}
    hasMore={!conversational && code.hasMore}
    onLoadMore={() => void code.more()}
    onOpen={() => {
      if (!conversational) void code.refresh();
    }}
    {disabled}
    {conversational}
  />
  {#if conversational}
    <Field.Field orientation="horizontal" class="store-settings__option-row">
      <Field.Content
        ><Field.Label for={`${id}-thread`}
          ><StoreSettingsIcon Icon={Message} />Thread</Field.Label
        ><Field.Description
          >Select an existing thread and its metadata home, or leave empty for
          any thread in the source.</Field.Description
        ></Field.Content
      >
      <WorkflowThreadPicker
        id={`${id}-thread`}
        navigation={controller?.threadNavigation}
        repositories={controller?.state.repositories ?? []}
        repository={definition.repository}
        value={conditions.thread ?? undefined}
        {disabled}
        onChange={(thread, repository) => {
          if (repository) definition.repository = repository;
          update({ thread });
        }}
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
  {:else}<WorkflowTextCondition
      {id}
      bind:definition
      {disabled}
      {conversational}
    />
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
