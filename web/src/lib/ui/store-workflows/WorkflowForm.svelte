<script lang="ts">
  import StoreRepositoryPicker from "../store-application/StoreRepositoryPicker.svelte";
  import { Button } from "@lapismd/design-core/shadcn/button";
  import Plus from "@lucide/svelte/icons/plus";
  import ArrowDown from "@lucide/svelte/icons/arrow-down";
  import "./workflow-sequence.css";
  import WorkflowInspector from "./WorkflowInspector.svelte";
  import WorkflowStep from "./WorkflowStep.svelte";
  import type {
    StoreRepositorySummary,
    StoreController,
  } from "../../controller/contracts.js";
  import type { WorkflowDefinition } from "../../controller/workflow-types.js";
  import { triggerLabel, executorLabel } from "./workflow-presentation.js";
  let {
    definition = $bindable(),
    repositories,
    authors = [],
    acquireRepository,
    home = $bindable(),
    homes,
    homeDisabled = false,
    inspector = $bindable(true),
    disabled = false,
    invalid = $bindable(false),
    onInvalid,
    onValid,
  }: {
    definition: WorkflowDefinition;
    home: string;
    homes: readonly StoreRepositorySummary[];
    homeDisabled?: boolean;
    inspector?: boolean;
    repositories: readonly StoreRepositorySummary[];
    authors?: readonly { id: string; name: string }[];
    acquireRepository?: StoreController["acquireRepository"];
    disabled?: boolean;
    invalid?: boolean;
    onInvalid?(message: string): void;
    onValid?(): void;
  } = $props();
  const id = $props.id();
  let selection = $state(-1);
  function select(index: number) {
    selection = index;
    inspector = true;
  }
  function add(index: number) {
    definition.actions.splice(index, 0, {
      id: `action-${crypto.randomUUID().slice(0, 8)}`,
      executor: { type: "command", name: "build" },
      inputs: {},
      timeout_seconds: 300,
    });
    select(index);
  }
  function move(index: number, direction: -1 | 1) {
    const next = index + direction;
    [definition.actions[index], definition.actions[next]] = [
      definition.actions[next]!,
      definition.actions[index]!,
    ];
    if (selection === index) selection = next;
    else if (selection === next) selection = index;
  }
  function remove(index: number) {
    definition.actions.splice(index, 1);
    if (selection === index) {
      selection = -1;
      invalid = false;
      onValid?.();
    } else if (selection > index) selection--;
  }
</script>

<div class="workflow-editor" data-inspector-open={inspector}>
  <div class="workflow-canvas">
    <div class="workflow-context">
      <StoreRepositoryPicker
        label="Metadata home"
        repositories={homes}
        bind:value={home}
        variant="ghost"
        disabled={homeDisabled}
      />
    </div>
    <div class="workflow-sequence" aria-label="Workflow sequence">
      <WorkflowStep
        label="Trigger"
        description={triggerLabel(definition.trigger)}
        selected={selection === -1}
        expanded={inspector && selection === -1}
        inspectorId={id}
        onSelect={() => select(-1)}
      />
      {#each definition.actions as item, index (item.id)}
        <Button
          variant="ghost"
          size="icon"
          class="workflow-connector"
          aria-label={`Insert action before ${item.id}`}
          title="Add action"
          disabled={disabled || definition.actions.length >= 32}
          onclick={() => add(index)}
        >
          <ArrowDown class="workflow-connector__arrow" aria-hidden="true" />
          <Plus class="workflow-connector__plus" aria-hidden="true" />
        </Button>
        <WorkflowStep
          label={item.id}
          description={executorLabel(item)}
          {index}
          selected={selection === index}
          expanded={inspector && selection === index}
          inspectorId={id}
          {disabled}
          canMoveUp={index > 0}
          canMoveDown={index < definition.actions.length - 1}
          canRemove
          onSelect={() => select(index)}
          onRemove={() => remove(index)}
          onMoveUp={() => move(index, -1)}
          onMoveDown={() => move(index, 1)}
        />
      {/each}
      <Button
        variant="ghost"
        size="icon"
        class="workflow-connector workflow-connector--add"
        aria-label="Add action"
        title="Add action"
        disabled={disabled || definition.actions.length >= 32}
        onclick={() => add(definition.actions.length)}
      >
        <ArrowDown class="workflow-connector__arrow" aria-hidden="true" />
        <Plus class="workflow-connector__plus" aria-hidden="true" />
      </Button>
    </div>
  </div>
  <WorkflowInspector
    {acquireRepository}
    {authors}
    hidden={!inspector}
    {id}
    bind:definition
    {selection}
    {repositories}
    {disabled}
    bind:invalid
    {onInvalid}
    {onValid}
    canRemove
    onRemove={() => remove(selection)}
    onClose={() => (inspector = false)}
  />
</div>
